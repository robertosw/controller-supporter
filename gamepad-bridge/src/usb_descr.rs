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
    fs::File,
    io::Write,
    process::{exit, Command},
};

use crate::print_and_exit;
use crate::run_cmd;

const BASE_DIR: &str = "/sys/kernel/config/usb_gadget";
const DEVICE_DIR: &str = "/sys/kernel/config/usb_gadget/raspi";
const ENG_STR_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/strings/0x409";
const CONFIGS_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/configs/c.1";
const FNC_HID_DIR: &str = "/sys/kernel/config/usb_gadget/raspi/functions/hid.usb0";

/// - `b_length` (Size of this descriptor) is always **18 bytes**
/// - fields starting with `struct_` are not taken from the official usb.org documentation
pub struct UsbDeviceDescriptor<'a> {
    // pub b_descriptor_type: u8,       // Device descriptor type (assigned by USB)                 | Set by gadget driver
    // pub b_num_configurations: u8,    // How many configuration does this device have             | Set by gadget driver
    // pub i_manufacturer: u8,          // Index of String descriptor describing manufacturer.      | Set by gadget driver according to contents of /strings
    // pub i_product: u8,               // Index of string descriptor describing product.           | Set by gadget driver according to contents of /strings
    // pub i_serial_number: u8,         // Index of String descriptor describing the device’s       | Set by gadget driver according to contents of /strings
    pub bcd_usb: u16,           // USB HID Specification Release 1.0.                               | 0x200 = 2.00
    pub b_device_class: u8,     // class code                                                       | 0x00 for HID
    pub b_device_sub_class: u8, // subclass code                                                    | 0x00 for HID
    pub b_device_protocol: u8,  // protocol                                                         | 0x00 for HID
    pub b_max_packet_size0: u8, // Maximum packet size for endpoint zero                            | 8 / 16 / 32 / 64
    pub id_vendor: u16,         //
    pub id_product: u16,        //
    pub bcd_device: u16,        // Device release number (assigned by manufacturer)                 | 0x100 = 1.00
    pub struct_strings: UsbDeviceStrings<'a>,
    pub struct_configuration: UsbConfigurationDescriptor,
}

// TODO Validate this from host side:
// echo 100 > file   appends a \n at the end of the file. writing with file.write_all does not do that,
// but this should be irrelevant because the driver will probably read the data

