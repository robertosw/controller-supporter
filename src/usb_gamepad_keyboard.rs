use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::thread;
use std::time::Duration;

use crate::universal_gamepad::UniversalGamepad;
use crate::usb_gadget::*;
use crate::UsbGadgetDescriptor;

pub const GENERIC_KEYBOARD: UsbGadgetDescriptor = UsbGadgetDescriptor {
    bcd_usb: 0x0200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 8,
    id_vendor: 0x1d6b,
    id_product: 0x0104,
    bcd_device: 0x0100,
    write_output_once: _write_output_once,
    strings_0x409: UsbGadgetStrings {
        manufacturer: "Tobias Girstmair",
        product: "iSticktoit.net USB Device",
        serialnumber: "fedcba9876543210",
    },
    configs_c1: UsbGadgetConfigs {
        bm_attributes: 0b10000000,
        max_power: 250,
        configs_string: "Configuration 1",
    },
    functions_hid: UsbGadgetFunctionsHid {
        hid_subclass: 1,
        protocol: 1,
        report_length: 8,
        report_descriptor: &[
            0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
            0x09, 0x06, // Usage (Keyboard)
            0xA1, 0x01, // Collection (Application)
            0x05, 0x07, //   Usage Page (Kbrd/Keypad)
            0x19, 0xE0, //   Usage Minimum (0xE0)
            0x29, 0xE7, //   Usage Maximum (0xE7)
            0x15, 0x00, //   Logical Minimum (0)
            0x25, 0x01, //   Logical Maximum (1)
            0x75, 0x01, //   Report Size (1)
            0x95, 0x08, //   Report Count (8)
            0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0x95, 0x01, //   Report Count (1)
            0x75, 0x08, //   Report Size (8)
            0x81, 0x03, //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0x95, 0x05, //   Report Count (5)
            0x75, 0x01, //   Report Size (1)
            0x05, 0x08, //   Usage Page (LEDs)
            0x19, 0x01, //   Usage Minimum (Num Lock)
            0x29, 0x05, //   Usage Maximum (Kana)
            0x91, 0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x95, 0x01, //   Report Count (1)
            0x75, 0x03, //   Report Size (3)
            0x91, 0x03, //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x95, 0x06, //   Report Count (6)
            0x75, 0x08, //   Report Size (8)
            0x15, 0x00, //   Logical Minimum (0)
            0x25, 0x65, //   Logical Maximum (101)
            0x05, 0x07, //   Usage Page (Kbrd/Keypad)
            0x19, 0x00, //   Usage Minimum (0x00)
            0x29, 0x65, //   Usage Maximum (0x65)
            0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0xC0, // End Collection
        ],
    },
};

fn _write_output_once(_gamepad: &UniversalGamepad, _counter: u8, _seconds: u8) {
    const REPORT_LENGTH: usize = 8;

    let mut hidg0 = match File::options().write(true).append(false).open("/dev/hidg0") {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open file hidg0 {err}");
            exit(1);
        }
    };

    let out: [u8; REPORT_LENGTH] = [0x11, 0x22, 0x33, 0x44, 0x55, 0xFF, 0xAA, 0x00];

    match hidg0.write_all(&out) {
        // Ok(bytes) => print!("{bytes}b out"),
        Ok(_) => (),
        Err(err) => {
            println!("write to hidg0 failed: {:?}", err);
        }
    }

    // TODO achieve a real timed interval
    thread::sleep(Duration::from_millis(150));

    let out: [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];

    match hidg0.write_all(&out) {
        // Ok(bytes) => print!("{bytes}b out"),
        Ok(_) => (),
        Err(err) => {
            println!("write to hidg0 failed: {:?}", err);
        }
    }

    thread::sleep(Duration::from_millis(150));
}
