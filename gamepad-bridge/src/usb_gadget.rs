#![allow(unused_variables)] // TODO remove!

use std::process::{exit, Command};

const DIR: &str = "/sys/kernel/config/usb_gadget";
const STR_ENG: &str = "strings/0x409";
const CONFIG_DIR: &str = "configs/c.1";
const FN_HID: &str = "functions/hid.usb0";

/// Using linux' ConfigFS, create a new usb gadget
pub fn configure_as_gadget(
    name: &str,
    // vendor_id: &str,
    // product_id: &str,
    serialnr: &str,
    manufacturer: &str,
    product_name: &str,
    config_name: &str, // example: Wireless Controller
    hid_protocol: u8,
    hid_subclass: u8,
    hid_report_length: u8,
    // TODO: Make this a struct
) {
    // const REPORT_DESC: &str = "\\x05\\x01\\x09\\x06\\xa1\\x01\\x05\\x07\\x19\\xe0\\x29\\xe7\\x15\\x00\\x25\\x01\\x75\\x01\\x95\\x08\\x81\\x02\\x95\\x01\\x75\\x08\\x81\\x03\\x95\\x05\\x75\\x01\\x05\\x08\\x19\\x01\\x29\\x05\\x91\\x02\\x95\\x01\\x75\\x03\\x91\\x03\\x95\\x06\\x75\\x08\\x15\\x00\\x25\\x65\\x05\\x07\\x19\\x00\\x29\\x65\\x81\\x00\\xc0";

    // this is the above descriptor just transformed
    // TODO: Read what really has to get into the report_desc
    let report_desc: [u8; 64] = [
        0x05, 0x01, // Usage Page (Generic Desktop Controls)
        0x09, 0x06, // Usage (Keyboard)
        0xA1, 0x01, // Collection (Application)
        0x05, 0x07, // Usage Page (Keyboard/Keypad)
        0x19, 0xE0, // Usage Minimum (Left Control)
        0x29, 0xE7, // Usage Maximum (Right GUI)
        0x15, 0x00, // Logical Minimum (0)
        0x25, 0x01, // Logical Maximum (1)
        0x75, 0x01, // Report Size (1)
        0x95, 0x08, // Report Count (8)
        0x81, 0x02, // Input (Data, Variable, Absolute), Modifier byte
        0x95, 0x01, // Report Count (1)
        0x75, 0x08, // Report Size (8)
        0x81, 0x03, // Input (Constant), Reserved byte
        0x95, 0x05, // Report Count (5)
        0x75, 0x01, // Report Size (1)
        0x05, 0x08, // Usage Page (LEDs)
        0x19, 0x01, // Usage Minimum (Num Lock)
        0x29, 0x05, // Usage Maximum (Kana)
        0x91, 0x02, // Output (Data, Variable, Absolute), LED report
        0x95, 0x01, // Report Count (1)
        0x75, 0x03, // Report Size (3)
        0x91, 0x03, // Output (Constant), LED report padding
        0x95, 0x06, // Report Count (6)
        0x75, 0x08, // Report Size (8)
        0x15, 0x00, // Logical Minimum (0)
        0x25, 0x65, // Logical Maximum (101)
        0x05, 0x07, // Usage Page (Keyboard/Keypad)
        0x19, 0x00, // Usage Minimum (Reserved (no event indicated))
        0x29, 0x65, // Usage Maximum (Keyboard Application)
        0x81, 0x00, // Input (Data, Array), Key array (6 keys)
        0xC0, 0x00, // End Collection
    ];

    let report_desc_str: String = report_desc.iter().map(|&byte| format!("\\x{:02X}", byte)).collect();
    let report_desc_str: &str = report_desc_str.as_str();

    println!("========== configuring device as usb gadget ==========");

    println!("\n $ =command \t > =stdout \t ! =stderr");

    run_cmd(format!("sudo modprobe libcomposite"));
    run_cmd(format!("sudo mkdir {DIR}/{name}"));

    // TODO somehow use hex int here
    run_cmd(format!("sudo echo 0x1d6b > {DIR}/{name}/idVendor"));
    run_cmd(format!("sudo echo 0x0104 > {DIR}/{name}/idProduct"));
    run_cmd(format!("sudo echo 0x0100 > {DIR}/{name}/bcdDevice"));
    run_cmd(format!("sudo echo 0x0200 > {DIR}/{name}/bcdUSB"));
    run_cmd(format!("sudo mkdir -p {DIR}/{name}/{STR_ENG}"));
    run_cmd(format!("sudo echo '{serialnr}' > {DIR}/{name}/{STR_ENG}/serialnumber",));
    run_cmd(format!("sudo echo '{manufacturer}' > {DIR}/{name}/{STR_ENG}/manufacturer",));
    run_cmd(format!("sudo echo '{product_name}' > {DIR}/{name}/{STR_ENG}/product"));
    run_cmd(format!("sudo mkdir -p {DIR}/{name}/{CONFIG_DIR}/{STR_ENG}"));
    run_cmd(format!("sudo echo '{config_name}' > {DIR}/{name}/{CONFIG_DIR}/{STR_ENG}/configuration"));
    run_cmd(format!("sudo echo 500 > {DIR}/{name}/{CONFIG_DIR}/MaxPower"));

    // Adding HID functions
    run_cmd(format!("sudo mkdir -p {DIR}/{name}/{FN_HID}"));
    run_cmd(format!("sudo echo {hid_protocol} > {DIR}/{name}/{FN_HID}/protocol"));
    run_cmd(format!("sudo echo {hid_subclass} > {DIR}/{name}/{FN_HID}/subclass"));
    run_cmd(format!("sudo echo {hid_report_length} > {DIR}/{name}/{FN_HID}/report_length"));
    // these two are not quite the same, using the first returns in: Syntax error: Unterminated quoted string
    // run_cmd(format!("sudo sh -c 'echo -ne {REPORT_DESC} > {DIR}/{name}/{FN_HID}/report_desc' "));
    run_cmd(String::from("sudo sh -c ") + "echo -ne" + report_desc_str + " > " + DIR + "/" + name + "/" + FN_HID + "/report_desc");
    run_cmd(format!("sudo ln -s {DIR}/{name}/{FN_HID} {DIR}/{name}/{CONFIG_DIR}/"));

    let output = match Command::new("ls").arg("/sys/class/udc").output() {
        Ok(output) => output,
        Err(error) => {
            println!("unwrapping the output failed: {:?}", error);
            return;
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
            return;
        }
    };

    // run_cmd(format!("sudo sh -c 'echo {first_udc} > {DIR}/{name}/UDC' "));
    run_cmd(String::from("sudo sh -c ") + "echo " + first_udc + " > " + DIR + "/" + name + "/UDC");

    println!("\n");
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

/// Runs the given command, waiting for its return and displays any output or error messages
///
/// ```Rust
/// // example call:
/// run_cmd("echo Hello World")
/// ```
fn run_cmd(input: String) {
    println!("\n$ {input}");

    let (cmd, remainder) = match input.split_once(char::is_whitespace) {
        Some((extracted, remainder)) => (extracted, remainder),
        None => return,
    };

    let args: Vec<&str> = remainder.split_whitespace().collect();

    let output = match Command::new(cmd).args(args).output() {
        Ok(output) => output,
        Err(error) => {
            println!("Error: {:?}", error);
            return;
        }
    };

    let stdout = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(error) => {
            println!("! stdout of command {:?} could not be parsed", input);
            return;
        }
    };
    let stderr = match String::from_utf8(output.stderr) {
        Ok(string) => string,
        Err(error) => {
            println!("! stderr of command {:?} could not be parsed: {:?}", input, error);
            return;
        }
    };

    println!("> {:?}", stdout);
    println!("! {:?}", stderr);
}

