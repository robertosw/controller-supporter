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

pub struct DeviceDescriptor {
    b_length: u8,             // size of this descriptor                                        | 18 bytes = 0x12
    b_descriptor_type: u8,    // Device descriptor type (assigned by USB)                       | probably 1
    bcd_usb: u16,             // TODO USB HID Specification Release 1.0.                        | try 0x200 (?= 2.00)
    b_device_class: u8,       // class code                                                     | 0x00 for HID
    b_device_sub_class: u8,   // subclass code                                                  | 0x00 for HID
    b_device_protocol: u8,    // protocol                                                       | 0x00 for HID
    b_max_packet_size0: u8,   // Maximum packet size for endpoint zero                          | 8 / 16 / 32 / 64
    id_vendor: u16,           //
    id_product: u16,          //
    bcd_device: u16,          // Device release number (assigned by manufacturer)               | try 0x100 (= 1.00)
    i_manufacturer: u8,       // TODO Index of String descriptor describing manufacturer.       | How can you set this?
    i_product: u8,            // TODO Index of string descriptor describing product.            | How can you set this?
    i_serial_number: u8,      // Index of String descriptor describing the device’s             | 0x00 for no serial number
    b_num_configurations: u8, // How many configuration does this device have                   | in this case, 0x01
    configuration: ConfigurationDescriptor,
}

pub struct ConfigurationDescriptor {
    b_length: u8,              // Size of this descriptor in bytes                              | 0x09
    b_descriptor_type: u8,     // Configuration (assigned by USB).                              | 0x02
    w_total_length: u16,       // TODO Total length of data returned (see page 77)              | has to be calculated (all descriptors are fixed size)
    b_num_interfaces: u8,      // Number of interfaces supported                                | in this case, 0x01
    b_configuration_value: u8, // basically the id of this configuration                        | 0x01
    i_configuration: u8,       // Index of string descriptor for this configuration             | 0x00  = no string
    bm_attributes: u8,         // bit8=Bus Powered  bit7=Self Powered  bit6=Remote Wakeup       | 0xc0 = 1100 0000 == self and bus powered
    max_power: u8,             // Maximum power consumption IN 2mA STEPS!!                      | 0xFA = 250 decimal == 500mA
    interface: InterfaceDescriptor,
}

pub struct InterfaceDescriptor {
    b_length: u8,              // Size of this descriptor in bytes                              | 0x09
    b_descriptor_type: u8,     // Interface descriptor type (assigned by USB)                   | 0x04
    b_interface_number: u8,    // Interface Counter (zero based)                                | 0x00
    b_alternate_setting: u8,   // Value used to select alternate setting                        | 0x00
    b_num_endpoints: u8,       // Nr of endpoints used (excluding endpoint zero)                | 0x02 (PS5 has two, IN and OUT)
    b_interface_class: u8,     // Class code (assigned by USB).                                 | always 0x03 for HID devices
    b_interface_sub_class: u8, // 0 = None  1 = Boot Interface Subclass                         | 0x00
    b_interface_protocol: u8,  // 0 = None  1 = Keyboard  2 = Mouse                             | 0x00
    i_interface: u8,           // Index of string descriptor describing this interface          | 0x00
    hid_device: HidDeviceDescriptor,
    endpoint_in: EndpointDescriptor,
    endpoint_out: EndpointDescriptor,
}

pub struct HidDeviceDescriptor {
    b_length: u8,          // Size of this descriptor in bytes                               | 0x09
    b_descriptor_type: u8, // HID descriptor type (assigned by USB).                         | 0x21 = 32
    bcd_hid: u16,          // HID Class Specification release number                         | 0x111 = 1.11
    b_country_code: u8,    // Hardware target country                                        | 0x00
    b_num_descriptors: u8, // Number of HID class descriptors to follow                      | 0x01

    /// this is also called bDescriptorType in the docu.. quite confusing
    b_descriptor_type_report: u8, // Report descriptor type                                     | 0x22 = 33
    w_descriptor_length: u16, // Total length of Report descriptor                              | 0x111 = 273
    report: *const u8,        // the report descriptor (as ref to allow array)
}

