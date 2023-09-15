use ::hidapi::{BusType, HidApi};
use hidapi::{DeviceInfo, HidDevice};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{universal_gamepad::UniversalGamepad, usb_gamepad::Gamepad};

#[derive(Debug)]
pub enum HidApiGamepadError {
    NoBTDevice,
    NoSupportedDevice,
    OpenFailed,
}

pub enum GamepadModel {
    PS5,
    PS4,
}

// TODO Use for first manual debugging / interpreting of new gamepads
pub fn read_unknown_usb_input() {
    // _process_input_unknown()
}

fn _process_input_unknown(input: Vec<u8>) {
    print!("{}", termion::cursor::Goto(1, 1));

    // adjust which bytes should be visible. For PS Gamepads the first two bytes are just counters
    let mut i: usize = 0;

    for byte in input[i..].iter() {
        print!("{}|{:03}\t", i, byte);
        i += 1;
    }
}

/// Checks for connected HID Devices, tries to find a supported one
///
/// Returns in `HidApiGamepadError` if:
/// - No bluetooth hid device is connected
/// - None of the connected devices are from a supported vendor
/// - None of known vendor devices are known products
/// - Opening a device failed
pub fn get_hid_gamepad(api: &HidApi) -> Result<(HidDevice, GamepadModel), HidApiGamepadError> {
    let bluetooth_devices: Vec<&DeviceInfo> = match _get_bluetooth_hid_devices(api) {
        Ok(vec) => vec,
        Err(_) => return Err(HidApiGamepadError::NoBTDevice),
    };

    // most likely only one gamepad will be connected at one time, so its fastest to assume an vec size of 1
    let mut error_info: Vec<(u16, u16, Option<&str>)> = Vec::with_capacity(1);

    for device_info in bluetooth_devices {
        let vid: u16 = device_info.vendor_id();
        let pid: u16 = device_info.product_id();

        match (vid, pid) {
            // PS5 Gamepad
            (0x054c, 0x0ce6) => {
                match api.open(vid, pid) {
                    Ok(hid_device) => return Ok((hid_device, GamepadModel::PS5)),
                    Err(err) => {
                        println!("OpenFailed: vendor {:?}, product {:?}, Error {:?}", vid, pid, err);
                        return Err(HidApiGamepadError::OpenFailed);
                    }
                };
            }
            _ => {
                error_info.push((vid, pid, device_info.product_string()));
                continue;
            }
        };
    }

    println!("All of these devices are connected but not supported:");
    for device in error_info {
        println!("vendor {:?}, product {:?} {:?}", device.0, device.1, device.2);
    }

    return Err(HidApiGamepadError::NoSupportedDevice);
}

/// - If there are any hid devices connected via bluetooth, these will be returned.
/// - If not, returns Error
fn _get_bluetooth_hid_devices(api: &HidApi) -> Result<Vec<&DeviceInfo>, ()> {
    // most likely only one gamepad will be connected at one time, so its fastest to assume an vec size of 1
    // Still, this function has to check all connected devices
    let mut bluetooth_devices: Vec<&DeviceInfo> = Vec::with_capacity(1);

    for device_info in api.device_list() {
        let bus_type: BusType = device_info.bus_type();

        // println!("bus type {:?}", device_info.bus_type());
        // println!("product {:?}", device_info.product_string());
        // println!("release {:?}", device_info.release_number());
        // println!("serial_number {:?}", device_info.serial_number());
        // println!("usage {:?}", device_info.usage());
        // println!("usage page {:?}", device_info.usage_page());

        // println!("{:#?}", device_info);

        match bus_type {
            BusType::Bluetooth => bluetooth_devices.push(device_info),
            _ => continue,
        };
    }

    if bluetooth_devices.is_empty() {
        println!("No Devices connected via Bluetooth found");
        return Err(());
    }

    return Ok(bluetooth_devices);
}

pub fn read_bt_gamepad_input(device: HidDevice, input_gamepad: &Gamepad, universal_gamepad: Arc<Mutex<UniversalGamepad>>) -> ! {
    // if set to false, calls to read may return nothing, but also dont block
    match device.set_blocking_mode(false) {
        Ok(_) => (),
        Err(err) => panic!("HidError: {:?}", err),
    };

    // This is from PS5 Dual Sense controller, read out with terminal tool hid-recorder (hid-tools)
    // TODO Read this out dynamically
    let min_report_size: usize = 16;

    let mut buf: [u8; 100] = [0 as u8; 100];

    loop {
        // setting -1 as timeout means waiting for the next input event, in this mode valid_bytes_count == HID_ARRAY_SIZE
        // setting 0ms as timeout, probably means sometimes the previous input event is taken, but the execution time of this whole block is 100x faster!
        // also: reading in blocking mode might be problematic if the gamepad is disconnected => infinite wait
        match device.read_timeout(&mut buf[..], 0) {
            Ok(value) => match value.cmp(&min_report_size) {
                std::cmp::Ordering::Greater => {
                    let mut gamepad_locked = universal_gamepad.lock().expect("Locking Arc<Mutex<UniversalGamepad>> failed!");
                    *gamepad_locked = input_gamepad.bt_input_to_universal_gamepad(&buf.to_vec());
                }
                _ => continue,
            },
            Err(e) => {
                println!("read_timeout error: {e}");
                continue;
            }
        };

        // TODO can this be changed to wait for inputs?
        thread::sleep(Duration::from_micros(1500));
    }
}
