use ::hidapi::{BusType, HidApi};
use hidapi::{DeviceInfo, HidDevice};

use crate::hidapi_gamepads::GamepadModel;

pub enum HidApiGamepadError {
    NoBTDevice,
    UnknownVendor,
    UnknownProduct,
    OpenFailed,
}

/// Checks for connected HID Devices Returns all supported gamepads, might be 0 <br>
/// If no gamepads are connected, displays this in terminal and exits program
pub fn get_hid_gamepad(api: &HidApi) -> Result<(HidDevice, GamepadModel), HidApiGamepadError> {
    // most likely only one gamepad will be connected at one time, so its fastest to assume an vec size of 1
    // Still, this function has to check all connected devices
    let mut bluetooth_devices: Vec<&DeviceInfo> = Vec::with_capacity(1);

    // go trough all devices and take bluetooth devices
    for device_info in api.device_list() {
        let bus_type: BusType = device_info.bus_type();

        println!("bus type {:?}", device_info.bus_type());
        println!("product {:?}", device_info.product_string());
        println!("release {:?}", device_info.release_number());
        println!("serial_number {:?}", device_info.serial_number());
        println!("usage {:?}", device_info.usage());
        println!("usage page {:?}", device_info.usage_page());

        println!("{:#?}", device_info);

        match bus_type {
            BusType::Bluetooth => bluetooth_devices.push(device_info),
            _ => continue,
        };
    }

    if bluetooth_devices.is_empty() {
        println!("No Devices connected via Bluetooth found");
        return Err(HidApiGamepadError::NoBTDevice);
    }

    for device_info in bluetooth_devices {
        let vid: u16 = device_info.vendor_id();
        let pid: u16 = device_info.product_id();

        match vid {
            0x054c => match pid {
                0x0ce6 => {
                    match api.open(vid, pid) {
                        Ok(hid_device) => return Ok((hid_device, GamepadModel::PS5)),
                        Err(err) => {
                            println!("OpenFailed: vendor {:?}, product {:?}, Error {:?}", vid, pid, err);
                            return Err(HidApiGamepadError::OpenFailed);
                        }
                    };
                }
                _ => {
                    println!("UnknownProduct: vendor {:?}, product {:?}", vid, pid);
                    return Err(HidApiGamepadError::UnknownProduct);
                }
            },
            _ => {
                println!("UnknownVendor: vendor {:?}, product {:?}", vid, pid);
                return Err(HidApiGamepadError::UnknownVendor);
            }
        };
    }

    return Err(HidApiGamepadError::NoBTDevice);
}