/*
    #!/bin/bash
    cd /sys/kernel/config/usb_gadget/
    mkdir -p raspi
    cd raspi
    echo 0x1d6b > idVendor # Linux Foundation
    echo 0x0104 > idProduct # Multifunction Composite Gadget
    echo 0x0100 > bcdDevice # v1.0.0
    echo 0x0200 > bcdUSB # USB2
    mkdir -p strings/0x409
    echo "fedcba9876543210" > strings/0x409/serialnumber
    echo "Robert Oswald" > strings/0x409/manufacturer
    echo "Raspberry Pi Zero 2 as USB Device" > strings/0x409/product
    mkdir -p configs/c.1/strings/0x409
    echo "Config 1: ECM network" > configs/c.1/strings/0x409/configuration
    echo 500 > configs/c.1/MaxPower

    # Add functions here
    mkdir -p functions/hid.usb0
    echo 0 > functions/hid.usb0/protocol
    echo 0 > functions/hid.usb0/subclass
    echo 64 > functions/hid.usb0/report_length
    echo -ne \\x05\\x01\\x09\\x06\\xa1\\x01\\x05\\x07\\x19\\xe0\\x29\\xe7\\x15\\x00\\x25\\x01\\x75\\x01\\x95\\x08\\x81\\x02\\x95\\x01\\x75\\x08\\x81\\x03\\x95\\x05\\x75\\x01\\x05\\x08\\x19\\x01\\x29\\x05\\x91\\x02\\x95\\x01\\x75\\x03\\x91\\x03\\x95\\x06\\x75\\x08\\x15\\x00\\x25\\x65\\x05\\x07\\x19\\x00\\x29\\x65\\x81\\x00\\xc0 > functions/hid.usb0/report_desc
    ln -s functions/hid.usb0 configs/c.1/
    # End functions

    ls /sys/class/udc > UDC
*/

