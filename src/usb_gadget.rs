/* Important Notes
 *
 * https://www.usb.org/sites/default/files/hid1_11.pdf
 *
 * Pages 76 - 79
 *
 * For HID class devices:
 *  - The Class type is not defined at the Device descriptor level.
 *    The class type for a HID class device is defined by the Interface descriptor
 *  - Subclass field is used to identify Boot Devices.
*/

use std::{
    fs::{self, File},
    io::Write,
    process::exit,
    thread,
    time::Duration,
};

use crate::{
    helper_fn::{print_and_exit, run_cmd},
    universal_gamepad::UniversalGamepad,
};

const DEVICE_DIR: &str = "/sys/kernel/config/usb_gadget/raspi";
const ENG_STR_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/strings/0x409";
const CONFIGS_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/configs/c.1";
const FNC_HID_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/functions/hid.usb0";

pub struct UsbGadgetDescriptor<'a> {
    pub bcd_usb: u16,           // USB HID Specification Release 1.0.                               | 0x200 = 2.00
    pub b_device_class: u8,     // class code                                                       | 0x00 for HID
    pub b_device_sub_class: u8, // subclass code                                                    | 0x00 for HID
    pub b_device_protocol: u8,  // protocol                                                         | 0x00 for HID
    pub b_max_packet_size0: u8, // Maximum packet size for endpoint zero                            | 8 / 16 / 32 / 64
    pub id_vendor: u16,         //
    pub id_product: u16,        //
    pub bcd_device: u16,        // Device release number (assigned by manufacturer)                 | 0x100 = 1.00
    pub strings_0x409: UsbGadgetStrings<'a>,
    pub configs_c1: UsbGadgetConfigs<'a>,
    pub functions_hid: UsbGadgetFunctionsHid,
    pub write_output_once: fn(&UniversalGamepad, u8, u8),
}

