#![allow(dead_code)]

#[macro_use]
extern crate version;

use ctrlc::set_handler;
use hidapi::HidApi;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod bluetooth_fn;
mod hidapi_fn;
mod hidapi_read_ps5_usb;
mod hidapi_structs;
mod usb_descr;
mod usb_gadget;

use crate::bluetooth_fn::*;
use crate::hidapi_fn::*;
use crate::usb_gadget::*;

// lsusb   udevadm monitor       minicom

fn main() {
    println!("\nGamepad-Bridge started: v{:}\n", version!());

    configure_as_gadget("raspi", "abcdef12345", "Generic Manufacturer", "My Product", "My Config", 0, 0, 64);

    println!("printing all rusb devices");
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }

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

    exit(0);

    // TODO Check if hidg0 device exists

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
