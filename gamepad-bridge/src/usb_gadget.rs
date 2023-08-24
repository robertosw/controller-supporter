use std::{
    fs::File,
    io::Write,
    process::{exit, Command},
};

use crate::{run_cmd, usb_descr::UsbDeviceDescriptor};

// TODO Validate this from host side:
// echo 100 > file   appends a \n at the end of the file. writing with file.write_all does not do that,
// but this should be irrelevant because the driver will probably read the data

const BASE_DIR: &str = "/sys/kernel/config/usb_gadget";
const DEVICE_DIR: &str = "/sys/kernel/config/usb_gadget/raspi";
const ENG_STR_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/strings/0x409";
const CONFIGS_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/configs/c.1";
const FNC_HID_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/functions/hid.usb0";

/// Using linux' ConfigFS, create the given usb device
pub fn enable_gadget_mode(device: UsbDeviceDescriptor, serialnumber: &str, manufacturer: &str, product: &str) {
    // TODO ConfigurationStrings
    // After creating any directory inside /sys/kernel/config/usb_gadget the system creates some basic structure.
    // This structure does not cover all the possible field. Non-existent are:
    // bLength, bDescriptorType, iManufacturer, iProduct, iSerialNumber, bNumConfigurations
    // This could mean that some of the work is done by the driver.
    // This might mean that a driver user can just write some strings and the driver will count and index them

    match create_directories() {
        Ok(_) => (),
        Err(_) => {
            print!("Error while creating directories, stopping.");
            return;
        }
    };

    match write_device_descriptor(&device) {
        Ok(_) => (),
        Err(err) => {
            print!("{err}");
            return;
        }
    };

    // match write_strings(serialnumber, manufacturer, product) {
    //     Ok(_) => (),
    //     Err(_) => (),
    // };

    match bind_to_udc() {
        Ok(_) => (),
        Err(err) => {
            print!("{err}");
            return;
        }
    }
}

fn create_directories() -> Result<(), ()> {
    // mount the configfs filesystem in this directory
    // none means it is not mounted onto some physical device
    match run_cmd("/", "mount none /sys/kernel/config -t configfs") {
        Ok(_) => (),
        Err(_) => return Err(()),
    };
    match run_cmd("/sys/kernel/config/usb_gadget", "mkdir raspi") {
        Ok(_) => (),
        Err(_) => return Err(()),
    };

    // Strings
    match run_cmd("/sys/kernel/config/usb_gadget/raspi", "mkdir strings") {
        Ok(_) => (),
        Err(_) => return Err(()),
    };
    match run_cmd("/sys/kernel/config/usb_gadget/raspi/strings", "mkdir 0x409") {
        Ok(_) => (),
        Err(_) => return Err(()),
    };

    // Configuration
    // match run_cmd("/sys/kernel/config/usb_gadget/raspi", format!("mkdir configs")) {
    //     Ok(_) => (),
    //     Err(_) => return Err(()),
    // };
    // match run_cmd("/sys/kernel/config/usb_gadget/raspi/configs", format!("mkdir c.1")) {
    //     Ok(_) => (),
    //     Err(_) => return Err(()),
    // };
    // match run_cmd("/sys/kernel/config/usb_gadget/raspi/configs", format!("mkdir c.1")) {
    //     Ok(_) => (),
    //     Err(_) => return Err(()),
    // };

    return Ok(());

    // run_cmd(format!("sudo mkdir -p {DIR}/{name}/{CONFIG_DIR}/{STR_ENG}"));
    // run_cmd(format!("sudo mkdir -p {DIR}/{name}/{FN_HID}"));
}