impl UsbGadgetDescriptor<'_> {
    /// Moves all triggers and joysticks and presses and releases all buttons
    pub fn write_continously_testing(&self) -> ! {
        let mut gamepad: UniversalGamepad = UniversalGamepad::nothing_pressed();

        const OSCILLATE_UPPER: u8 = 192;
        const OSCILLATE_LOWER: u8 = 64;
        let mut oscillate: u8 = OSCILLATE_LOWER;
        let mut up: bool = true;

        // println!("sleeping 10s");
        // thread::sleep(Duration::from_secs(10));
        println!("lets go");

        loop {
            // This counts one byte at a time from OSCILLATE_LOWER to OSCILLATE_UPPER and back to OSCILLATE_LOWER

            match oscillate {
                OSCILLATE_LOWER => up = true,
                OSCILLATE_UPPER => up = false,
                _ => (),
            }

            if up && (oscillate < OSCILLATE_UPPER) {
                oscillate += 1;
            }
            if !up && (oscillate > OSCILLATE_LOWER) {
                oscillate -= 1;
            }

            gamepad.sticks.left.x = oscillate;
            gamepad.sticks.left.y = oscillate;
            gamepad.sticks.right.x = oscillate;
            gamepad.sticks.right.y = oscillate;
            gamepad.triggers.left = oscillate;
            gamepad.triggers.right = oscillate;
            println!("{oscillate}");

            self.write_output_once(&gamepad, 0, 0);

            // TODO achieve a real timed interval
            thread::sleep(Duration::from_millis(4));
        }
    }

    /// Calls the function pointer `write_output_once` (was provided at instantiation)
    pub fn write_output_once(&self, gamepad: &UniversalGamepad, counter: u8, seconds: u8) {
        (self.write_output_once)(gamepad, counter, seconds);
    }

    /// Using linux' ConfigFS, create the given usb device
    pub fn configure_device(&self) {
        self._create_directories();

        self._write_to_disk();
        self.configs_c1.write_to_disk();
        self.strings_0x409.write_to_disk();
        self.functions_hid.write_to_disk();

        self._assign_fn_to_config();
        match self._bind_to_udc() {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to bind gadget to udc: {:?}", err);

                println!("\n\nIts possible that your Raspberry Pi has not been configured correctly to use the USB Device Controller.");
                println!("Please take a look at the file Raspberry Pi Setup.md in the GitHub Repo and setup your Raspberry Pi");

                println!("\nThis tool will exit now.");
                exit(1);
            }
        };
    }

    // fn init_kernel_modules(&self) {
    //     match run_cmd("/", "modprobe dwc2") {
    //         Ok(_) => (),
    //         Err(_) => print_and_exit("Could load kernel module 'dwc2'", 20),
    //     };
    //     match run_cmd("/", "modprobe libcomposite") {
    //         Ok(_) => (),
    //         Err(_) => print_and_exit("Could load kernel module 'libcomposite'", 20),
    //     };
    // }

    /// will exit if any operation is not successful
    fn _create_directories(&self) {
        match run_cmd("/sys/kernel/config/usb_gadget", "mkdir raspi") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi", 9),
        };

        // Strings
        // The system already creates the directory "strings"
        match run_cmd("/sys/kernel/config/usb_gadget/raspi/strings", "mkdir 0x409") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi/strings/0x409", 9),
        };

        // Configuration
        // The system already creates the directory "configs"
        match run_cmd("/sys/kernel/config/usb_gadget/raspi/configs", "mkdir c.1") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi/configs/c.1", 9),
        };

        match run_cmd("/sys/kernel/config/usb_gadget/raspi/configs/c.1", "mkdir -p strings/0x409") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi/configs/c.1/strings/0x409", 14),
        };

        // Functions
        // The system already creates the directory "functions"
        match run_cmd("/sys/kernel/config/usb_gadget/raspi/functions", "mkdir hid.usb0") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi/functions/hid.usb0", 9),
        };
    }

    /// Writes the data of `UsbGadgetDescriptor` into the files `bcdDevice`, `bcdUSB`, `bDeviceClass`, `bDeviceSubClass`, `bDeviceProtocol`, `bMaxPacketSize0`, `idVendor`, `idProduct`
    ///
    /// Will exit if any operation is not successful
    ///
    /// <br>
    ///
    /// ### Why only these and not all?
    /// After creating any directory inside `/sys/kernel/config/usb_gadget` the system creates some basic structure.
    /// This structure does not cover all the possible fields.
    ///
    /// Comparing to usb.org specification, non-existent fields are:
    /// `bLength`, `bDescriptorType`, `iManufacturer`, `iProduct`, `iSerialNumber`, `bNumConfigurations`
    ///
    /// This implies that some of the work is done by the driver.
    fn _write_to_disk(&self) {
        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/bcdDevice")) {
            Ok(mut file) => match file.write_all(&self.bcd_device.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bcdDevice", 10),
            },
            Err(_) => print_and_exit("Could not open file bcdDevice", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/bcdUSB")) {
            Ok(mut file) => match file.write_all(&self.bcd_usb.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bcdUSB", 10),
            },
            Err(_) => print_and_exit("Could not open file bcdUSB", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/bDeviceClass")) {
            Ok(mut file) => match file.write_all(&self.b_device_class.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bDeviceClass", 10),
            },
            Err(_) => print_and_exit("Could not open file bDeviceClass", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/bDeviceSubClass")) {
            Ok(mut file) => match file.write_all(&self.b_device_sub_class.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bDeviceSubClass", 10),
            },
            Err(_) => print_and_exit("Could not open file bDeviceSubClass", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/bDeviceProtocol")) {
            Ok(mut file) => match file.write_all(&self.b_device_protocol.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bDeviceProtocol", 10),
            },
            Err(_) => print_and_exit("Could not open file bDeviceProtocol", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/bMaxPacketSize0")) {
            Ok(mut file) => match file.write_all(&self.b_max_packet_size0.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bMaxPacketSize0", 10),
            },
            Err(_) => print_and_exit("Could not open file bMaxPacketSize0", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/idVendor")) {
            Ok(mut file) => match file.write_all(&self.id_vendor.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file idVendor", 10),
            },
            Err(_) => print_and_exit("Could not open file idVendor", 11),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/idProduct")) {
            Ok(mut file) => match file.write_all(&self.id_product.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file idProduct", 10),
            },
            Err(_) => print_and_exit("Could not open file idProduct", 11),
        };
    }

    fn _assign_fn_to_config(&self) {
        match run_cmd(DEVICE_DIR, "ln -s functions/hid.usb0/ configs/c.1/") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not link functions (functions/hid.usb0/) to configs (configs/c.1/)", 14),
        }
    }

    fn _bind_to_udc(&self) -> Result<(), &str> {
        let paths = match fs::read_dir("/sys/class/udc") {
            Ok(paths) => paths,
            Err(err) => print_and_exit(format!("Error while reading directory /sys/class/udc: {:?}", err).as_str(), 20),
        };

        let dir_entry = match paths.last() {
            Some(result) => match result {
                Ok(dir_entry) => dir_entry,
                Err(_) => return Err("Error unwrapping DirEntry"),
            },
            None => return Err("/sys/class/udc appears to be empty"),
        };

        // satisfy the borrow checker :)
        let binding = dir_entry.file_name();
        let first_udc: &str = match binding.to_str() {
            Some(str) => str,
            None => return Err("Failed at transforming DirEntry to str"),
        };

        match File::options().write(true).truncate(true).open(&(DEVICE_DIR.to_string() + "/UDC")) {
            Ok(mut file) => match file.write_all(&first_udc.as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file UDC", 10),
            },
            Err(_) => print_and_exit("Could not open file UDC", 11),
        };

        return Ok(());
    }
}

pub struct UsbGadgetStrings<'a> {
    /// can be empty
    pub serialnumber: &'a str,
    /// can be empty
    pub product: &'a str,
    /// can be empty
    pub manufacturer: &'a str,
}

impl UsbGadgetStrings<'_> {
    /// Writes the contents of each string into the corresponding file, if the string is not empty
    ///
    /// **Exits** as soon as one write operation is not successful
    fn write_to_disk(&self) {
        if self.manufacturer.is_empty() == false {
            match File::create(&(ENG_STR_DIR.to_string() + "/manufacturer")) {
                Ok(mut file) => match file.write_all(&self.manufacturer.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => print_and_exit("Could not write to file manufacturer", 14),
                },
                Err(_) => print_and_exit("Could not open file manufacturer", 15),
            };
        }

        if self.product.is_empty() == false {
            match File::create(&(ENG_STR_DIR.to_string() + "/product")) {
                Ok(mut file) => match file.write_all(&self.product.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => print_and_exit("Could not write to file product", 14),
                },
                Err(_) => print_and_exit("Could not open file product", 15),
            };
        }

        if self.serialnumber.is_empty() == false {
            match File::create(&(ENG_STR_DIR.to_string() + "/serialnumber")) {
                Ok(mut file) => match file.write_all(&self.serialnumber.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => print_and_exit("Could not write to file serialnumber", 14),
                },
                Err(_) => print_and_exit("Could not open file serialnumber", 15),
            };
        }
    }
}

pub struct UsbGadgetConfigs<'a> {
    /// Left most bit always has to be set or file write is not permitted
    ///
    /// Allowed values are:
    /// - `1110 0000 = 224 = 0xE0`    Bus powered & Self powered & Remote Wakeup
    /// - `1100 0000 = 192 = 0xC0`    Bus powered & Self powered
    /// - `1000 0000 = 128 = 0x80`    Bus powered
    ///
    /// What these bits mean is from usb.org documentation, but kernel implementation might have changed that
    pub bm_attributes: u8,

    /// Max value is 500mA, because thats how USB works
    pub max_power: u16,

    pub configs_string: &'a str,
}

