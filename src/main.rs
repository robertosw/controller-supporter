#![allow(dead_code, unreachable_code)]

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
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
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
use crate::usb_gamepad::Gamepad;
use crate::usb_gamepad_ps4::DUALSHOCK;
use crate::usb_gamepad_ps5::DUALSENSE;

//  if working inside a docker container: (started with the docker-compose from project root)
//  - build and run (inside container)  `cargo run`
//
//  if working on native os as non root: (from /gamepad-bridge)
//  - build & run   `cargo build --release && sudo chown root:root target/release/gamepad-bridge && sudo chmod +s target/release/gamepad-bridge && /target/release/gamepad-bridge`

/* TODO
    The input and output thread are taking up nearly 100% cpu all the time
    It would be better if input thread would wait for bt input, and output thread would wait for new data from input thread
    
    T1 / T2 = thread 1 / 2

    - Main creates two channels, channel one for T1 -> T2 and channel two for Main -> T1
    - Usage Channel One:
        - T2 uses receiver.recv inside its loop to block until a new message is recieved
        - After recieving, the channel is cleared automatically
        - This way T2 only works for new messages (the interval is lost, but maybe thats not a problem)
            - This could still be solved by async-std and async_std::thread::sleep_until
    - Usage Channel Two:
        - Main sends nothing to T1, apart from the request to exit
        - T1 closes the channel to T2, and by doing to T2 knows that it has to exit as well
*/

fn main() {
    println!("\nGamepad-Bridge started: v{:}", version!());
    println!("This program needs to be run as root user. Please set uuid accordingly.\n");

    // ----- Enable Gadget
    // If this is done at a later point, the host might run into errors when trying to classify this device and turn it off
    // TODO output_gamepad should be expected from a command argument or set to a default if not given
    let output_gamepad: &Gamepad = &DUALSENSE;
    output_gamepad.gadget.configure_device();

    // ----- Create all channels
    // These are used to tell the reading and writing threads to finish (they are normally infinite loops)
    let (sender_ctrlc, recv_ctrlc) = channel();
    let (sender_write_output, recv_write_output): (Sender<bool>, Receiver<bool>) = channel();
    let (sender_read_input, recv_read_input): (Sender<bool>, Receiver<bool>) = channel();

    // ----- Setup CTRL+C handler
    ctrlc::set_handler(move || sender_ctrlc.send(()).expect("Could not send signal on channel.")).expect("Error setting Ctrl-C handler");

    // ----- BT connection here
    // TODO

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
    let thread_handle_read_input = thread::Builder::new()
        .name("input".to_string())
        .spawn(move || hidapi_fn::read_bt_gamepad_input(device, input_gamepad, gamepad_clone, recv_read_input))
        .expect("creating input thread failed");
    println!("Input thread created");

    // TODO Maybe remove this later, but currently the output-writing step is reached so fast that /dev/hidg0 is not yet ready.
    // This just prevents some of the "Cannot send after transport endpoint shutdown" errors because of this ^
    thread::sleep(Duration::from_secs(1));

    // ----- Write Output to gadget
    let gamepad_clone = universal_gamepad.clone();
    let thread_handle_write_output = thread::Builder::new()
        .name("output".to_string())
        .spawn(move || output_gamepad.write_to_gadget_intervalic(gamepad_clone, Duration::from_millis(1), 0.01, recv_write_output))
        .expect("creating output thread failed");
    println!("Output thread created");

    // ----- Clean up (if Ctrl + C is pressed)

    // This is blocking
    match recv_ctrlc.recv() {
        Ok(_) => println!(""),
        Err(e) => print_error_and_exit!("Receiving from CTRL C channel failed:", e, 1),
    }

    println!("Waiting for read input thread to finish");
    sender_read_input.send(true).expect("sending to read input thread failed");
    thread_handle_read_input.join().unwrap();

    println!("Waiting for write output thread to finish");
    sender_write_output.send(true).expect("sending to write output thread failed");
    thread_handle_write_output.join().unwrap();

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
