#![allow(dead_code, unreachable_code)]

#[macro_use]
extern crate version;
// To allow using the version! macro

use ctrlc::set_handler;
use hidapi::HidApi;
use std::process::exit;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod bluetooth_fn;
mod hidapi_fn;
mod hidapi_read_ps5_usb;
mod hidapi_structs;
mod usb_gadget;
mod usb_gamepads;
mod helper_fn;

use crate::bluetooth_fn::*;
use crate::hidapi_fn::*;
use crate::usb_gamepads::*;

// lsusb   udevadm monitor       minicom

fn main() {
    println!("\nGamepad-Bridge started: v{:}\n", version!());

    // TODO Ensure that this is always run as sudo! Exit if not

    // TODO Somehow the host doesnt want what has been setup by configfs, maybe try with shell script again because this worked already
    // Maybe my report_desc and my functions and configs dont match?
    // corresponding errors:
    // usb 1-2: device not accepting address 16, error -62
    // usb 1-2: device descriptor read/64, error -110
    // usb usb1-port2: unable to enumerate USB device

    // PS5_GAMEPAD.configure_device();
    GENERIC_KEYBOARD.configure_device();

    exit(0);

    println!("printing all hidadpi devices");
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            println!("Error getting HidApi access: {:?}", err);
            exit(2);
        }
    };

    for device in api.device_list() {
        print!("{:?}", device.bus_type());
        print!("{:?}", device.interface_number());
        print!("{:?}", device.manufacturer_string());
        print!("{:?}", device.manufacturer_string_raw());
        print!("{:?}", device.path());
        print!("{:?}", device.product_id());
        print!("{:?}", device.product_string());
        print!("{:?}", device.product_string_raw());
        print!("{:?}", device.release_number());
        print!("{:?}", device.serial_number());
        print!("{:?}", device.serial_number_raw());
        print!("{:?}", device.vendor_id());
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

fn hidapi_starter() {
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            println!("Error getting HidApi access: {:?}", err);
            exit(2);
        }
    };

    let _gamepads: Vec<hidapi::DeviceInfo> = find_supported_gamepads(api);
}