impl UsbGadgetConfigs<'_> {
    fn write_to_disk(&self) {
        match File::options().write(true).truncate(true).open(&(CONFIGS_DIR.to_string() + "/bmAttributes")) {
            Ok(mut file) => match file.write_all(&self.bm_attributes.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bmAttributes", 12),
            },
            Err(_) => print_and_exit("Could not open file bmAttributes", 13),
        }

        // this value is orignially called bMaxPower (usb.org and in kernel source code)
        // but this file gets created by the driver as soon as a folder is created in /configs
        match File::options().write(true).truncate(true).open(&(CONFIGS_DIR.to_string() + "/MaxPower")) {
            Ok(mut file) => match file.write_all(&self.max_power.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file MaxPower", 12),
            },
            Err(_) => print_and_exit("Could not open file MaxPower", 13),
        }

        // TODO if empty, dont write

        match File::options()
            .write(true)
            .truncate(true)
            .open(&(CONFIGS_DIR.to_string() + "/strings/0x409/configuration"))
        {
            Ok(mut file) => match file.write_all(&self.configs_string.as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file configuration", 12),
            },
            Err(_) => print_and_exit("Could not open file configuration", 13),
        }
    }
}

/// This represents everything that has to be written into the directory .../usb_gadget/NAME/functions/hid.usb0/
pub struct UsbGadgetFunctionsHid {
    /// HID protocol to use
    ///
    /// Default is `0`, Keyboard is `1`, Mouse might be `2`
    pub protocol: u8,

