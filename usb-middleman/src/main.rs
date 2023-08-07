use rusb::{Context, DeviceDescriptor, DeviceHandle, DeviceList, UsbContext};

fn find_supported_gamepads() -> Result<Vec<DeviceDescriptor>, String> {
    let devices: DeviceList<Context>;
    let mut found_gpads = Vec::new();

    match Context::new() {
        Ok(ctx) => {
            match ctx.devices() {
                Ok(device_list) => devices = device_list,
                Err(_) => return Err(String::from("device-list could not be accessed")),
            };
        }
        Err(_) => return Err(String::from("USB Context could not be created")),
    };

    // Iterate through all connected devices
    for device in devices.iter() {
        // get descriptor, handle and manufacturer
        let descriptor: DeviceDescriptor = match device.device_descriptor() {
            Ok(device_descr) => device_descr,
            Err(_) => continue,
        };
        let handle: DeviceHandle<Context> = match device.open() {
            Ok(device_handle) => device_handle,
            Err(_) => continue,
        };
        let manufacturer: String = match handle.read_manufacturer_string_ascii(&descriptor) {
            Ok(string) => string,
            Err(_) => String::from(""),
        };

        // Check if the device is a known Gamepad
        // Sonys PS4 and PS5 Gamepad are VendorID 1356
        if (manufacturer.contains("Sony")) || descriptor.vendor_id() == 1356 {
            found_gpads.push(descriptor);
        }
    }

    return Ok(found_gpads);
}

fn main() {
    let found_gpads: Vec<DeviceDescriptor>;

    match find_supported_gamepads() {
        Ok(gpads) => found_gpads = gpads,
        Err(err_descr) => panic!("{:?}", err_descr),
    };
}
