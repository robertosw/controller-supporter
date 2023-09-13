#![allow(dead_code, unreachable_code, unused_imports)]

#[macro_use]
extern crate version;
// To allow using the version! macro

extern crate termion;

use ctrlc::set_handler;
use helper_fn::print_and_exit;
use hidapi::HidApi;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod bluetooth_fn;
mod helper_fn;
mod hidapi_fn;
mod hidapi_gamepad;
mod hidapi_read_ps5_usb;
mod usb_gadget;
mod usb_gamepads;

use crate::bluetooth_fn::*;
use crate::hidapi_fn::{get_hid_gamepad, process_input};
use crate::hidapi_gamepad::UniversalGamepad;
use crate::usb_gamepads::PS4_GAMEPAD;
use crate::usb_gamepads::PS5_GAMEPAD;

//  if working inside a docker container: (started with the docker-compose from project root)
//  - build and run (inside container)  `cargo run`
//
//  if working on native os as non root: (from /gamepad-bridge)
//  - build & run   `cargo build --release && sudo chown root:root target/release/gamepad-bridge && sudo chmod +s target/release/gamepad-bridge && /target/release/gamepad-bridge`

pub const HID_ARRAY_SIZE: usize = 75;

fn main() {
    println!("\nGamepad-Bridge started: v{:}", version!());
    println!("This program needs to be run as root user. Please set uuid accordingly.\n");

    PS5_GAMEPAD.configure_device();
    // PS4_GAMEPAD.configure_device();
    // _generate_output();

    exit(0);
    // GENERIC_KEYBOARD.configure_device();

    _read_gamepad_input();
}

fn _generate_output() {
    let mut hidg0 = match File::options()
        .write(true)
        .truncate(false)
        .open("/dev/hidg0")
    {
        Ok(file) => file,
        Err(_) => print_and_exit("Could not open file bmAttributes", 13),
    };

    // This counts one byte at a time from 0 to 255 and back to 0

    const REPORT_LENGTH: usize = 64;
    const DURATION_MS: u64 = 4000;
    let mut output_buf: [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];

    let mut i: usize = 0;
    while i < REPORT_LENGTH {
        let mut up: bool = true;
        while (output_buf[i] < 255) && up {
            output_buf[i] += 1;
            println!("{:?}", output_buf);
            match hidg0.write_all(&output_buf) {
                Ok(_) => (),
                Err(err) => {
                    println!("write to hidg0 failed: {:?}", err);
                    exit(1);
                },
            }
            thread::sleep(Duration::from_millis((DURATION_MS / 2) / 255));
        }
        up = false;
        while (output_buf[i] > 0) && !up {
            println!("{:?}", output_buf);
            output_buf[i] -= 1;
            match hidg0.write_all(&output_buf) {
                Ok(_) => (),
                Err(err) => {
                    println!("write to hidg0 failed: {:?}", err);
                    exit(1);
                },
            }
            thread::sleep(Duration::from_millis((DURATION_MS / 2) / 255));
        }

        i += 1;
    }
}

fn _read_gamepad_input() -> ! {
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            println!("Error getting HidApi access: {:?}", err);
            exit(2);
        }
    };

    let (device, model) = match get_hid_gamepad(&api) {
        Ok(val) => val,
        Err(_) => exit(1),
    };

    // --- Read from device --- //

    // if false, calls to read may return nothing, but also dont block
    match device.set_blocking_mode(false) {
        Ok(_) => (),
        Err(err) => panic!("HidError: {:?}", err),
    };
    let mut input_buf = [0 as u8; HID_ARRAY_SIZE];
    let mut output_gamepad: UniversalGamepad = UniversalGamepad::nothing_pressed();

    print!("{}", termion::clear::All);

    loop {
        // setting -1 as timeout means waiting for the next input event, in this mode valid_bytes_count == HID_ARRAY_SIZE
        // setting 0ms as timeout, probably means sometimes the previous input event is taken, but the execution time of this whole block is 100x faster!
        // also: reading in blocking mode might be problematic if the gamepad is disconnected => infinite wait
        let _valid_bytes_count: usize = match device.read_timeout(&mut input_buf[..], 0) {
            Ok(value) => {
                if value != HID_ARRAY_SIZE {
                    continue;
                } else {
                    process_input(input_buf, &model, &mut output_gamepad);
                    value
                }
            }
            Err(_) => continue,
        };

        thread::sleep(Duration::from_micros(1500)); // <= 1500 is fine for now delay
    }
}

// TODO Check if hidg0 device exists

// TODO Retry hidg crate

// TODO Write to hidg0 device manually (example)
// sudo su
// echo -ne "\0\0\x4\0\0\0\0\0" > /dev/hidg0 #press the A-button
// echo -ne "\0\0\0\0\0\0\0\0" > /dev/hidg0 #release all keys

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

    bt_power_on();
    let (shared_mem_scan_output, thread_handle) = bt_scan_on_threaded();

    // find new controllers
    // loop while ctrlc has not been pressed (.load == true)
    while ctrlc.load(Ordering::SeqCst) {
        handle_bt_scan_output(&shared_mem_scan_output);

        thread::sleep(Duration::from_millis(500));
    }

    thread_handle.join().unwrap();
}
