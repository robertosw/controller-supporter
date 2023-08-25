use std::{
    fs::File,
    io::Write,
    process::{exit, Command},
};

use crate::usb_descr::{UsbDeviceDescriptor, UsbDeviceStrings};

// TODO Validate this from host side:
// echo 100 > file   appends a \n at the end of the file. writing with file.write_all does not do that,
// but this should be irrelevant because the driver will probably read the data

const BASE_DIR: &str = "/sys/kernel/config/usb_gadget";
const DEVICE_DIR: &str = "/sys/kernel/config/usb_gadget/raspi";
const ENG_STR_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/strings/0x409";
const CONFIGS_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/configs/c.1";
const FNC_HID_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/functions/hid.usb0";

/// Using linux' ConfigFS, create the given usb device
pub fn enable_gadget_mode(device: UsbDeviceDescriptor, device_strings: UsbDeviceStrings) {
    match create_directories() {
        Ok(_) => (),
        Err(_) => {
            println!("Error while creating directories, stopping.");
            return;
        }
    };

    match write_device_descriptor(&device) {
        Ok(_) => (),
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    match write_strings(device_strings) {
        Ok(_) => (),
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    match write_config(&device) {
        Ok(_) => (),
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    match bind_to_udc() {
        Ok(_) => (),
        Err(err) => {
            println!("{err}");
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
    // The system / driver already creates the directory "strings"
    match run_cmd("/sys/kernel/config/usb_gadget/raspi/strings", "mkdir 0x409") {
        Ok(_) => (),
        Err(_) => return Err(()),
    };

    // Configuration
    // The system / driver already creates the directory "configs"
    match run_cmd("/sys/kernel/config/usb_gadget/raspi/configs", "mkdir c.1") {
        Ok(_) => (),
        Err(_) => return Err(()),
    };
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
    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/bcdDevice"))
    {
        Ok(mut file) => match file.write_all(&device.bcd_device.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bcdDevice"),
        },
        Err(_) => return Err("Could not open file bcdDevice"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/bcdUSB"))
    {
        Ok(mut file) => match file.write_all(&device.bcd_usb.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bcdUSB"),
        },
        Err(_) => return Err("Could not open file bcdUSB"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/bDeviceClass"))
    {
        Ok(mut file) => match file.write_all(&device.b_device_class.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bDeviceClass"),
        },
        Err(_) => return Err("Could not open file bDeviceClass"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/bDeviceSubClass"))
    {
        Ok(mut file) => match file.write_all(&device.b_device_sub_class.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bDeviceSubClass"),
        },
        Err(_) => return Err("Could not open file bDeviceSubClass"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/bDeviceProtocol"))
    {
        Ok(mut file) => match file.write_all(&device.b_device_protocol.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bDeviceProtocol"),
        },
        Err(_) => return Err("Could not open file bDeviceProtocol"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/bMaxPacketSize0"))
    {
        Ok(mut file) => match file.write_all(&device.b_max_packet_size0.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bMaxPacketSize0"),
        },
        Err(_) => return Err("Could not open file bMaxPacketSize0"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/idVendor"))
    {
        Ok(mut file) => match file.write_all(&device.id_vendor.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file idVendor"),
        },
        Err(_) => return Err("Could not open file idVendor"),
    };

    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(DEVICE_DIR.to_string() + "/idProduct"))
    {
        Ok(mut file) => match file.write_all(&device.id_product.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file idProduct"),
        },
        Err(_) => return Err("Could not open file idProduct"),
    };

    return Ok(());
}

/// Writes the contents of each string into the corresponding file, if the string is not empty
///
/// Returns with `Err` as soon as one write operation is not successful
fn write_strings(device_strings: UsbDeviceStrings) -> Result<(), &str> {
    if !device_strings.manufacturer.is_empty() {
        match File::create(&(ENG_STR_DIR.to_string() + "/manufacturer")) {
            Ok(mut file) => match file.write_all(device_strings.manufacturer.as_bytes()) {
                Ok(_) => (),
                Err(_) => return Err("Could not write to file manufacturer"),
            },
            Err(_) => return Err("Could not open file manufacturer"),
        };
    }

    if !device_strings.product.is_empty() {
        match File::create(&(ENG_STR_DIR.to_string() + "/product")) {
            Ok(mut file) => match file.write_all(device_strings.product.as_bytes()) {
                Ok(_) => (),
                Err(_) => return Err("Could not write to file product"),
            },
            Err(_) => return Err("Could not open file product"),
        };
    }

    if !device_strings.serialnumber.is_empty() {
        match File::create(&(ENG_STR_DIR.to_string() + "/serialnumber")) {
            Ok(mut file) => match file.write_all(device_strings.serialnumber.as_bytes()) {
                Ok(_) => (),
                Err(_) => return Err("Could not write to file serialnumber"),
            },
            Err(_) => return Err("Could not open file serialnumber"),
        };
    }

    return Ok(());
}

///
fn write_config(device: &UsbDeviceDescriptor) -> Result<(), &str> {
    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(CONFIGS_DIR.to_string() + "/bmAttributes"))
    {
        Ok(mut file) => match file.write_all(&device.struct_configuration.bm_attributes.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file bmAttributes"),
        },
        Err(_) => return Err("Could not open file bmAttributes"),
    }

    // this value is orignially called bMaxPower (usb.org and in kernel source code)
    // but this file gets created by the driver as soon as a folder is created in /configs
    match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&(CONFIGS_DIR.to_string() + "/MaxPower"))
    {
        Ok(mut file) => match file.write_all(&device.struct_configuration.max_power.to_string().as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("Could not write to file MaxPower"),
        },
        Err(_) => return Err("Could not open file MaxPower"),
    }

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

/// always runs command as sudo
pub fn run_cmd(current_dir: &str, cmd: &str) -> Result<(), ()> {
    println!("\n$ sudo {cmd}");
    let args: Vec<&str> = cmd.split_whitespace().collect();

    let output = match Command::new("sudo").args(args).current_dir(current_dir).output() {
        Ok(output) => output,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(());
        }
    };
    let stdout = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(error) => {
            println!("! stdout of command {:?} could not be parsed: {:?}", cmd, error);
            return Err(());
        }
    };
    let stderr = match String::from_utf8(output.stderr) {
        Ok(string) => string,
        Err(error) => {
            println!("! stderr of command {:?} could not be parsed: {:?}", cmd, error);
            return Err(());
        }
    };
    println!("> {:?}", stdout);
    println!("! {:?}", stderr);

    return Ok(());
}
