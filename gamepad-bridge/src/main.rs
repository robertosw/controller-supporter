#![allow(dead_code)]

use ctrlc::set_handler;
use hidapi::HidApi;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use hidg::{Class, Device, Key, Keyboard, Led};

mod bluetooth_fn;
mod hidapi_fn;
mod hidapi_read_ps5_usb;
mod hidapi_structs;
mod usb_gadget;

use crate::bluetooth_fn::*;
use crate::hidapi_fn::*;
use crate::usb_gadget::*;

// libusb   udeavadm monitor       minicom

fn main() {
    println!("\nGamepad-Bridge started: v0.4.3\n");

    configure_as_gadget("raspi", "abcdef12345", "Generic Manufacturer", "My Product", "My Config", 0, 0, 64);

    let mut device = Device::<Keyboard>::open("/dev/hidg0").unwrap();

    // Create input report
    let mut input = Keyboard.input();

    // Press left ctrl modifier
    input.press_key(Key::LeftCtrl);

    // Press key 'A'
    input.press_key(Key::A);

    // Send input report
    device.input(&input).unwrap();

    // Get pressed keys
    println!("Keys: {:?}", input.pressed().collect::<Vec<Key>>());

    // Release left ctrl modifier
    input.release_key(Key::LeftCtrl);

    // Release key 'A'
    input.release_key(Key::A);

    // Send input report
    device.input(&input).unwrap();

    // Create output report
    let mut output = Keyboard.output();

    // Receive output report
    device.output(&mut output).unwrap();

    // Print lit LEDs
    println!("LEDs: {:?}", output.lit().collect::<Vec<Led>>());

    // hier könnte man vielleicht einfach den usbbus von linux nehmen, anscheinend bietet die rusb crate das

    // oder ansonsten kann man vielleicht einfach eins der /dev/tty als device annehmen...

    // Ideas for program flow
    // 1. the whole procedure (BT finding, input read, output to usb) is being duplicated for each player right inside main. So 1-4 threads
    //     Problem: two threads could use the same gamepad and think its their own.. so output duplication
    // 2. the bluetooth scanning is one thread, seperate from main (output written in file or shared mem)
    //     - output is interpreted inside main thread
    //     after an active device is connected, only then is a thread spawned for this device
    //     -> threads dont have to know from each others existence (maybe for usb output, but we'll see)

    // Create a shared boolean flag to indicate if Ctrl+C was pressed
    let ctrlc = Arc::new(AtomicBool::new(true));
    let ctrlc_clone = ctrlc.clone();

    // Set the flag to false when Ctrl+C is pressed
    match set_handler(move || ctrlc_clone.store(false, Ordering::SeqCst)) {
        Ok(_) => (),
        Err(err) => {
            println!("Error setting Ctrl-C handler {:?}", err);
            exit(1);
        }
    };

    bt_power_on();
    let (shared_mem_scan_output, thread_handle) = bt_scan_on_threaded();

    // find new controllers
    // loop while ctrlc has not been pressed (.load == true)
    while ctrlc.load(Ordering::SeqCst) {
        handle_bt_scan_output(&shared_mem_scan_output);

        thread::sleep(Duration::from_millis(500));
    }

    thread_handle.join().unwrap();
}

fn hidapi_starter() {
    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            println!("Error getting HidApi access: {:?}", err);
            exit(2);
        }
    };

    let _gamepads: Vec<hidapi::DeviceInfo> = find_supported_gamepads(api);
}
