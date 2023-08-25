use crate::usb_descr::*;

const PS5_REPORT_DESCRIPTOR_SIZE: u16 = 273;

pub const PS5_DEVICE_STRINGS: UsbDeviceStrings = UsbDeviceStrings {
    manufacturer: "Sony Interactive Entertainment",
    product: "Wireless Controller",
    serialnumber: "",
};

pub const PS5_GAMEPAD: UsbDeviceDescriptor = UsbDeviceDescriptor {
    bcd_usb: 0x200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 64,
    id_vendor: 0x054c,
    id_product: 0x0ce6,
    bcd_device: 0x100,
    struct_configuration: UsbConfigurationDescriptor {
        bm_attributes: 0b11000000,
        max_power: 250,
        struct_interface: UsbInterfaceDescriptor {
            b_descriptor_type: 4,
            b_interface_number: 0,
            b_alternate_setting: 0,
            b_num_endpoints: 2,
            b_interface_class: 3,
            b_interface_sub_class: 0,
            b_interface_protocol: 0,
            i_interface: 0,
            struct_hid_device: UsbHidDeviceDescriptor {
                b_descriptor_type: 32,
                bcd_hid: 0x111,
                b_country_code: 0,
                b_num_descriptors: 1,
                b_descriptor_type_report: 33,
                w_descriptor_length: PS5_REPORT_DESCRIPTOR_SIZE,
            },
            struct_endpoint_in: UsbEndpointDescriptor {
                b_descriptor_type: 5,
                b_endpoint_address: 0x84,
                bm_attributes: 3,
                w_max_packet_size: 64,
                b_interval: 6,
            },
            struct_endpoint_out: UsbEndpointDescriptor {
                b_descriptor_type: 5,
                b_endpoint_address: 0x03,
                bm_attributes: 3,
                w_max_packet_size: 64,
                b_interval: 6,
            },
        },
    },
};

pub const PS5_REPORT_DESCRIPTOR: [u8; PS5_REPORT_DESCRIPTOR_SIZE as usize] = [
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x05, // Usage (Game Pad)
    0xA1, 0x01, // Collection (Application)
    0x85, 0x01, //   Report ID (1)
    0x09, 0x30, //   Usage (X)                          Describes actual position (xyz coords in world)
    0x09, 0x31, //   Usage (Y)                          Describes actual position (xyz coords in world)
    0x09, 0x32, //   Usage (Z)                          Describes actual position (xyz coords in world)
    0x09, 0x35, //   Usage (Rz)                         Describes actual rotation xyz
    0x09, 0x33, //   Usage (Rx)                         Describes actual rotation xyz
    0x09, 0x34, //   Usage (Ry)                         Describes actual rotation xyz
    0x15, 0x00, //   Logical Minimum (0)                All of these coords can range from 0 to 255 (both inclusive)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)        All of these coords can range from 0 to 255 (both inclusive)
    0x75, 0x08, //   Report Size (8)                    [0, 255] is represented by 8 bit
    0x95, 0x06, //   Report Count (6)                   = 6x8bit
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    //                               ^ Rel instead of Abs would mean they represent the change since last step
    //
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
    0x09, 0x20, //   Usage (0x20)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    //
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
    //
    0x05, 0x09, //   Usage Page (Button)
    0x19, 0x01, //   Usage Minimum (0x01)               First Button is ID 1
    0x29, 0x0F, //   Usage Maximum (0x0F)               Last Button is ID 15
    0x15, 0x00, //   Logical Minimum (0)                Each Button can send 0
    0x25, 0x01, //   Logical Maximum (1)                or 1
    0x75, 0x01, //   Report Size (1)                    sending 1/0 needs 1 bit
    0x95, 0x0F, //   Report Count (15)                  Confirms that there are 15 Buttons
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)       Information about what these Buttons are (HID spec Sec. 6.2.2.5)
    //
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
    0x09, 0x21, //   Usage (0x21)
    0x95, 0x0D, //   Report Count (13)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    //
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
    0x09, 0x22, //   Usage (0x22)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x34, //   Report Count (52)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x85, 0x02, //   Report ID (2)
    //
    // This might be the rumble???
    0x09, 0x23, //   Usage (0x23)
    0x95, 0x2F, //   Report Count (47)
    0x91, 0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x05, //   Report ID (5)
    //
    // The following are Feature Reports, probably for the LEDs
    0x09, 0x33, //   Usage (0x33)
    0x95, 0x28, //   Report Count (40)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x08, //   Report ID (8)
    //
    0x09, 0x34, //   Usage (0x34)
    0x95, 0x2F, //   Report Count (47)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x09, //   Report ID (9)
    //
    0x09, 0x24, //   Usage (0x24)
    0x95, 0x13, //   Report Count (19)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x0A, //   Report ID (10)
    //
    0x09, 0x25, //   Usage (0x25)
    0x95, 0x1A, //   Report Count (26)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x20, //   Report ID (32)
    //
    0x09, 0x26, //   Usage (0x26)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x21, //   Report ID (33)
    //
    0x09, 0x27, //   Usage (0x27)
    0x95, 0x04, //   Report Count (4)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x22, //   Report ID (34)
    //
    0x09, 0x40, //   Usage (0x40)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x80, //   Report ID (-128)
    //
    0x09, 0x28, //   Usage (0x28)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x81, //   Report ID (-127)
    //
    0x09, 0x29, //   Usage (0x29)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x82, //   Report ID (-126)
    //
    0x09, 0x2A, //   Usage (0x2A)
    0x95, 0x09, //   Report Count (9)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x83, //   Report ID (-125)
    //
    0x09, 0x2B, //   Usage (0x2B)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x84, //   Report ID (-124)
    //
    0x09, 0x2C, //   Usage (0x2C)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0x85, //   Report ID (-123)
    //
    0x09, 0x2D, //   Usage (0x2D)
    0x95, 0x02, //   Report Count (2)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xA0, //   Report ID (-96)
    //
    0x09, 0x2E, //   Usage (0x2E)
    0x95, 0x01, //   Report Count (1)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xE0, //   Report ID (-32)
    //
    0x09, 0x2F, //   Usage (0x2F)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF0, //   Report ID (-16)
    //
    0x09, 0x30, //   Usage (0x30)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF1, //   Report ID (-15)
    //
    0x09, 0x31, //   Usage (0x31)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF2, //   Report ID (-14)
    //
    0x09, 0x32, //   Usage (0x32)
    0x95, 0x0F, //   Report Count (15)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF4, //   Report ID (-12)
    //
    0x09, 0x35, //   Usage (0x35)
    0x95, 0x3F, //   Report Count (63)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x85, 0xF5, //   Report ID (-11)
    //
    0x09, 0x36, //   Usage (0x36)
    0x95, 0x03, //   Report Count (3)
    0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    //
    0xC0, //   End of Collection with Report ID 1
];
