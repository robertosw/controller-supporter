use crate::usb_gadget::*;

pub const PS5_GAMEPAD: UsbGadgetDescriptor = UsbGadgetDescriptor {
    bcd_usb: 0x200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 64,
    id_vendor: 0x054c,
    id_product: 0x0ce6,
    bcd_device: 0x100,
    strings_0x409: UsbGadgetStrings {
        manufacturer: "Sony Interactive Entertainment",
        product: "Wireless Controller",
        serialnumber: "",
    },
    configs_c1: UsbGadgetConfigs {
        bm_attributes: 0b11000000,
        max_power: 500,
        configs_string: "",
    },
    functions_hid: UsbGadgetFunctionsHid {
        hid_subclass: 0,
        protocol: 0,
        report_length: 64,
        report_descriptor: &[
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
            0x81,
            0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)       Information about what these Buttons are (HID spec Sec. 6.2.2.5)
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
        ],
    },
};

pub const PS4_GAMEPAD: UsbGadgetDescriptor = UsbGadgetDescriptor {
    bcd_usb: 0x200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 64,
    id_vendor: 0x054c,
    id_product: 0x09cc,
    bcd_device: 0x100,
    strings_0x409: UsbGadgetStrings {
        serialnumber: "",
        product: "Wireless Controller",
        manufacturer: "Sony Interactive Entertainment",
    },
    configs_c1: UsbGadgetConfigs {
        bm_attributes: 0b11000000,
        max_power: 500,
        configs_string: "",
    },
    functions_hid: UsbGadgetFunctionsHid {
        protocol: 0,
        report_descriptor: &[
            0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
            0x09, 0x05, // Usage (Game Pad)
            0xA1, 0x01, // Collection (Application)
            0x85, 0x01, //   Report ID (1)
            0x09, 0x30, //   Usage (X)
            0x09, 0x31, //   Usage (Y)
            0x09, 0x32, //   Usage (Z)
            0x09, 0x35, //   Usage (Rz)
            0x15, 0x00, //   Logical Minimum (0)
            0x26, 0xFF, 0x00, //   Logical Maximum (255)
            0x75, 0x08, //   Report Size (8)
            0x95, 0x04, //   Report Count (4)
            0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
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
            0x29, 0x0E, //   Usage Maximum (0x0E)
            0x15, 0x00, //   Logical Minimum (0)
            0x25, 0x01, //   Logical Maximum (1)
            0x75, 0x01, //   Report Size (1)
            0x95, 0x0E, //   Report Count (14)
            0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
            0x09, 0x20, //   Usage (0x20)
            0x75, 0x06, //   Report Size (6)
            0x95, 0x01, //   Report Count (1)
            0x15, 0x00, //   Logical Minimum (0)
            0x25, 0x7F, //   Logical Maximum (127)
            0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0x05, 0x01, //   Usage Page (Generic Desktop Ctrls)
            0x09, 0x33, //   Usage (Rx)
            0x09, 0x34, //   Usage (Ry)
            0x15, 0x00, //   Logical Minimum (0)
            0x26, 0xFF, 0x00, //   Logical Maximum (255)
            0x75, 0x08, //   Report Size (8)
            0x95, 0x02, //   Report Count (2)
            0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined 0xFF00)
            0x09, 0x21, //   Usage (0x21)
            0x95, 0x36, //   Report Count (54)
            0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
            0x85, 0x05, //   Report ID (5)
            0x09, 0x22, //   Usage (0x22)
            0x95, 0x1F, //   Report Count (31)
            0x91, 0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x04, //   Report ID (4)
            0x09, 0x23, //   Usage (0x23)
            0x95, 0x24, //   Report Count (36)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x02, //   Report ID (2)
            0x09, 0x24, //   Usage (0x24)
            0x95, 0x24, //   Report Count (36)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x08, //   Report ID (8)
            0x09, 0x25, //   Usage (0x25)
            0x95, 0x03, //   Report Count (3)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x10, //   Report ID (16)
            0x09, 0x26, //   Usage (0x26)
            0x95, 0x04, //   Report Count (4)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x11, //   Report ID (17)
            0x09, 0x27, //   Usage (0x27)
            0x95, 0x02, //   Report Count (2)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x12, //   Report ID (18)
            0x06, 0x02, 0xFF, //   Usage Page (Vendor Defined 0xFF02)
            0x09, 0x21, //   Usage (0x21)
            0x95, 0x0F, //   Report Count (15)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x13, //   Report ID (19)
            0x09, 0x22, //   Usage (0x22)
            0x95, 0x16, //   Report Count (22)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x14, //   Report ID (20)
            0x06, 0x05, 0xFF, //   Usage Page (Vendor Defined 0xFF05)
            0x09, 0x20, //   Usage (0x20)
            0x95, 0x10, //   Report Count (16)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x15, //   Report ID (21)
            0x09, 0x21, //   Usage (0x21)
            0x95, 0x2C, //   Report Count (44)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x06, 0x80, 0xFF, //   Usage Page (Vendor Defined 0xFF80)
            0x85, 0x80, //   Report ID (-128)
            0x09, 0x20, //   Usage (0x20)
            0x95, 0x06, //   Report Count (6)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x81, //   Report ID (-127)
            0x09, 0x21, //   Usage (0x21)
            0x95, 0x06, //   Report Count (6)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x82, //   Report ID (-126)
            0x09, 0x22, //   Usage (0x22)
            0x95, 0x05, //   Report Count (5)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x83, //   Report ID (-125)
            0x09, 0x23, //   Usage (0x23)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x84, //   Report ID (-124)
            0x09, 0x24, //   Usage (0x24)
            0x95, 0x04, //   Report Count (4)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x85, //   Report ID (-123)
            0x09, 0x25, //   Usage (0x25)
            0x95, 0x06, //   Report Count (6)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x86, //   Report ID (-122)
            0x09, 0x26, //   Usage (0x26)
            0x95, 0x06, //   Report Count (6)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x87, //   Report ID (-121)
            0x09, 0x27, //   Usage (0x27)
            0x95, 0x23, //   Report Count (35)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x88, //   Report ID (-120)
            0x09, 0x28, //   Usage (0x28)
            0x95, 0x22, //   Report Count (34)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x89, //   Report ID (-119)
            0x09, 0x29, //   Usage (0x29)
            0x95, 0x02, //   Report Count (2)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x90, //   Report ID (-112)
            0x09, 0x30, //   Usage (0x30)
            0x95, 0x05, //   Report Count (5)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x91, //   Report ID (-111)
            0x09, 0x31, //   Usage (0x31)
            0x95, 0x03, //   Report Count (3)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x92, //   Report ID (-110)
            0x09, 0x32, //   Usage (0x32)
            0x95, 0x03, //   Report Count (3)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0x93, //   Report ID (-109)
            0x09, 0x33, //   Usage (0x33)
            0x95, 0x0C, //   Report Count (12)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA0, //   Report ID (-96)
            0x09, 0x40, //   Usage (0x40)
            0x95, 0x06, //   Report Count (6)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA1, //   Report ID (-95)
            0x09, 0x41, //   Usage (0x41)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA2, //   Report ID (-94)
            0x09, 0x42, //   Usage (0x42)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA3, //   Report ID (-93)
            0x09, 0x43, //   Usage (0x43)
            0x95, 0x30, //   Report Count (48)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA4, //   Report ID (-92)
            0x09, 0x44, //   Usage (0x44)
            0x95, 0x0D, //   Report Count (13)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA5, //   Report ID (-91)
            0x09, 0x45, //   Usage (0x45)
            0x95, 0x15, //   Report Count (21)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA6, //   Report ID (-90)
            0x09, 0x46, //   Usage (0x46)
            0x95, 0x15, //   Report Count (21)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xF0, //   Report ID (-16)
            0x09, 0x47, //   Usage (0x47)
            0x95, 0x3F, //   Report Count (63)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xF1, //   Report ID (-15)
            0x09, 0x48, //   Usage (0x48)
            0x95, 0x3F, //   Report Count (63)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xF2, //   Report ID (-14)
            0x09, 0x49, //   Usage (0x49)
            0x95, 0x0F, //   Report Count (15)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA7, //   Report ID (-89)
            0x09, 0x4A, //   Usage (0x4A)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA8, //   Report ID (-88)
            0x09, 0x4B, //   Usage (0x4B)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xA9, //   Report ID (-87)
            0x09, 0x4C, //   Usage (0x4C)
            0x95, 0x08, //   Report Count (8)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xAA, //   Report ID (-86)
            0x09, 0x4E, //   Usage (0x4E)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xAB, //   Report ID (-85)
            0x09, 0x4F, //   Usage (0x4F)
            0x95, 0x39, //   Report Count (57)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xAC, //   Report ID (-84)
            0x09, 0x50, //   Usage (0x50)
            0x95, 0x39, //   Report Count (57)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xAD, //   Report ID (-83)
            0x09, 0x51, //   Usage (0x51)
            0x95, 0x0B, //   Report Count (11)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xAE, //   Report ID (-82)
            0x09, 0x52, //   Usage (0x52)
            0x95, 0x01, //   Report Count (1)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xAF, //   Report ID (-81)
            0x09, 0x53, //   Usage (0x53)
            0x95, 0x02, //   Report Count (2)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xB0, //   Report ID (-80)
            0x09, 0x54, //   Usage (0x54)
            0x95, 0x3F, //   Report Count (63)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xB1, //   Report ID (-79)
            0x09, 0x55, //   Usage (0x55)
            0x95, 0x02, //   Report Count (2)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xB2, //   Report ID (-78)
            0x09, 0x56, //   Usage (0x56)
            0x95, 0x02, //   Report Count (2)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xE0, //   Report ID (-32)
            0x09, 0x57, //   Usage (0x57)
            0x95, 0x02, //   Report Count (2)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xB3, //   Report ID (-77)
            0x09, 0x55, //   Usage (0x55)
            0x95, 0x3F, //   Report Count (63)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0x85, 0xB4, //   Report ID (-76)
            0x09, 0x55, //   Usage (0x55)
            0x95, 0x3F, //   Report Count (63)
            0xB1, 0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
            0xC0, // End Collection

            // 507 bytes
        ],
        report_length: 64,
        hid_subclass: 0,
    },
};

pub const GENERIC_KEYBOARD: UsbGadgetDescriptor = UsbGadgetDescriptor {
    bcd_usb: 0x0200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 64,
    id_vendor: 0x1d6b,
    id_product: 0x0104,
    bcd_device: 0x0100,
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

pub const MULTIFUNCTION_COMPOSITE: UsbGadgetDescriptor = UsbGadgetDescriptor {
    bcd_usb: 0x0200,
    b_device_class: 0,
    b_device_sub_class: 0,
    b_device_protocol: 0,
    b_max_packet_size0: 64,
    id_vendor: 0x1d6b,
    id_product: 0x0104,
    bcd_device: 0x0100,
    strings_0x409: UsbGadgetStrings {
        manufacturer: "Manufacturer",
        product: "USB Gadget with multiple functions",
        serialnumber: "",
    },
    configs_c1: UsbGadgetConfigs {
        bm_attributes: 0b11000000,
        max_power: 500,
        configs_string: "",
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
