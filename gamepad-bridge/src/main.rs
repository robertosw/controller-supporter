#![allow(unused_imports, dead_code)]

use ctrlc::set_handler;
use hidapi::HidApi;
use regex::Regex;
use std::ops::ControlFlow;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

mod bluetooth_fn;
mod hidapi_fn;
mod hidapi_read_ps5_usb;
mod hidapi_structs;

use crate::bluetooth_fn::*;
use crate::hidapi_fn::*;

fn main() {
    println!("\nGamepad-Bridge started: v0.3\n");

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

    // scanning in new thread
    let (shared_mem_scan_output, thread_handle) = bt_scan_on_threaded();

    // find new controllers
    // loop while ctrlc has not been pressed (.load == true)
    while ctrlc.load(Ordering::SeqCst) {
        let output_copy: Vec<String> = move_from_shared_mem(&shared_mem_scan_output);

        // check if anything new was added and do something with it
        for line in output_copy {
            let line_str: &str = line.as_str();

            if line.contains("Discovery started") {
                // First line of this command can be ignored
                continue;
            }

            let first_asci_upper: usize = match line.find(|c: char| c.is_ascii_uppercase()) {
                None => continue,
                Some(usize) => usize,
            };
            let log_type = &line_str[first_asci_upper..first_asci_upper + 3];
            println!("Type: {:?}", log_type);

            match log_type {
                "NEW" => check_if_device_is_controller(line_str),
                "CHG" => (),
                "DEL" => (),
                _ => (),
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    thread_handle.join().unwrap();
}

/// After this function returns the device has been handled, so the loop can be continued
fn check_if_device_is_controller(line_str: &str) {
    // Possible outputs
    // [NEW] Device 54:C2:8B:53:A4:3C 54-C2-8B-53-A4-3C         --> irrelevant
    // [NEW] Device 54:C2:8B:53:A4:3C Name of Device            --> THIS is interesting
    // [CHG] Controller 14:F6:D8:7D:51:94 Discovering: yes      --> everything that starts with Controller can be discharged
    // [CHG] Device 6E:FF:68:D4:4D:CC RSSI: -92                 --> irrelevant
    // [CHG] Device 6E:FF:68:D4:4D:CC TxPower: 17               --> irrelevant
    // [DEL]

    // the contents of [] are a bit messed up because of the colors:
    // [\u{1}\u{1b}[0;93m\u{2}CHG\u{1}\u{1b}[0m\u{2}]
    // [\u{1}\u{1b}[0;92m\u{2}NEW\u{1}\u{1b}[0m\u{2}]

    // Cut off the log type
    let index_next_whitespace: usize = match line_str.find(|c: char| c.is_whitespace()) {
        None => return,
        Some(usize) => usize,
    };
    let line_str = &line_str[index_next_whitespace + 1..];

    // get the descriptor and cut it off
    let (descriptor, line_str) = match line_str.split_once(char::is_whitespace) {
        Some((extracted, remainder)) => (extracted, remainder),
        None => return,
    };
    println!("descriptor: {:?}", &descriptor);
    if descriptor != "Device" {
        println!("");
        return;
    }

    // get mac address and device name
    let (mac_address, device_name) = match line_str.split_once(char::is_whitespace) {
        Some((extracted, remainder)) => (extracted, remainder),
        None => return,
    };
    println!("mac: {:?}", &mac_address);
    println!("device_name: {:?}", &device_name);
    if device_name.contains(" controller") || device_name.contains(" Controller") {
        // TODO Connect to controller
    }

    println!("");
    return;
}

/// returns the current contents of the shared memory, clears shared memory in the process
fn move_from_shared_mem(shared_memory: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    // always unwrap after calling lock.
    // If lock fails, this thread should panic because the other thread is in a deadlock
    // If the Mutex is locked by other thread, this one waits here until free
    let mut scan_output_locked = shared_memory.lock().unwrap();
    let copy: Vec<String> = scan_output_locked.clone();
    scan_output_locked.clear();

    return copy;

    // locks are released after a block goes out of sope
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
