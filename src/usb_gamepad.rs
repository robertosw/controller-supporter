use std::{
    fs::File,
    io::Write,
    process::exit,
    sync::{Arc, Mutex, mpsc::{TryRecvError, Receiver}},
    thread,
    time::{Duration, Instant},
};

use crate::{print_error_and_exit, universal_gamepad::UniversalGamepad, usb_gadget::UsbGadgetDescriptor};

pub struct Gamepad {
    pub gadget: UsbGadgetDescriptor,

    /// This depends on how the function bt_input_to_universal_gamepad() works
    pub min_bt_report_size: usize,
    pub bt_input_to_universal_gamepad: fn(&Vec<u8>) -> UniversalGamepad,
    pub universal_gamepad_to_usb_output: fn(&UniversalGamepad) -> Vec<u8>,
}
impl Gamepad {
    pub fn bt_input_to_universal_gamepad(&self, bt_input: &Vec<u8>) -> UniversalGamepad {
        return (self.bt_input_to_universal_gamepad)(&bt_input);
    }

    /// - Transforms the given `UniversalGamepad` into the correct output array for this `Gamepad`
    /// - Attempts to write the entire output array into the file /dev/hidg0
    pub fn write_to_gadget_continously(&self, universal_gamepad: Arc<Mutex<UniversalGamepad>>) -> ! {
        loop {
            let usb_output: Vec<u8> = {
                let gamepad_locked = universal_gamepad.lock().expect("Locking Arc<Mutex<UniversalGamepad>> failed!");
                self._universal_gamepad_to_usb_output(&gamepad_locked)
            };

            let mut hidg0 = match File::options().write(true).append(false).open("/dev/hidg0") {
                Ok(file) => file,
                Err(err) => print_error_and_exit!("Could not open file hidg0", err, 1),
            };

            match hidg0.write_all(&usb_output) {
                Ok(_) => (),
                Err(err) => println!("write to hidg0 failed: {:?}", err),
            }

            thread::sleep(Duration::from_micros(500));
        }
    }

    /// To adjust how precice the interval has to be "hit", `max_deviation` can be used.
    ///
    /// - `max_deviation`:
    ///     - using `1` would mean, that the interval is practically disregarded and writing to file is done as soon as possible
    ///     - using `0` would nearly never write to file, because this loop itself has a bit of runtime and never hits the interval with 0ns accuracy
    ///     - useful values are
    ///         - `~ 0.05`, if its important that writing is done nearly every interval
    ///         - `< 0.001`, if its important that the interval is precicely hit
    pub fn write_to_gadget_intervalic(&self, universal_gamepad: Arc<Mutex<UniversalGamepad>>, interval: Duration, max_deviation: f32, recv: Receiver<bool>) {
        // its safe to use u128 for nanoseconds
        // 2^64 ns are ~580 years
        // so 2^128 are 580² years

        let start: Instant = Instant::now();
        let interval_ns = interval.as_nanos();
        let mut interval_counts_before: u128 = 0;

        let mut code_ran: bool = false;
        let mut usb_output: Vec<u8> = Vec::new();

        loop {
            match recv.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break;
                }
                Err(TryRecvError::Empty) => println!(":"),
            }
            {
                if code_ran == false {
                    // program code that might not run fast enough for interval here
                    let gamepad_locked = universal_gamepad.lock().expect("Locking Arc<Mutex<UniversalGamepad>> failed!");
                    usb_output = self._universal_gamepad_to_usb_output(&gamepad_locked);
                }
                code_ran = true;
            }

            // Is this loop still "on time"?
            let now: Instant = Instant::now();
            let elapsed_ns: u128 = (now - start).as_nanos();
            let interval_counts_now: u128 = elapsed_ns / interval_ns;

            // By how many ns does the current run deviate from the cycle
            let diff_from_interval_ns: u128 = elapsed_ns % interval_ns;

            let is_next_interval: bool = interval_counts_now > interval_counts_before;
            let is_close_enough: bool = (diff_from_interval_ns as f32 / interval_ns as f32) <= max_deviation;

            if is_next_interval && is_close_enough {
                {
                    // code that is supposed to be timed, herek
                    let mut hidg0 = match File::options().write(true).append(false).open("/dev/hidg0") {
                        Ok(file) => file,
                        Err(err) => print_error_and_exit!("Could not open file hidg0", err, 1),
                    };

                    match hidg0.write_all(&usb_output) {
                        Ok(_) => (),
                        Err(err) => println!("write to hidg0 failed: {:?}", err),
                    }
                }

                interval_counts_before = interval_counts_now;
                code_ran = false;
                usb_output.clear();
            }
        }
    }

    pub fn debug_output_bt_input(&self, gamepad: &UniversalGamepad) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        println!(
            "Lx:{:5?}\tLy:{:5?}\tL: {:5?}\tRx:{:5?}\tRy:{:5?}\tR: {:5?}",
            gamepad.sticks.left.x,
            gamepad.sticks.left.y,
            gamepad.sticks.left.pressed,
            gamepad.sticks.right.x,
            gamepad.sticks.right.y,
            gamepad.sticks.right.pressed,
        );

        print!("{}", termion::cursor::Goto(1, 2));
        println!(
            "Tl:{:5?}\tTr:{:5?}\tBl:{:?}\tBr:{:?}",
            gamepad.triggers.left, gamepad.triggers.right, gamepad.buttons.bumpers.left, gamepad.buttons.bumpers.right,
        );

        print!("{}", termion::cursor::Goto(1, 3));
        println!(
            "X: {:5?}\tO: {:5?}\t□: {:5?}\t∆: {:5?}",
            gamepad.buttons.main.lower, gamepad.buttons.main.right, gamepad.buttons.main.left, gamepad.buttons.main.upper
        );

        print!("{}", termion::cursor::Goto(1, 4));
        println!(
            "↑: {:5?}\t→: {:5?}\t↓: {:5?}\t←: {:5?}",
            gamepad.buttons.dpad.up, gamepad.buttons.dpad.right, gamepad.buttons.dpad.down, gamepad.buttons.dpad.left
        );

        print!("{}", termion::cursor::Goto(1, 5));
        println!(
            "S: {:5?}\tM: {:5?}\tLogo: {:5?}",
            gamepad.buttons.specials.left, gamepad.buttons.specials.right, gamepad.buttons.specials.logo
        );
    }

    /// creates a `Vec<u8>` that is the HID Report which has to be written in `/dev/hidg0`
    ///
    /// The length will be asserted at runtime to be `self.gadget.functions_hid.report_length`. This function will **panic** if the length is not correct
    fn _universal_gamepad_to_usb_output(&self, gamepad: &UniversalGamepad) -> Vec<u8> {
        return (self.universal_gamepad_to_usb_output)(gamepad);
    }
}
