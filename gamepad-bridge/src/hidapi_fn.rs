use ::hidapi::{BusType, HidApi};
use hidapi::{DeviceInfo, HidDevice};

use crate::{hidapi_gamepad::*, HID_ARRAY_SIZE};

pub enum HidApiGamepadError {
    NoBTDevice,
    NoSupportedDevice,
    OpenFailed,
}

pub enum GamepadModel {
    PS5,
    PS4,
}

/// Checks for connected HID Devices, tries to find a supported one
///
/// Returns in `HidApiGamepadError` if:
/// - No bluetooth hid device is connected
/// - None of the connected devices are from a supported vendor
/// - None of known vendor devices are known products
/// - Opening a device failed
pub fn get_hid_gamepad(api: &HidApi) -> Result<(HidDevice, GamepadModel), HidApiGamepadError> {
    let bluetooth_devices: Vec<&DeviceInfo> = match get_bluetooth_hid_devices(api) {
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
fn get_bluetooth_hid_devices(api: &HidApi) -> Result<Vec<&DeviceInfo>, ()> {
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
            BusType::Usb => bluetooth_devices.push(device_info), // TODO REMOVE
            _ => continue,
        };
    }

    if bluetooth_devices.is_empty() {
        println!("No Devices connected via Bluetooth found");
        return Err(());
    }

    return Ok(bluetooth_devices);
}

pub fn process_input(input: [u8; HID_ARRAY_SIZE], model: &GamepadModel, output: &mut UniversalGamepad) {
    match model {
        GamepadModel::PS5 => process_input_ps5(output, input),
        GamepadModel::PS4 => {}
    }

    println!("Lx: {:?}", output.sticks.left.x);
    println!("Ly: {:?}", output.sticks.left.y);
    println!("L : {:?}", output.sticks.left.pressed);

    println!("Rx: {:?}", output.sticks.right.x);
    println!("Ry: {:?}", output.sticks.right.y);
    println!("R : {:?}", output.sticks.right.pressed);

    println!("Tl: {:?}", output.triggers.left);
    println!("Tr: {:?}", output.triggers.right);
    println!("Bl: {:?}", output.bumpers.left);
    println!("Br: {:?}", output.bumpers.right);

    println!("X: {:?}", output.buttons.lower);
    println!("O: {:?}", output.buttons.right);
    println!("□: {:?}", output.buttons.left);
    println!("∆: {:?}", output.buttons.upper);

    println!("↑: {:?}", output.dpad.up);
    println!("→: {:?}", output.dpad.right);
    println!("↓: {:?}", output.dpad.down);
    println!("←: {:?}", output.dpad.left);
}

fn process_input_ps5(output: &mut UniversalGamepad, input: [u8; HID_ARRAY_SIZE]) {
    let dpad = 0b00001111 & input[8];

    output.sticks = Sticks {
        left: Stick {
            x: input[1],
            y: input[2],
            pressed: match input[9] {
                64 => true,
                _ => false,
            },
        },
        right: Stick {
            x: input[3],
            y: input[4],
            pressed: match input[9] {
                128 => true,
                _ => false,
            },
        },
    };
    output.triggers = Triggers {
        left: input[5],
        right: input[6],
    };
    output.bumpers = Bumpers {
        left: match input[9] {
            1 => true,
            _ => false,
        },
        right: match input[9] {
            2 => true,
            _ => false,
        },
    };
    output.buttons = MainButtons {
        upper: (input[8] & 0b10000000 != 0),
        right: (input[8] & 0b01000000 != 0),
        lower: (input[8] & 0b00100000 != 0),
        left: (input[8] & 0b00010000 != 0),
    };
    output.dpad = DPad {
        right: (dpad == 1 || dpad == 2 || dpad == 3),
        down: (dpad == 3 || dpad == 4 || dpad == 5),
        left: (dpad == 5 || dpad == 6 || dpad == 7),
        up: (dpad == 0 || dpad == 1 || dpad == 7),
    };
    output.specials = SpecialButtons {
        touchpad: match input[11] {
            2 => true,
            _ => false,
        },
        right: match input[11] {
            32 => true,
            _ => false,
        },
        left: match input[10] {
            16 => true,
            _ => false,
        },
        logo: match input[11] {
            1 => true,
            _ => false,
        },
    };
}
