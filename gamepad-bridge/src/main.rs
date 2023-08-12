#![allow(unused_imports, dead_code)]

use hidapi::HidApi;
use std::process::exit;
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
    println!("\n Gamepad-Bridge started");

    // Ideas for program flow
    // 1. the whole procedure (BT finding, input read, output to usb) is being duplicated for each player right inside main. So 1-4 threads
    //     Problem: two threads could use the same gamepad and think its their own.. so output duplication
    // 2. the bluetooth scanning is one thread, seperate from main (output written in file or shared mem)
    //     - output is interpreted inside main thread
    //     after an active device is connected, only then is a thread spawned for this device
    //     -> threads dont have to know from each others existence (maybe for usb output, but we'll see)

    bt_power_on();

    // bt scanning
    let scan_output: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let request_scan_stop: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    // spawn new thread
    let scan_clone = scan_output.clone();
    let request_scan_stop_clone = request_scan_stop.clone();
    let _handle = thread::spawn(move || bt_scan_on(scan_clone, request_scan_stop_clone));

    // use scanning data
    loop {
        let mut _output_clone: Vec<String> = Vec::new();
        
        // copy data from shared memory
        {
            // always unwrap after calling lock.
            // If lock fails, this thread should panic because the other thread is in a deadlock
            let mut scan_output_locked = scan_output.lock().unwrap();
            _output_clone = scan_output_locked.clone();
            scan_output_locked.clear();

            // locks are released after a block goes out of sope
        }
        
        // check if anything new was added and do something with it
        for line in _output_clone {
            println!("Output in main: {:?}", line);
        }

        thread::sleep(Duration::from_millis(500));
    }

    _handle.join().unwrap(); // dont exit main without waiting for the thread to end
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
