#![allow(dead_code)]

#[macro_use]
extern crate version;
// To allow using the version! macro

extern crate termion;

use flume::bounded;
use flume::unbounded;
use flume::Receiver;
use flume::Sender;
use hidapi::HidApi;
use std::env;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use usb_gadget::UsbGadgetDescriptor;

mod bluetooth_fn;
mod helper_fn;
mod hidapi_fn;
mod universal_gamepad;
mod usb_gadget;
mod usb_gamepad;
mod usb_gamepad_keyboard;
mod usb_gamepad_ps4;
mod usb_gamepad_ps5;

use crate::bluetooth_fn::*;
use crate::universal_gamepad::UniversalGamepad;
use crate::usb_gamepad::Gamepad;
use crate::usb_gamepad_ps4::DUALSHOCK;
use crate::usb_gamepad_ps5::DUALSENSE;

//  if working inside a docker container: (started with the docker-compose from project root)
//  - build and run (inside container)  `cargo run`
//
//  if working on native os as non root: (from /gamepad-bridge)
//  - build & run   `cargo build --release && sudo chown root:root target/release/gamepad-bridge && sudo chmod +s target/release/gamepad-bridge && /target/release/gamepad-bridge`

// for benchmarking in tests use: cargo test -- --show-output

fn main() {
    println!("\nGamepad-Bridge started: v{:}", version!());
    println!("This program needs to be run as root user. Please set uuid accordingly.\n");

    wait_for_bt_device();
    exit(0);

    // ----- Enable Gadget
    // If this is done at a later point, the host might run into errors when trying to classify this device and turn it off
    let output_gamepad: &Gamepad = Gamepad::from_cmdline_args();
    output_gamepad.gadget.configure_device();
    println!("Gadget enabled");

    // ----- Create all channels
    // These are used to tell the reading and writing threads to finish (they are normally infinite loops)
    let (sender_ctrlc, recv_ctrlc) = mpsc::channel();
    let (sender_exit_request, recv_exit_request): (Sender<()>, Receiver<()>) = bounded(1);
    let (sender_gamepad, recv_gamepad): (Sender<UniversalGamepad>, Receiver<UniversalGamepad>) = unbounded();

    // ----- Setup CTRL+C handler
    ctrlc::set_handler(move || sender_ctrlc.send(()).expect("Could not send signal on channel.")).expect("Error setting Ctrl-C handler");

    // ----- BT connection here
    wait_for_bt_device();

    // ----- What gamepad is connected?
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => print_error_and_exit!("Error getting HidApi access", err, 2),
    };

    let (device, input_gamepad): (hidapi::HidDevice, &Gamepad) = match hidapi_fn::get_hid_gamepad(&api) {
        Ok((device, model)) => match model {
            hidapi_fn::SupportedInputGamepads::Ps5DualSense => (device, &DUALSENSE),
            hidapi_fn::SupportedInputGamepads::PS4DualShock => (device, &DUALSHOCK),
        },
        Err(err) => print_error_and_exit!("Error accessing connected hid gamepad", err, 1),
    };

    println!("Gamepad connected");

    // ----- Reading input of BT gamepad
    let thread_handle_input = thread::Builder::new()
        .name("input".to_string())
        .spawn(move || hidapi_fn::read_bt_gamepad_input(device, input_gamepad, sender_gamepad, recv_exit_request))
        .expect("creating input thread failed");
    println!("Input thread running");

    // TODO Maybe remove this later, but currently the output-writing step is reached so fast that /dev/hidg0 is not yet ready.
    // This just prevents some of the "Cannot send after transport endpoint shutdown" errors because of this ^
    thread::sleep(Duration::from_secs(1));

    // ----- Write Output to gadget
    let thread_handle_output = thread::Builder::new()
        .name("output".to_string())
        .spawn(move || output_gamepad.write_to_gadget_continously(recv_gamepad))
        .expect("creating output thread failed");
    println!("Output thread running");
    println!("");

    // ----- Clean up (if Ctrl + C is pressed)

    // This is blocking
    match recv_ctrlc.recv() {
        Ok(_) => println!(""),
        Err(e) => print_error_and_exit!("Receiving from CTRL C channel failed:", e, 1),
    }

    println!("Waiting for input and output threads to finish");
    sender_exit_request.send(()).expect("sending to input thread failed");
    thread_handle_input.join().unwrap();
    thread_handle_output.join().unwrap();

    // clean_up_device() removes hidg0 file, so this has to run after write output thread is closed
    println!("Disabling gadget");
    output_gamepad.gadget.clean_up_device();

    println!("Everything is cleaned up :)");
}

