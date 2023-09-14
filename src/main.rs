#![allow(dead_code, unreachable_code, unused_imports)]

#[macro_use]
extern crate version;
// To allow using the version! macro

extern crate termion;

use ctrlc::set_handler;
use helper_fn::print_and_exit;
use hidapi::HidApi;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;
use std::os::unix::prelude::OpenOptionsExt;
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
use crate::hidapi_fn::GamepadModel;
use crate::hidapi_fn::{get_hid_gamepad, process_input};
use crate::hidapi_gamepad::UniversalGamepad;
use crate::usb_gamepads::GENERIC_KEYBOARD;
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

    // GENERIC_KEYBOARD.configure_device();
    // _generate_output_keyboard();

    PS5_GAMEPAD.configure_device();
    _generate_output_ps5();

    // PS4_GAMEPAD.configure_device();

    exit(0);

    // _read_gamepad_input();
}

fn _generate_output_keyboard() {
    // Currently simulating keyboard

    const REPORT_LENGTH: usize = 8; // see usb_gamepads.rs or Gamepad Info Collection.md
    const DURATION_MS: u64 = 4000;

    loop {
        let mut hidg0 = match File::options().write(true).append(false).open("/dev/hidg0") {
            Ok(file) => file,
            Err(err) => {
                println!("Could not open file hidg0 {err}");
                exit(1);
            }
        };

        let out: [u8; REPORT_LENGTH] = [0x11, 0x22, 0x33, 0x44, 0x55, 0xFF, 0xAA, 0x00];

        match hidg0.write_all(&out) {
            // Ok(bytes) => print!("{bytes}b out"),
            Ok(_) => (),
            Err(err) => {
                println!("write to hidg0 failed: {:?}", err);
            }
        }

        // TODO achieve a real timed interval
        thread::sleep(Duration::from_millis(150));

        let out: [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];

        match hidg0.write_all(&out) {
            // Ok(bytes) => print!("{bytes}b out"),
            Ok(_) => (),
            Err(err) => {
                println!("write to hidg0 failed: {:?}", err);
            }
        }

        thread::sleep(Duration::from_millis(150));
    }
}

fn _generate_output_ps5() {
    const O_NONBLOCK: i32 = 2048;

    // Currently simulating PS5 Dual Sense

    const REPORT_LENGTH: usize = PS5_GAMEPAD.b_max_packet_size0 as usize; // see usb_gamepads.rs or Gamepad Info Collection.md
    const DURATION_MS: u64 = 4000;

    let mut dummy: UniversalGamepad = UniversalGamepad::nothing_pressed();
    let mut counter: u8 = 0;
    let mut seconds: u8 = 0;

    let mut oscillate = 0;
    let mut up: bool = true;

    println!("sleeping 10s");
    thread::sleep(Duration::from_secs(10));
    println!("lets go");

    loop {
        let mut hidg0 = match File::options().write(true).append(false).open("/dev/hidg0") {
            Ok(file) => file,
            Err(_) => print_and_exit("Could not open file hidg0", 13),
        };

        counter += 1;

        if counter == 255 {
            // This is roughly a second for 4ms interval
            // 4ms * 250 would be 1s, but maybe the real value also isnt a second, but counts the overflows?
            seconds += 1
        }

        let mut rng = rand::thread_rng();
        let logo_touchpad_byte: u8 = rng.gen_range(0..=2);

        // This counts one byte at a time from 0 to 255 and back to 0

        if oscillate == 255 {
            up = false;
        }
        while (oscillate < 255) && up {
            oscillate += 1;
        }
        while (oscillate > 0) && !up {
            oscillate -= 1;
        }

        dummy.sticks.left.x = oscillate;
        dummy.sticks.left.y = oscillate;
        dummy.sticks.right.x = oscillate;
        dummy.sticks.right.y = oscillate;
        dummy.triggers.left = oscillate;
        dummy.triggers.right = oscillate;

        let out: [u8; REPORT_LENGTH] = [
            0x01,
            dummy.sticks.left.x,
            dummy.sticks.left.y,
            dummy.sticks.right.x,
            dummy.sticks.right.y,
            dummy.triggers.left,
            dummy.triggers.right,
            counter,
            oscillate, // Buttons and DPad
            oscillate, // Special Buttons, Bumpers, Triggers and Sticks (only WHAT is pressed, for triggers not value)
            0,         // Logo / Touchpad
            0,         // always 0
            counter,   //
            seconds,   //
            0xee,      // might be charging state (in %) (unlikely, changes drastically after reconnect)
            0xad,      // ??
            0x00,      // gyroskop here (seems to be relative, not absolute)
            0x00,      // gyroskop here (seems to be relative, not absolute)
            0xff,      // gyroskop here (seems to be relative, not absolute)
            0xff,      // gyroskop here (seems to be relative, not absolute)
            0x02,      // gyroskop here (seems to be relative, not absolute)
            0x00,      // gyroskop here (seems to be relative, not absolute)
            0x06,      // gyroskop here (seems to be relative, not absolute)
            0x00,      // gyroskop here (seems to be relative, not absolute)
            0x81,      // gyroskop here (seems to be relative, not absolute)
            0x1f,      // gyroskop here (seems to be relative, not absolute)
            0x07,      // gyroskop here (seems to be relative, not absolute)
            0x06,      // gyroskop here (seems to be relative, not absolute)
            0x46,      // gyroskop here (seems to be relative, not absolute)
            0x66,      // gyroskop here (seems to be relative, not absolute)
            counter,   // this is a really slow counter (goes up every ~10s)
            0x00,      // ??
            0x14,      // ??
            0x80,      // ??
            0x00,      // ??
            0x00,      // ??
            0x00,      // ??
            0x80,      // ??
            0x00,      // ??
            0x00,      // ??
            0x00,      // ??
            0x00,      // ??
            0x09,      // ??
            0x09,      // ??
            0x00,      // ??
            0x00,      // ??
            0x00,      // ??
            0x00,      // ??
            0x00,      // ??
            0xe3,      // random?
            0x79,      // random?
            0xab,      // random?
            0x00,      // slow counter
            0x17,      // constant?
            0x08,      // constant?
            0x00,      // constant?
            0x5b,      // random?
            0x7f,      // random?
            0xef,      // random?
            0x9c,      // random?
            0xac,      // random?
            0x03,      // random?
            0x92,      // random?
            0x30,      // random?
        ];

        match hidg0.write_all(&out) {
            Ok(_) => (),
            Err(err) => {
                println!("write to hidg0 failed: {:?}", err);
            }
        }

        // TODO achieve a real timed interval
        thread::sleep(Duration::from_millis(4));
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