    /// data to be used in HID reports, except data passed with /dev/hidg<X>
    pub report_descriptor: &'static [u8],

    /// This is NOT the length of the report descriptor!
    pub report_length: u16, // Total length of Report descriptor          | 0x111 = 273

    /// HID subclass to use
    ///
    /// Default is `0`, Keyboard is `1`
    ///
    /// usb.org specification says `1` would mean boot interface
    pub hid_subclass: u8,
}

impl UsbGadgetFunctionsHid {
    fn write_to_disk(&self) {
        // protocol
        match File::options().write(true).truncate(true).open(&(FNC_HID_DIR.to_string() + "/protocol")) {
            Ok(mut file) => match file.write_all(&self.protocol.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file protocol", 12),
            },
            Err(_) => print_and_exit("Could not open file protocol", 13),
        }

        // report_length
        match File::options().write(true).truncate(true).open(&(FNC_HID_DIR.to_string() + "/report_length")) {
            Ok(mut file) => match file.write_all(&self.report_length.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file report_length", 12),
            },
            Err(_) => print_and_exit("Could not open file report_length", 13),
        }

        // subclass
        match File::options().write(true).truncate(true).open(&(FNC_HID_DIR.to_string() + "/subclass")) {
            Ok(mut file) => match file.write_all(&self.hid_subclass.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file subclass", 12),
            },
            Err(_) => print_and_exit("Could not open file subclass", 13),
        }

        // report_desc
        match File::options().write(true).truncate(true).open(&(FNC_HID_DIR.to_string() + "/report_desc")) {
            Ok(mut file) => match file.write_all(&self.report_descriptor) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file report_desc", 12),
            },
            Err(_) => print_and_exit("Could not open file report_desc", 13),
        }
    }
}

/* All of this as a shell script faster testing:

#!/bin/bash
cd /sys/kernel/config/usb_gadget/
mkdir -p isticktoit
cd isticktoit
echo 0x1d6b > idVendor # Linux Foundation
echo 0x0104 > idProduct # Multifunction Composite Gadget
echo 0x0100 > bcdDevice # v1.0.0
echo 0x0200 > bcdUSB # USB2
mkdir -p strings/0x409
echo "fedcba9876543210" > strings/0x409/serialnumber
echo "Tobias Girstmair" > strings/0x409/manufacturer
echo "iSticktoit.net USB Device" > strings/0x409/product
mkdir -p configs/c.1/strings/0x409
echo "Config 1: ECM network" > configs/c.1/strings/0x409/configuration
echo 250 > configs/c.1/MaxPower
mkdir -p functions/hid.usb0
echo 1 > functions/hid.usb0/protocol
echo 1 > functions/hid.usb0/subclass
echo 8 > functions/hid.usb0/report_length
echo -ne \\x05\\x01\\x09\\x06\\xa1\\x01\\x05\\x07\\x19\\xe0\\x29\\xe7\\x15\\x00\\x25\\x01\\x75\\x01\\x95\\x08\\x81\\x02\\x95\\x01\\x75\\x08\\x81\\x03\\x95\\x05\\x75\\x01\\x05\\x08\\x19\\x01\\x29\\x05\\x91\\x02\\x95\\x01\\x75\\x03\\x91\\x03\\x95\\x06\\x75\\x08\\x15\\x00\\x25\\x65\\x05\\x07\\x19\\x00\\x29\\x65\\x81\\x00\\xc0 > functions/hid.usb0/report_desc
ln -s functions/hid.usb0 configs/c.1/
ls /sys/class/udc > UDC

*/