pub struct EndpointDescriptor {
    b_length: u8,           // size of this descriptor in bytes                                     | 0x07
    b_descriptor_type: u8,  // Endpoint descriptor type (assigned by USB).                          | 0x05
    b_endpoint_address: u8, // TODO Explanation below, this might get set by linux gadget drivers   | 0x84  EP 4 IN
    bm_attributes: u8,      // Explanation below                                                    | 00000011 = 3
    w_max_packet_size: u8,  // max packet size                                                      | 0x0040 = 64 bytes
    b_interval: u8,         // in ms                                                                | 0x06
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

pub const PS5GAMEPAD: DeviceDescriptor = DeviceDescriptor {
    b_length: 18,
    b_descriptor_type: 1,
    bcd_usb: 0x200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 64,
    id_vendor: 0x054c,
    id_product: 0x0ce6,
    bcd_device: 0x100,
    i_manufacturer: 1,
    i_product: 2,
    i_serial_number: 0,
    b_num_configurations: 1,
    configuration: ConfigurationDescriptor {
        b_length: 9,
        b_descriptor_type: 2,
        w_total_length: (9 + 9 + 9 + 7 + 7), // configuration, interface, hid (without report), both endpoints
        b_num_interfaces: 1,
        b_configuration_value: 1,
        i_configuration: 0,
        bm_attributes: 0b1100000,
        max_power: 250,
        interface: InterfaceDescriptor {
            b_length: 9,
            b_descriptor_type: 4,
            b_interface_number: 0,
            b_alternate_setting: 0,
            b_num_endpoints: 2,
            b_interface_class: 3,
            b_interface_sub_class: 0,
            b_interface_protocol: 0,
            i_interface: 0,
            hid_device: HidDeviceDescriptor {
                b_length: 9,
                b_descriptor_type: 32,
                bcd_hid: 0x111,
                b_country_code: 0,
                b_num_descriptors: 1,
                b_descriptor_type_report: 33,
                w_descriptor_length: PS5_REPORT_DESCRIPTOR_SIZE,
                report: PS5_REPORT_DESCRIPTOR.as_ptr(),
            },
            endpoint_in: EndpointDescriptor {
                b_length: 7,
                b_descriptor_type: 5,
                b_endpoint_address: 0x84,
                bm_attributes: 3,
                w_max_packet_size: 64,
                b_interval: 6,
            },
            endpoint_out: EndpointDescriptor {
                b_length: 7,
                b_descriptor_type: 5,
                b_endpoint_address: 0x03,
                bm_attributes: 3,
                w_max_packet_size: 64,
                b_interval: 6,
            },
        },
    },
};

const PS5_REPORT_DESCRIPTOR_SIZE: u16 = 273;

const PS5_REPORT_DESCRIPTOR: [u8; PS5_REPORT_DESCRIPTOR_SIZE as usize] = [
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x05, // Usage (Game Pad)
    0xA1, 0x01, // Collection (Application)
    0x85, 0x01, //   Report ID (1)
    0x09, 0x30, //   Usage (X)
    0x09, 0x31, //   Usage (Y)
    0x09, 0x32, //   Usage (Z)
    0x09, 0x35, //   Usage (Rz)
    0x09, 0x33, //   Usage (Rx)
    0x09, 0x34, //   Usage (Ry)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x06, //   Report Count (6)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
    0x09, 0x20, //   Usage (0x20)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x01, //   Usage Page (Generic Desktop Ctrls)
    0x09, 0x39, //   Usage (Hat switch)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x07, //   Logical Maximum (7)
    0x35, 0x00, //   Physical Minimum (0)
    0x46, 0x3B, 0x01, //   Physical Maximum (315)
    0x65, 0x14, //   Unit (System: English Rotation, Length: Centimeter)
    0x75, 0x04, //   Report Size (4)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x42, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,Null State)
    0x65, 0x00, //   Unit (None)
    0x05, 0x09, //   Usage Page (Button)
    0x19, 0x01, //   Usage Minimum (0x01)
    0x29, 0x0F, //   Usage Maximum (0x0F)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x0F, //   Report Count (15)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
    0x09, 0x21, //   Usage (0x21)
    0x95, 0x0D, //   Report Count (13)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
    0x09, 0x22, //   Usage (0x22)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x34, //   Report Count (52)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x85, 0x02, //   Report ID (2)
    0x09, 0x23, //   Usage (0x23)
    0x95, 0x2F, //   Report Count (47)
    0x91, 0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x05, //   Report ID (5)
    0x09, 0x33, //   Usage (0x33)
    0x95, 0x28, //   Report Count (40)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x08, //   Report ID (8)
    0x09, 0x34, //   Usage (0x34)
    0x95, 0x2F, //   Report Count (47)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x09, //   Report ID (9)
    0x09, 0x24, //   Usage (0x24)
    0x95, 0x13, //   Report Count (19)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x0A, //   Report ID (10)
    0x09, 0x25, //   Usage (0x25)
    0x95, 0x1A, //   Report Count (26)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x20, //   Report ID (32)
    0x09, 0x26, //   Usage (0x26)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x21, //   Report ID (33)
    0x09, 0x27, //   Usage (0x27)
    0x95, 0x04, //   Report Count (4)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x22, //   Report ID (34)
    0x09, 0x40, //   Usage (0x40)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x80, //   Report ID (-128)
    0x09, 0x28, //   Usage (0x28)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x81, //   Report ID (-127)
    0x09, 0x29, //   Usage (0x29)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x82, //   Report ID (-126)
    0x09, 0x2A, //   Usage (0x2A)
    0x95, 0x09, //   Report Count (9)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x83, //   Report ID (-125)
    0x09, 0x2B, //   Usage (0x2B)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x84, //   Report ID (-124)
    0x09, 0x2C, //   Usage (0x2C)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x85, //   Report ID (-123)
    0x09, 0x2D, //   Usage (0x2D)
    0x95, 0x02, //   Report Count (2)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xA0, //   Report ID (-96)
    0x09, 0x2E, //   Usage (0x2E)
    0x95, 0x01, //   Report Count (1)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xE0, //   Report ID (-32)
    0x09, 0x2F, //   Usage (0x2F)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF0, //   Report ID (-16)
    0x09, 0x30, //   Usage (0x30)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF1, //   Report ID (-15)
    0x09, 0x31, //   Usage (0x31)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF2, //   Report ID (-14)
    0x09, 0x32, //   Usage (0x32)
    0x95, 0x0F, //   Report Count (15)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF4, //   Report ID (-12)
    0x09, 0x35, //   Usage (0x35)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF5, //   Report ID (-11)
    0x09, 0x36, //   Usage (0x36)
    0x95, 0x03, //   Report Count (3)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0,
];
