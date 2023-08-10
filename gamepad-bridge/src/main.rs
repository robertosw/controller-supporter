#![allow(unused_imports, dead_code)]

use hidapi::HidApi;
use std::process::exit;

mod hidapi_fn;
mod hidapi_structs;
mod hidapi_read_ps5_usb;
use crate::hidapi_fn::find_supported_gamepads;

fn main() {
    println!("\n Gamepad-Bridge started");

    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            println!("Error getting HidApi access: {:?}", err);
            exit(2);
        }
    };

    let gamepads: Vec<hidapi::DeviceInfo> = find_supported_gamepads(api);
}
