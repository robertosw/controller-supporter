#![allow(unused_imports, dead_code)]

mod read_ps5_usb;
mod structs;
use std::process::exit;

use ::hidapi::{BusType, HidApi, HidDevice};

use crate::read_ps5_usb::*;

fn main() {
    let api = HidApi::new().unwrap();

    // Print out information about all connected devices
    for device in api.device_list() {
        let vendor_id: u16 = device.vendor_id();
        let bus_type: BusType = device.bus_type();

        // Check if device is supported
        match bus_type {
            BusType::Usb => {
                if vendor_id != 1356 {
                    // Sony is 1356
                    continue;
                }
            }
            BusType::Bluetooth => {
                println!("Bluetooth is not yet supported");
                println!("Device name {:?}", device.product_string());
                continue;
            }
            _ => continue,
        };

        println!("bus type {:?}", device.bus_type());
        println!("interface nr {:?}", device.interface_number());
        println!("product {:?}", device.product_string());
        println!("release {:?}", device.release_number());
        println!("usage {:?}", device.usage());
        println!("usage page {:?}", device.usage_page());

        println!("{:#?}", device);

        let vid: u16 = 1356;
        let pid: u16 = device.product_id();

        let open_device = match api.open(vid, pid) {
            Ok(hid_device) => hid_device,
            Err(err) => panic!("Error: {:?}", err),
        };

        read_ps5_usb(&open_device);
    }
}

// Terminal 1: .../gamepad-bridge/gamepad-bridge$                  clear && cargo build --release
// Terminal 2: .../gamepad-bridge/gamepad-bridge/target/release$   clear && sudo chown root gamepad-bridge && sudo chmod u+s gamepad-bridge && ./gamepad-bridge
