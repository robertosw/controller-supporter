use ::hidapi::{BusType, HidApi};
use hidapi::{DeviceInfo, HidDevice};

use crate::hidapi_gamepads::GamepadModel;

pub enum HidApiGamepadError {
    NoBTDevice,
    NoSupportedDevice,
    OpenFailed,
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
        let prod_str: Option<&str> = device_info.product_string();

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
                error_info.push((vid, pid, prod_str));
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
        return Err(());
    }

    return Ok(bluetooth_devices);
}
