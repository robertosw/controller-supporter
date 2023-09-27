#![allow(dead_code)]

#[macro_use]
extern crate version;
// To allow using the version! macro

extern crate termion;

use ctrlc::set_handler;
use flume::bounded;
use flume::unbounded;
use flume::Receiver;
use flume::Sender;
use hidapi::HidApi;
use std::env;
use std::process::exit;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
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
use crate::usb_gamepad::OutputGamepad;
use crate::usb_gamepad_ps4::DUALSHOCK;
use crate::usb_gamepad_ps5::DUALSENSE;

//  if working inside a docker container: (started with the docker-compose from project root)
//  - build and run (inside container)  `cargo run`
//
//  if working on native os as non root: (from /gamepad-bridge)
//  - build & run   `cargo build --release && sudo chown root:root target/release/gamepad-bridge && sudo chmod +s target/release/gamepad-bridge && /target/release/gamepad-bridge`

// for benchmarking in tests use: cargo test -- --show-output

fn main() {
    println!("\nGamepad-Bridge started: v{:}", version!());
    println!("This program needs to be run as root user. Please set uuid accordingly.\n");

    // ----- Enable Gadget
    // If this is done at a later point, the host might run into errors when trying to classify this device and turn it off
    let output_gamepad: &OutputGamepad = OutputGamepad::from_cmdline_args();
    output_gamepad.gadget.configure_device();
    println!("Gadget enabled");

    // ----- Create all channels
    // These are used to tell the reading and writing threads to finish (they are normally infinite loops)
    let (sender_ctrlc, recv_ctrlc) = mpsc::channel();
    let (sender_exit_request, recv_exit_request): (Sender<()>, Receiver<()>) = bounded(1);
    let (sender_gamepad, recv_gamepad): (Sender<UniversalGamepad>, Receiver<UniversalGamepad>) = unbounded();

    // ----- Setup CTRL+C handler
    ctrlc::set_handler(move || sender_ctrlc.send(()).expect("Could not send signal on channel.")).expect("Error setting Ctrl-C handler");

    // ----- BT connection here
    // TODO

    // ----- What gamepad is connected?
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => print_error_and_exit!("Error getting HidApi access", err, 2),
    };

    let (device, input_gamepad): (hidapi::HidDevice, &OutputGamepad) = match hidapi_fn::get_hid_gamepad(&api) {
        Ok((device, model)) => match model {
            hidapi_fn::SupportedInputGamepads::Ps5DualSense => (device, &DUALSENSE),
            hidapi_fn::SupportedInputGamepads::PS4DualShock => (device, &DUALSHOCK),
        },
        Err(err) => print_error_and_exit!("Error accessing connected hid gamepad", err, 1),
    };

    println!("Gamepad connected");

    // ----- Reading input of BT gamepad
    let thread_handle_input = thread::Builder::new()
        .name("input".to_string())
        .spawn(move || hidapi_fn::read_bt_gamepad_input(device, input_gamepad, sender_gamepad, recv_exit_request))
        .expect("creating input thread failed");
    println!("Input thread running");

    // TODO Maybe remove this later, but currently the output-writing step is reached so fast that /dev/hidg0 is not yet ready.
    // This just prevents some of the "Cannot send after transport endpoint shutdown" errors because of this ^
    thread::sleep(Duration::from_secs(1));

    // ----- Write Output to gadget
    let thread_handle_output = thread::Builder::new()
        .name("output".to_string())
        .spawn(move || output_gamepad.write_to_gadget_continously(recv_gamepad))
        .expect("creating output thread failed");
    println!("Output thread running");
    println!("");

    // ----- Clean up (if Ctrl + C is pressed)

    // This is blocking
    match recv_ctrlc.recv() {
        Ok(_) => println!(""),
        Err(e) => print_error_and_exit!("Receiving from CTRL C channel failed:", e, 1),
    }

    println!("Waiting for input and output threads to finish");
    sender_exit_request.send(()).expect("sending to input thread failed");
    thread_handle_input.join().unwrap();
    thread_handle_output.join().unwrap();

    // clean_up_device() removes hidg0 file, so this has to run after write output thread is closed
    println!("Disabling gadget");
    output_gamepad.gadget.clean_up_device();

    println!("Everything is cleaned up :)");
}

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