// for benchmarking in tests use: cargo test -- --show-output

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usb_gamepad::OUTPUT_GAMEPADS;
    use flume::{unbounded, Sender};
    use std::time::Instant;

    pub const RUNS: u32 = 50000;

    /// on my machine this takes 1.5µs
    #[test]
    fn bench1_all_gamepads_bt_input_to_gamepad() {
        println!("");
        println!("Benchmark BT input -> UniversalGamepad output");
        println!("{} runs per gamepad", RUNS);

        for gamepad in OUTPUT_GAMEPADS {
            // Skip unfinished gamepads
            if gamepad.is_supported == false {
                println!("{} skipped, not supported", gamepad.display_name);
                continue;
            }

            // prepare fake input
            let bt_input: Vec<u8> = vec![0; gamepad.min_bt_report_size];

            // prepare benchmark value
            let mut counter: u32 = 0;
            let mut times: Duration = Duration::from_secs(0);

            while counter < RUNS {
                let before = Instant::now();

                // It might be better not to use "let _ =" because this never assignes the output
                // and could result in faster but unrealistic runtime
                let _universal_gamepad = gamepad.bt_input_to_universal_gamepad(&bt_input);

                let diff = Instant::now() - before;
                times += diff;
                counter += 1;
            }
            let avg = times / RUNS;
            println!("{} took: {:4.2?}", gamepad.display_name, avg);
        }
    }

    /// on my machine this takes ~1.5µs
    #[test]
    fn bench2_all_gamepads_gamepad_to_usb() {
        println!("");
        println!("Benchmark UniversalGamepad input -> Usb gadget output");
        println!("{} runs per gamepad", RUNS);

        for gamepad in OUTPUT_GAMEPADS {
            // Skip unfinished gamepads
            if gamepad.is_supported == false {
                println!("{} skipped, not supported", gamepad.display_name);
                continue;
            }

            // prepare fake input
            let universal_gamepad = UniversalGamepad::nothing_pressed();

            // prepare benchmark value
            let mut counter: u32 = 0;
            let mut times: Duration = Duration::from_secs(0);

            while counter < RUNS {
                let before = Instant::now();

                // It might be better not to use "let _ =" because this never assignes the output
                // and could result in faster but unrealistic runtime
                let _usb_output = gamepad.universal_gamepad_to_usb_output(&universal_gamepad);

                let diff = Instant::now() - before;
                times += diff;
                counter += 1;
            }
            let avg = times / RUNS;
            println!("{} took: {:4.2?}", gamepad.display_name, avg);
        }
    }

    /// on my machine this takes 10µs
    #[test]
    fn bench3_all_gamepads_with_channels() {
        println!("");
        println!("Benchmark BT Input to UniversalGamepad - channel - to usb gadget output");
        println!("{} runs per gamepad", RUNS);

        for gamepad in OUTPUT_GAMEPADS {
            // Skip unfinished gamepads
            if gamepad.is_supported == false {
                println!("{} skipped, not supported", gamepad.display_name);
                continue;
            }

            let (sender_gamepad, recv_gamepad): (Sender<(UniversalGamepad, Instant)>, Receiver<(UniversalGamepad, Instant)>) = unbounded();

            let thread_handle_input = thread::Builder::new()
                .name("input".to_string())
                .spawn(move || _bench3_input_thread(sender_gamepad, &gamepad))
                .expect("creating input thread failed");

            let thread_handle_output = thread::Builder::new()
                .name("output".to_string())
                .spawn(move || _bench3_output_thread(recv_gamepad, &gamepad))
                .expect("creating input thread failed");

            thread_handle_input.join().unwrap();
            match thread_handle_output.join() {
                Ok(avg) => println!("{} took: {:4.2?}", gamepad.display_name, avg),
                Err(_) => println!("error unwrapping output handle"),
            }
        }
    }

    fn _bench3_input_thread(sender: Sender<(UniversalGamepad, Instant)>, gamepad: &Gamepad) {
        // prepare fake input
        let bt_input: Vec<u8> = vec![0; gamepad.min_bt_report_size];

        let mut counter: u32 = 0;

        while counter < RUNS {
            let start = Instant::now();

            let universal_gamepad = gamepad.bt_input_to_universal_gamepad(&bt_input);
            match sender.send((universal_gamepad, start)) {
                Ok(_) => {}
                Err(err) => println!("Error sending gamepad to output thread: {err}"),
            };

            counter += 1;
            thread::sleep(Duration::from_micros(10));
        }
    }

    fn _bench3_output_thread(receiver: Receiver<(UniversalGamepad, Instant)>, gamepad: &Gamepad) -> Duration {
        let mut duration_sum: Duration = Duration::from_secs(0);

        for (universal_gamepad, start) in receiver.iter() {
            let _usb_out = gamepad.universal_gamepad_to_usb_output(&universal_gamepad);

            let end = Instant::now();
            let diff = end - start;
            duration_sum += diff;
        }

        let avg = duration_sum / RUNS;
        return avg;
    }
}
