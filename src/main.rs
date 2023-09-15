#![allow(dead_code)]

#[macro_use]
extern crate version;
// To allow using the version! macro

extern crate termion;

use ctrlc::set_handler;
use hidapi::HidApi;
use std::env;
use std::process::exit;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use usb_gadget::UsbGadgetDescriptor;

mod bluetooth_fn;
mod helper_fn;
mod hidapi_fn;
mod universal_gamepad;
mod usb_gadget;
mod usb_gamepad;
mod usb_gamepad_keyboard;
mod usb_gamepad_ps4;
mod usb_gamepad_ps5;

use crate::bluetooth_fn::*;
use crate::universal_gamepad::UniversalGamepad;
use crate::usb_gamepad::Gamepad;
use crate::usb_gamepad_ps4::DUALSHOCK;
use crate::usb_gamepad_ps5::DUALSENSE;

//  if working inside a docker container: (started with the docker-compose from project root)
//  - build and run (inside container)  `cargo run`
//
//  if working on native os as non root: (from /gamepad-bridge)
//  - build & run   `cargo build --release && sudo chown root:root target/release/gamepad-bridge && sudo chmod +s target/release/gamepad-bridge && /target/release/gamepad-bridge`

fn main() {
    println!("\nGamepad-Bridge started: v{:}", version!());
    println!("This program needs to be run as root user. Please set uuid accordingly.\n");

    // ----- Enable Gadget
    // If this is done later, the host might run into errors when trying to classify this device and turn it off
    // TODO output_gamepad should be expected from a command argument or set to a default if not given

    single_thread_interval(Duration::from_micros(500));

    exit(0);

    let output_gamepad: &Gamepad = &DUALSENSE;
    // output_gamepad.gadget.configure_device();

    // ----- BT connection here

    // ----- What gamepad is connected?
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => print_error_and_exit!("Error getting HidApi access", err, 2),
    };

    let (device, input_gamepad): (hidapi::HidDevice, &Gamepad) = match hidapi_fn::get_hid_gamepad(&api) {
        Ok((device, model)) => match model {
            hidapi_fn::GamepadModel::PS5 => (device, &DUALSENSE),
            hidapi_fn::GamepadModel::PS4 => (device, &DUALSHOCK),
        },
        Err(err) => print_error_and_exit!("Error accessing connected hid gamepad", err, 1),
    };

    println!("Gamepad connected");

    // ----- Reading input of BT gamepad
    let universal_gamepad: Arc<Mutex<UniversalGamepad>> = Arc::new(Mutex::new(UniversalGamepad::nothing_pressed()));
    let gamepad_clone = universal_gamepad.clone(); // Cloning has to be done outside of the closure
    let thread_handle_read_input = thread::spawn(move || hidapi_fn::read_bt_gamepad_input(device, input_gamepad, gamepad_clone));

    // Maybe remove this later, but currently this the output-writing step is reached so fast that /dev/hidg0 is not yet ready.
    // This just prevents some of the "Cannot send after transport endpoint shutdown" errors because of this ^
    thread::sleep(Duration::from_secs(1));

    // ----- Write Output to gadget
    let gamepad_clone = universal_gamepad.clone();
    let thread_handle_write_output = thread::spawn(move || output_gamepad.write_to_gadget_continously(gamepad_clone));

    // ----- Clean up (if Ctrl + C is pressed)
    // TODO move CTRL + C handling from BT to here
    // TODO undo gadget config completely, so that this program can be rerun without errors from configure_device()
    thread_handle_read_input.join().unwrap();
    thread_handle_write_output.join().unwrap();
}

fn single_thread_interval(interval: Duration) {
    // its safe to use u128 for nanoseconds
    // 2^64 ns are ~580 years
    // so 2^128 are 580² years

    let start: Instant = Instant::now();
    let interval_ns = interval.as_nanos();
    let mut interval_counts_before: u128 = 0;

    let mut diffs: Vec<Duration> = Vec::new();

    const ROUNDS: u128 = 1000;

    while interval_counts_before < ROUNDS {
        let elapsed_ns: u128 = start.elapsed().as_nanos();
        let interval_counts_now: u128 = elapsed_ns / interval_ns;

        if interval_counts_now > interval_counts_before {
            let now = Instant::now();
            let expected = start + (interval * interval_counts_now as u32);

            diffs.push(now - expected);


            
            interval_counts_before = interval_counts_now;
        }
    }


    //  benchmark results

    let mut avg_ns = 0;
    let mut avg_perc: f64 = 0.0;

    for difference in diffs {
        avg_ns += difference.as_nanos();
        let error_percent: f64 = (difference.as_nanos() as f64 / interval.as_nanos() as f64) * 100.0;
        avg_perc += error_percent;
    }

    println!("Avg ABS  {} ns", avg_ns / ROUNDS);
    println!("Avg PERC {:2.1?} %", avg_perc / ROUNDS as f64);
}

// Ideas for program flow
// 1. the whole procedure (BT finding, input read, output to usb) is being duplicated for each player right inside main. So 1-4 threads
//     Problem: two threads could use the same gamepad and think its their own.. so output duplication
// 2. the bluetooth scanning is one thread, seperate from main (output written in file or shared mem)
//     - output is interpreted inside main thread
//     after an active device is connected, only then is a thread spawned for this device
//     -> threads dont have to know from each others existence (maybe for usb output, but we'll see)

fn _bt_program_flow() {
    // Create a shared boolean flag to indicate if Ctrl+C was pressed
    let ctrlc = Arc::new(AtomicBool::new(true));
    let ctrlc_clone = ctrlc.clone();

    // Set the flag to false when Ctrl+C is pressed
    match set_handler(move || ctrlc_clone.store(false, Ordering::SeqCst)) {
        Ok(_) => (),
        Err(err) => {
            println!("Error setting Ctrl-C handler {:?}", err);
            exit(1);
        }
    };

    {
        let output_power_on = match Command::new("bluetoothctl").args(["power", "on"]).output() {
            Ok(out) => out,
            Err(err) => {
                println!("unwrapping the output failed: {:?}", err);
                exit(1);
            }
        };

        let stdout = String::from_utf8(output_power_on.stdout).ok();
        let stderr = String::from_utf8(output_power_on.stderr).ok();

        if !output_power_on.status.success() {
            println!("bluetoothctl power on failed:");
            println!("{:?}", stderr);
            exit(1);
        }

        println!("Stdout: {:?}", stdout);
        println!("Stderr: {:?}", stderr);
    };
    let (shared_mem_scan_output, thread_handle) = bt_scan_on_threaded();

    // find new controllers
    // loop while ctrlc has not been pressed (.load == true)
    while ctrlc.load(Ordering::SeqCst) {
        handle_bt_scan_output(&shared_mem_scan_output);

        thread::sleep(Duration::from_millis(500));
    }

    thread_handle.join().unwrap();
}
