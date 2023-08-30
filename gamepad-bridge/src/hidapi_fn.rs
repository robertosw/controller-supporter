use ::hidapi::{BusType, HidApi};
use hidapi::DeviceInfo;
use std::process::exit;

use crate::hidapi_read_ps5_usb::*;

// TODO It might be best to be able to create some struct describing how the input should be read
// that way, once the program is sure which device is connected, the appropriate input intepretation can be applied

/// Returns all supported gamepads, might be 0 <br>
/// If no gamepads are connected, displays this in terminal and exits program
pub fn find_supported_gamepads(api: HidApi) -> Vec<DeviceInfo> {
    // since api.device_list() returns an iterator,
    // it would be nececarry to go trough the whole
    // iterator twice to get the length and than handle all devices.
    // This way, only once is nececarry
    let mut device_count: usize = 0;
    let mut gamepads: Vec<hidapi::DeviceInfo> = Vec::new();

    // Print out information about all connected devices
    for device_info in api.device_list() {
        device_count += 1;
        let vendor_id: u16 = device_info.vendor_id();
        let bus_type: BusType = device_info.bus_type();

        // Check if device is supported
        match bus_type {
            BusType::Usb => {
                if vendor_id != 1356 {
                    // Sony is 1356
                    continue;
                }
            }
            BusType::Bluetooth => {
                // TODO Rewrite function below to support BT and both gamepads
                println!("Bluetooth is not yet supported");
                println!("Device name {:?}", device_info.product_string());
                // continue;
            }
            _ => continue,
        };

        println!("bus type {:?}", device_info.bus_type());
        println!("interface nr {:?}", device_info.interface_number());
        println!("product {:?}", device_info.product_string());
        println!("release {:?}", device_info.release_number());
        println!("usage {:?}", device_info.usage());
        println!("usage page {:?}", device_info.usage_page());

        println!("{:#?}", device_info);

        gamepads.push(device_info.to_owned());
    }

    // No devices connected
    if device_count == 0 {
        println!("No devices detected by HidApi. Exiting program.");
        exit(1);
    }

    return gamepads;
}

pub fn open_device(device_info: DeviceInfo, api: HidApi) {
    let vid: u16 = 1356;
    let pid: u16 = device_info.product_id();

    let open_device = match api.open(vid, pid) {
        Ok(hid_device) => hid_device,
        Err(err) => panic!("Error: {:?}", err),
    };

    read_ps5_usb(&open_device);
}
