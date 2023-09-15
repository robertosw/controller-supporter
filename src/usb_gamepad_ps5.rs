use crate::universal_gamepad::*;
use crate::usb_gadget::*;
use crate::usb_gamepad::Gamepad;
use crate::UsbGadgetDescriptor;

pub const DUALSENSE: Gamepad = Gamepad {
    gadget: UsbGadgetDescriptor {
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
    },
    min_bt_report_size: 12,
    bt_input_to_universal_gamepad: _bt_input_to_universal_gamepad,
    universal_gamepad_to_usb_output: _universal_gamepad_to_usb_output,
};

fn _bt_input_to_universal_gamepad(bt_input: &Vec<u8>) -> UniversalGamepad {
    let mut output: UniversalGamepad = UniversalGamepad::nothing_pressed();
    let dpad_byte = 0b00001111 & bt_input[9];

    output.sticks = Sticks {
        left: Stick {
            x: bt_input[2],
            y: bt_input[3],
            pressed: match bt_input[10] {
                64 => true,
                _ => false,
            },
        },
        right: Stick {
            x: bt_input[4],
            y: bt_input[5],
            pressed: match bt_input[10] {
                128 => true,
                _ => false,
            },
        },
    };
    output.triggers = Triggers {
        left: bt_input[6],
        right: bt_input[7],
    };
    output.buttons.bumpers = Bumpers {
        left: match bt_input[10] {
            1 => true,
            _ => false,
        },
        right: match bt_input[10] {
            2 => true,
            _ => false,
        },
    };
    output.buttons.main = MainButtons {
        upper: (bt_input[9] & 0b10000000 != 0),
        right: (bt_input[9] & 0b01000000 != 0),
        lower: (bt_input[9] & 0b00100000 != 0),
        left: (bt_input[9] & 0b00010000 != 0),
    };
    output.buttons.dpad = DPad {
        right: (dpad_byte == 1 || dpad_byte == 2 || dpad_byte == 3),
        down: (dpad_byte == 3 || dpad_byte == 4 || dpad_byte == 5),
        left: (dpad_byte == 5 || dpad_byte == 6 || dpad_byte == 7),
        up: (dpad_byte == 0 || dpad_byte == 1 || dpad_byte == 7),
    };
    output.buttons.specials = SpecialButtons {
        right: match bt_input[10] {
            32 => true,
            _ => false,
        },
        left: match bt_input[10] {
            16 => true,
            _ => false,
        },
        logo: match bt_input[11] {
            1 => true,
            _ => false,
        },
    };
    output.other.touchpad = Some(Touchpad {
        x_coord: 0,
        y_coord: 0,
        touched: false,
        pressed: match bt_input[11] {
            2 => true,
            _ => false,
        },
    });

    DUALSENSE.debug_output_bt_input(&output);

    return output;

    // maybe bytes 35 and 36 together are left-right

    // print!("{}", termion::cursor::Goto(1, 1));

    // let combined_u16: u16 = (input[35] as u16) << 8 | (input[36] as u16);

    // adjust which bytes should be visible. For PS Gamepads the first two bytes are just counters
    // print!("{:05}\t", combined_u16);

    // TODO Touchpad Support
    // when Byte 34 changes, the touchpad state changed (either now touched or now not touched)
    //  also this counts up each time the state changes
    // Touchpad Y Axis is byte 37
    // Touchpad X Axis is strange, (byte 35 or 36) probably consists of multiple bytes
    //   if only touched, the value is somewhat correct (0 is left, 255 is right)
    //   if you drag the finger across, this value overflows 4x on the whole way (l->r)
}

fn _universal_gamepad_to_usb_output(gamepad: &UniversalGamepad) -> Vec<u8> {
    let out: Vec<u8> = vec![
        0x01,
        gamepad.sticks.left.x,
        gamepad.sticks.left.y,
        gamepad.sticks.right.x,
        gamepad.sticks.right.y,
        gamepad.triggers.left,
        gamepad.triggers.right,
        0,    // counter
        0,    // Buttons and DPad
        0,    // Special Buttons, Bumpers, Triggers and Sticks (only WHAT is pressed, for triggers not value)
        0,    // Logo / Touchpad
        0,    // always 0
        0,    // counter
        0,    // seconds ?
        0xee, // might be charging state (in %) (unlikely, changes drastically after reconnect)
        0xad, // ??
        0x00, // gyroskop here (seems to be relative, not absolute)
        0x00, // gyroskop here (seems to be relative, not absolute)
        0xff, // gyroskop here (seems to be relative, not absolute)
        0xff, // gyroskop here (seems to be relative, not absolute)
        0x02, // gyroskop here (seems to be relative, not absolute)
        0x00, // gyroskop here (seems to be relative, not absolute)
        0x06, // gyroskop here (seems to be relative, not absolute)
        0x00, // gyroskop here (seems to be relative, not absolute)
        0x81, // gyroskop here (seems to be relative, not absolute)
        0x1f, // gyroskop here (seems to be relative, not absolute)
        0x07, // gyroskop here (seems to be relative, not absolute)
        0x06, // gyroskop here (seems to be relative, not absolute)
        0x46, // gyroskop here (seems to be relative, not absolute)
        0x66, // gyroskop here (seems to be relative, not absolute)
        0,    // this is a really slow counter (goes up every ~10s)
        0x00, // ??
        0x14, // ??
        0x80, // ??
        0x00, // ??
        0x00, // ??
        0x00, // ??
        0x80, // ??
        0x00, // ??
        0x00, // ??
        0x00, // ??
        0x00, // ??
        0x09, // ??
        0x09, // ??
        0x00, // ??
        0x00, // ??
        0x00, // ??
        0x00, // ??
        0x00, // ??
        0xe3, // random?
        0x79, // random?
        0xab, // random?
        0x00, // slow counter
        0x17, // constant?
        0x08, // constant?
        0x00, // constant?
        0x5b, // random?
        0x7f, // random?
        0xef, // random?
        0x9c, // random?
        0xac, // random?
        0x03, // random?
        0x92, // random?
        0x30, // random?
    ];

    let expected_length = DUALSENSE.gadget.functions_hid.report_length as usize;
    let vec_length = out.len();
    assert!(
        expected_length == vec_length,
        "The given vector for the usb gadget output is not the correct length. Expected: {expected_length}  Actual: {vec_length}"
    );

    return out;
}
