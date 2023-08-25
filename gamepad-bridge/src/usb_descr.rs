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

/// - `b_length` (Size of this descriptor) is always **18 bytes**
/// - fields starting with `struct_` are not taken from the official usb.org documentation
pub struct UsbDeviceDescriptor {
    pub b_descriptor_type: u8,    // Device descriptor type (assigned by USB)                       | probably 1
    pub bcd_usb: u16,             // TODO USB HID Specification Release 1.0.                        | try 0x200 (?= 2.00)
    pub b_device_class: u8,       // class code                                                     | 0x00 for HID
    pub b_device_sub_class: u8,   // subclass code                                                  | 0x00 for HID
    pub b_device_protocol: u8,    // protocol                                                       | 0x00 for HID
    pub b_max_packet_size0: u8,   // Maximum packet size for endpoint zero                          | 8 / 16 / 32 / 64
    pub id_vendor: u16,           //
    pub id_product: u16,          //
    pub bcd_device: u16,          // Device release number (assigned by manufacturer)               | try 0x100 (= 1.00)
    pub i_manufacturer: u8,       // TODO Index of String descriptor describing manufacturer.       | How can you set this?
    pub i_product: u8,            // TODO Index of string descriptor describing product.            | How can you set this?
    pub i_serial_number: u8,      // Index of String descriptor describing the device’s             | 0x00 for no serial number
    pub b_num_configurations: u8, // How many configuration does this device have                   | in this case, 0x01
    pub struct_configuration: UsbConfigurationDescriptor,
}

pub struct UsbDeviceStrings<'a> {
    pub serialnumber: &'a str,
    pub product: &'a str,
    pub manufacturer: &'a str,
}

/// - `b_length` (Size of this descriptor) is always **9 bytes**
/// - fields starting with `struct_` are not taken from the official usb.org documentation
pub struct UsbConfigurationDescriptor {
    pub b_descriptor_type: u8,     // Configuration (assigned by USB).                              | 0x02
    pub w_total_length: u16,       // TODO Total length of data returned (see page 77)              | has to be calculated (all descriptors are fixed size)
    pub b_num_interfaces: u8,      // Number of interfaces supported                                | in this case, 0x01
    pub b_configuration_value: u8, // basically the id of this configuration                        | 0x01
    pub i_configuration: u8,       // Index of string descriptor for this configuration             | 0x00  = no string
    pub bm_attributes: u8,         // bit8=Bus Powered  bit7=Self Powered  bit6=Remote Wakeup       | 0xc0 = 1100 0000 == self and bus powered
    pub max_power: u8,             // Maximum power consumption IN 2mA STEPS!!                      | 0xFA = 250 decimal == 500mA
    pub struct_interface: UsbInterfaceDescriptor,
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

/// - `b_length` (Size of this descriptor) is always **7 bytes**
pub struct UsbEndpointDescriptor {
    pub b_descriptor_type: u8,  // Endpoint descriptor type (assigned by USB).                          | 0x05
    pub b_endpoint_address: u8, // TODO Explanation below, this might get set by linux gadget drivers   | 0x84  EP 4 IN
    pub bm_attributes: u8,      // Explanation below                                                    | 00000011 = 3
    pub w_max_packet_size: u8,  // max packet size                                                      | 0x0040 = 64 bytes
    pub b_interval: u8,         // in ms                                                                | 0x06
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
