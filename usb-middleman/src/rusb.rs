use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, DeviceList, UsbContext};

fn _find_supported_gamepads() -> Result<Vec<Device<Context>>, String> {
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
            println!("\n{:?}\n", descriptor);
            found_gpads.push(device);
        }
    }

    return Ok(found_gpads);
}

pub fn rusb() {
    let found_gpads: Vec<Device<Context>>;

    match _find_supported_gamepads() {
        Ok(gpads) => found_gpads = gpads,
        Err(err_descr) => panic!("{:?}", err_descr),
    };

    for gpad in found_gpads {
        let descriptor: DeviceDescriptor = match gpad.device_descriptor() {
            Ok(device_descr) => device_descr,
            Err(_) => continue,
        };
        let handle: DeviceHandle<Context> = match gpad.open() {
            Ok(device_handle) => device_handle,
            Err(_) => continue,
        };

        let _max_packet_size: u8 = descriptor.max_packet_size();

        let address = gpad.address();

        const REPORT_SIZE: usize = 8; // Change this to match the report size of your HID device

        let mut buf = [0; REPORT_SIZE];
        let timeout = std::time::Duration::from_secs(1); // Set the timeout for reading

        // FIXME Input Parameters of this function are incorrect, ends either in IO- or in InvalidParam-Error
        // Maybe this is simply a permission problem
        // Tested ^ this, maybe it is but doesnt look like it is the problem
        match handle.read_interrupt(0x84, &mut buf, timeout) {
            Ok(size) => {
                // Successfully read data from the HID device
                println!("Read {} bytes: {:?}", size, &buf[..size]);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                // Other error occurred while reading from the HID device
                // Handle the error
            }
        }
    }
}
