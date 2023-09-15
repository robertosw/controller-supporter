use std::{fs::File, io::Write, process::exit, thread, time::Duration};

use crate::{universal_gamepad::UniversalGamepad, usb_gadget::UsbGadgetDescriptor};

pub struct Gamepad {
    pub gadget: UsbGadgetDescriptor,
    pub bt_input_to_universal_gamepad: fn(&Vec<u8>) -> UniversalGamepad,
    pub universal_gamepad_to_usb_output: fn(&UniversalGamepad) -> Vec<u8>,
}
impl Gamepad {
    /// Attempts to write the entire `usb_output` into the file /dev/hidg0
    pub fn write_to_gadget_once(&self, usb_output: Vec<u8>) {
        let mut hidg0 = match File::options().write(true).append(false).open("/dev/hidg0") {
            Ok(file) => file,
            Err(err) => {
                println!("Could not open file hidg0 {err}");
                exit(1);
            }
        };

        match hidg0.write_all(&usb_output) {
            Ok(_) => (),
            Err(err) => println!("write to hidg0 failed: {:?}", err),
        }
    }

    pub fn bt_input_to_universal_gamepad(&self, bt_input: &Vec<u8>) -> UniversalGamepad {
        return (self.bt_input_to_universal_gamepad)(&bt_input);
    }

    /// creates a `Vec<u8>` that is the HID Report which has to be written in `/dev/hidg0`
    ///
    /// The length will be asserted at runtime to be `self.gadget.functions_hid.report_length`. This function will **panic** if the length is not correct
    pub fn universal_gamepad_to_usb_output(&self, gamepad: &UniversalGamepad) -> Vec<u8> {
        return (self.universal_gamepad_to_usb_output)(gamepad);
    }

    /// Moves all triggers and joysticks and presses and releases all buttons
    pub fn write_dummy_data_continously(&self) -> ! {
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

            let usb_output = self.universal_gamepad_to_usb_output(&gamepad);
            self.write_to_gadget_once(usb_output);

            // TODO achieve a real timed interval
            thread::sleep(Duration::from_millis(4));
        }
    }
}