/*
   mkdir -p /sys/kernel/config/usb_gadget/g1
   echo "${bcd_usb}" > /sys/kernel/config/usb_gadget/g1/bcdUSB
   echo 0x0200 > /sys/kernel/config/usb_gadget/g1/bcdUSB
   echo "${device_class}" > /sys/kernel/config/usb_gadget/g1/bDeviceClass
   echo "${device_sub_class}" > /sys/kernel/config/usb_gadget/g1/bDeviceSubClass
   echo "${device_protocol}" > /sys/kernel/config/usb_gadget/g1/bDeviceProtocol
   echo 0x120 > /sys/kernel/config/usb_gadget/g1/idVendor
   echo "${product_id}" > /sys/kernel/config/usb_gadget/g1/idProduct
   echo "${bcd_device}" > /sys/kernel/config/usb_gadget/g1/bcdDevice
   mkdir -p /sys/kernel/config/usb_gadget/g1/strings/0x409
   echo "${manufacturer}" > /sys/kernel/config/usb_gadget/g1/strings/0x409/manufacturer
   echo "${product}" > /sys/kernel/config/usb_gadget/g1/strings/0x409/product
   echo "HID Keyboard" > /sys/kernel/config/usb_gadget/g1/strings/0x409/configuration
*/

/*
  mkdir -p /sys/kernel/config/usb_gadget/g1/functions/hid.usb0
  echo "1" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/protocol
  echo "1" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/subclass
  echo "${hid_report_length}" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/report_length
  echo "${hid_report_descriptor}" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/report_desc
  mkdir -p /sys/kernel/config/usb_gadget/g1/configs/c.1/strings/0x409
  echo "Config 1: HID Keyboard" > /sys/kernel/config/usb_gadget/g1/configs/c.1/strings/0x409/configuration
  echo 250 > /sys/kernel/config/usb_gadget/g1/configs/c.1/MaxPower
  ln -s /sys/kernel/config/usb_gadget/g1/functions/hid.usb0 /sys/kernel/config/usb_gadget/g1/configs/c.1/
*/