impl UsbDeviceDescriptor<'_> {
    /// Using linux' ConfigFS, create the given usb device
    pub fn configure_device(&self) {
        self.create_directories();
        self.write_to_disk();
        self.struct_configuration.write_to_disk();
        self.struct_configuration.struct_interface.write_to_disk();
        self.struct_configuration.struct_interface.struct_hid_device.write_to_disk();
        self.struct_configuration.struct_interface.struct_endpoint_in.write_to_disk();
        self.struct_configuration.struct_interface.struct_endpoint_out.write_to_disk();
        self.bind_to_udc();
    }

    /// will exit if any operation is not successful
    fn create_directories(&self) {
        match run_cmd("/sys/kernel/config/usb_gadget", "mkdir raspi") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi", 9),
        };

        // Strings
        // The system / driver already creates the directory "strings"
        match run_cmd("/sys/kernel/config/usb_gadget/raspi/strings", "mkdir 0x409") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi/strings/0x409", 9),
        };

        // Configuration
        // The system / driver already creates the directory "configs"
        match run_cmd("/sys/kernel/config/usb_gadget/raspi/configs", "mkdir c.1") {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not create directory /sys/kernel/config/usb_gadget/raspi/configs/c.1", 9),
        };
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
    fn write_to_disk(&self) {
        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/bcdDevice"))
        {
            Ok(mut file) => match file.write_all(&self.bcd_device.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bcdDevice", 10),
            },
            Err(_) => print_and_exit("Could not open file bcdDevice", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/bcdUSB"))
        {
            Ok(mut file) => match file.write_all(&self.bcd_usb.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bcdUSB", 10),
            },
            Err(_) => print_and_exit("Could not open file bcdUSB", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/bDeviceClass"))
        {
            Ok(mut file) => match file.write_all(&self.b_device_class.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bDeviceClass", 10),
            },
            Err(_) => print_and_exit("Could not open file bDeviceClass", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/bDeviceSubClass"))
        {
            Ok(mut file) => match file.write_all(&self.b_device_sub_class.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bDeviceSubClass", 10),
            },
            Err(_) => print_and_exit("Could not open file bDeviceSubClass", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/bDeviceProtocol"))
        {
            Ok(mut file) => match file.write_all(&self.b_device_protocol.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bDeviceProtocol", 10),
            },
            Err(_) => print_and_exit("Could not open file bDeviceProtocol", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/bMaxPacketSize0"))
        {
            Ok(mut file) => match file.write_all(&self.b_max_packet_size0.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bMaxPacketSize0", 10),
            },
            Err(_) => print_and_exit("Could not open file bMaxPacketSize0", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/idVendor"))
        {
            Ok(mut file) => match file.write_all(&self.id_vendor.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file idVendor", 10),
            },
            Err(_) => print_and_exit("Could not open file idVendor", 11),
        };

        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(DEVICE_DIR.to_string() + "/idProduct"))
        {
            Ok(mut file) => match file.write_all(&self.id_product.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file idProduct", 10),
            },
            Err(_) => print_and_exit("Could not open file idProduct", 11),
        };
    }

    fn bind_to_udc(&self) {
        let output = match Command::new("ls").arg("/sys/class/udc").output() {
            Ok(output) => output,
            Err(error) => {
                print_and_exit(format!("unwrapping the output failed: {:?}", error).as_str(), 1);
                return;
            }
        };

        let udc_name = String::from_utf8(output.stdout).ok().unwrap();

        // if there are multiple udcs (shouldn't be the case on the zero 2), take the first
        let (first_udc, _) = match udc_name.split_once(char::is_whitespace) {
            Some((first_udc, remainder)) => (first_udc, remainder),
            None => {
                // There are no udc registered
                println!("\n\nThe Raspberry Pi has not been configured correctly to use the USB Device Controller.");
                println!("Please run the following commands to configure this:");

                println!("\n $ echo 'dtoverlay=dwc2' | sudo tee -a /boot/config.txt");
                println!(" $ echo 'dwc2' | sudo tee -a /etc/modules");
                println!("These two commands should enable the device tree overlay.");

                println!("\n $ sudo echo 'libcomposite' | sudo tee -a /etc/modules");
                println!("This should enable the libcomposite module at every following boot.");

                println!("\nThis tool will exit now.");
                println!("After running the commands, check if the values were appended with 'sudo cat /boot/config.txt' and then reboot your Raspberry Pi.");

                exit(1);
            }
        };

        // run_cmd(format!("sudo sh -c 'echo {first_udc} > {DIR}/{name}/UDC' "));
        let cmd_string = String::from("sudo sh -c ") + "echo " + first_udc + " > UDC";

        match run_cmd(DEVICE_DIR, cmd_string.as_str()) {
            Ok(_) => (),
            Err(_) => print_and_exit("Could not bind device to UDC", 1),
        };
    }
}

pub struct UsbDeviceStrings<'a> {
    pub serialnumber: &'a str, // can be empty
    pub product: &'a str,      // can be empty
    pub manufacturer: &'a str, // can be empty
}

impl UsbDeviceStrings<'_> {
    /// Writes the contents of each string into the corresponding file, if the string is not empty
    ///
    /// **Exits** as soon as one write operation is not successful
    fn write_to_disk(&self) {
        if !&self.manufacturer.is_empty() {
            match File::create(&(ENG_STR_DIR.to_string() + "/manufacturer")) {
                Ok(mut file) => match file.write_all(&self.manufacturer.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => print_and_exit("Could not write to file manufacturer", 14),
                },
                Err(_) => print_and_exit("Could not open file manufacturer", 15),
            };
        }

        if !&self.product.is_empty() {
            match File::create(&(ENG_STR_DIR.to_string() + "/product")) {
                Ok(mut file) => match file.write_all(&self.product.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => print_and_exit("Could not write to file product", 14),
                },
                Err(_) => print_and_exit("Could not open file product", 15),
            };
        }

        if !&self.serialnumber.is_empty() {
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

/// - `b_length` (Size of this descriptor) is always **9 bytes**
/// - fields starting with `struct_` are not taken from the official usb.org documentation
pub struct UsbConfigurationDescriptor {
    // pub b_descriptor_type: u8,       // Configuration (assigned by USB).                         | Set by gadget driver
    // pub w_total_length: u16,         // Total length of data returned (see page 77)              | Set by gadget driver
    // pub b_num_interfaces: u8,        // Number of interfaces supported                           | Set by gadget driver
    // pub b_configuration_value: u8,   // basically the id of this configuration                   | Set by gadget driver
    // pub i_configuration: u8,         // Index of string descriptor for this configuration        | Set by gadget driver
    pub bm_attributes: u8, // bit8=Bus Powered  bit7=Self Powered  bit6=Remote Wakeup               | 0xc0 = 1100 0000 == self and bus powered
    pub max_power: u8,     // Maximum power consumption in 2mA STEPS!!                              | 0xFA = 250 decimal == 500mA
    pub struct_interface: UsbInterfaceDescriptor,
}

impl UsbConfigurationDescriptor {
    fn write_to_disk(&self) {
        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(CONFIGS_DIR.to_string() + "/bmAttributes"))
        {
            Ok(mut file) => match file.write_all(&self.bm_attributes.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file bmAttributes", 12),
            },
            Err(_) => print_and_exit("Could not open file bmAttributes", 13),
        }

        // this value is orignially called bMaxPower (usb.org and in kernel source code)
        // but this file gets created by the driver as soon as a folder is created in /configs
        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&(CONFIGS_DIR.to_string() + "/MaxPower"))
        {
            Ok(mut file) => match file.write_all(&self.max_power.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => print_and_exit("Could not write to file MaxPower", 12),
            },
            Err(_) => print_and_exit("Could not open file MaxPower", 13),
        }
    }
}

/// - `b_length` (Size of this descriptor) is always **9 bytes**
/// - fields starting with `struct_` are not taken from the official usb.org documentation
pub struct UsbInterfaceDescriptor {
    pub b_descriptor_type: u8,     // Interface descriptor type (assigned by USB)                   | 0x04
    pub b_interface_number: u8,    // Interface Counter (zero based)                                | 0x00
    pub b_alternate_setting: u8,   // Value used to select alternate setting                        | 0x00
    pub b_num_endpoints: u8,       // Nr of endpoints used (excluding endpoint zero)                | 0x02 (PS5 has two, IN and OUT)
    pub b_interface_class: u8,     // Class code (assigned by USB).                                 | always 0x03 for HID devices
    pub b_interface_sub_class: u8, // 0 = None  1 = Boot Interface Subclass                         | 0x00
    pub b_interface_protocol: u8,  // 0 = None  1 = Keyboard  2 = Mouse                             | 0x00
    pub i_interface: u8,           // Index of string descriptor describing this interface          | 0x00
    pub struct_hid_device: UsbHidDeviceDescriptor,
    pub struct_endpoint_in: UsbEndpointDescriptor,
    pub struct_endpoint_out: UsbEndpointDescriptor,
}

impl UsbInterfaceDescriptor {
    fn write_to_disk(&self) {}
}

/// - `b_length` (Size of this descriptor) is always **9 bytes**
pub struct UsbHidDeviceDescriptor {
    pub b_descriptor_type: u8, // HID descriptor type (assigned by USB).                            | 0x21 = 32
    pub bcd_hid: u16,          // HID Class Specification release number                            | 0x111 = 1.11
    pub b_country_code: u8,    // Hardware target country                                           | 0x00
    pub b_num_descriptors: u8, // Number of HID class descriptors to follow                         | 0x01

    /// this is also called bDescriptorType in the docu.. quite confusing
    pub b_descriptor_type_report: u8, // Report descriptor type                                     | 0x22 = 33
    pub w_descriptor_length: u16, // Total length of Report descriptor                              | 0x111 = 273
}

impl UsbHidDeviceDescriptor {
    fn write_to_disk(&self) {}
}

/// - `b_length` (Size of this descriptor) is always **7 bytes**
pub struct UsbEndpointDescriptor {
    pub b_descriptor_type: u8,  // Endpoint descriptor type (assigned by USB).                          | 0x05
    pub b_endpoint_address: u8, // TODO Explanation below, this might get set by linux gadget drivers   | 0x84  EP 4 IN
    pub bm_attributes: u8,      // Explanation below                                                    | 00000011 = 3
    pub w_max_packet_size: u8,  // max packet size                                                      | 0x0040 = 64 bytes
    pub b_interval: u8,         // in ms                                                                | 0x06
}

impl UsbEndpointDescriptor {
    fn write_to_disk(&self) {}
}
/* bEndpointAddress explained
 *
 * The address of the endpoint on the USB device
 * described by this descriptor. The address is encoded as
 * follows:
 *
 * Bit 0..3 The endpoint number
 * Bit 4..6 Reserved, reset to zero
 * Bit 7 Direction, ignored for
 * Control endpoints:
 * 0 - OUT endpoint
 * 1 - IN endpoint
 */

/* bmAttributes explained
 *
 * This field describes the endpoint’s attributes when it is
 * configured using the bConfigurationValue.
 *
 * Bit 0..1     Transfer type:
 * 00           Control
 * 01           Isochronous
 * 10           Bulk
 * 11           Interrupt
 * All other bits are reserved.
 */

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
