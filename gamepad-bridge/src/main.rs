#![allow(unused_imports, dead_code)]

use hidapi::HidApi;
use std::process::exit;

mod hidapi_fn;
mod hidapi_read_ps5_usb;
mod hidapi_structs;
mod bluetooth;

use crate::hidapi_fn::*;
use crate::bluetooth::*;

fn main() {
    println!("\n Gamepad-Bridge started");

    bluetooth();

    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            println!("Error getting HidApi access: {:?}", err);
            exit(2);
        }
    };

    let gamepads: Vec<hidapi::DeviceInfo> = find_supported_gamepads(api);
}