/// Writes the data of `device` into the files `bcdDevice`, `bcdUSB`, `bDeviceClass`, `bDeviceSubClass`, `bDeviceProtocol`, `bMaxPacketSize0`, `idVendor`, `idProduct`
///
/// <br>
///
/// ### Why only these and not all?
/// After creating any directory inside `/sys/kernel/config/usb_gadget` the system creates some basic structure.
/// This structure does not cover all the possible field. Non-existent are:
/// `bLength`, `bDescriptorType`, `iManufacturer`, `iProduct`, `iSerialNumber`, `bNumConfigurations`
///
/// This implies that some of the work is done by the driver.
fn write_device_descriptor(device: &UsbDeviceDescriptor) -> Result<(), &str> {
    match File::create(&(DEVICE_DIR.to_string() + "/bcdDevice")) {
        Ok(mut file) => match file.write_all(&device.bcd_device.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/bcdUSB")) {
        Ok(mut file) => match file.write_all(&device.bcd_usb.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/bDeviceClass")) {
        Ok(mut file) => match file.write_all(&device.b_device_class.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/bDeviceSubClass")) {
        Ok(mut file) => match file.write_all(&device.b_device_sub_class.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/bDeviceProtocol")) {
        Ok(mut file) => match file.write_all(&device.b_device_protocol.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/bMaxPacketSize0")) {
        Ok(mut file) => match file.write_all(&device.b_max_packet_size0.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/idVendor")) {
        Ok(mut file) => match file.write_all(&device.id_vendor.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::create(&(DEVICE_DIR.to_string() + "/idProduct")) {
        Ok(mut file) => match file.write_all(&device.id_product.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    return Ok(());
}
///
fn write_strings(_serialnumber: &str, manufacturer: &str, product: &str) -> Result<(), ()> {
    // TODO dynamically write or dont write (ORDER IS IMPORTANT!)
    // match run_cmd(ENG_STR_DIR, format!("echo {} > serialnumber", serialnumber)) {
    //     Ok(_) => (),
    //     Err(_) => return Err(()),
    // };
    // match run_cmd(ENG_STR_DIR, format!("echo -n {} > manufacturer", manufacturer)) {
    //     Ok(_) => (),
    //     Err(_) => return Err(()),
    // };
    // match run_cmd(ENG_STR_DIR, format!("echo -n {} > product", product)) {
    //     Ok(_) => (),
    //     Err(_) => return Err(()),
    // };

    return Ok(());
}

fn bind_to_udc() -> Result<(), &'static str> {
    let output = match Command::new("ls").arg("/sys/class/udc").output() {
        Ok(output) => output,
        Err(error) => {
            println!("unwrapping the output failed: {:?}", error);
            return Err("");
        }
    };

    let udc_name = String::from_utf8(output.stdout).ok().unwrap();

    if udc_name.trim().is_empty() {
        exit_with_udc_not_configured_msg();
    }

    // if there are multiple udcs (shouldn't be the case on the zero 2), take the first
    let (first_udc, _) = match udc_name.split_once(char::is_whitespace) {
        Some((first_udc, remainder)) => (first_udc, remainder),
        None => {
            exit_with_udc_not_configured_msg();
            return Err("");
        }
    };

    // run_cmd(format!("sudo sh -c 'echo {first_udc} > {DIR}/{name}/UDC' "));
    let cmd_string = String::from("sudo sh -c ") + "echo " + first_udc + " > UDC";

    match run_cmd(DEVICE_DIR, cmd_string.as_str()) {
        Ok(_) => (),
        Err(_) => return Err("Could not bind device to UDC"),
    };

    return Ok(());
}

fn exit_with_udc_not_configured_msg() {
    println!("\n\nThe Raspberry Pi has not been configured correctly to use the USB Device Controller.");
    println!("Please run the following commands to configure this:");

    println!("\n $ echo 'dtoverlay=dwc2' | sudo tee -a /boot/config.txt");
    println!(" $ echo 'dwc2' | sudo tee -a /etc/modules");
    println!("These two commands should enable the device tree overlay.");

    println!("\n $ sudo echo 'libcomposite' | sudo tee -a /etc/modules");
    println!("This should enable the libcomposite module at every following boot.");

    println!("\nThis tool will exit now.");
    println!("After running the commands, check if the values were appended with 'sudo cat /boot/config.txt' and then reboot your Raspberry Pi.");

    exit(0);
}
