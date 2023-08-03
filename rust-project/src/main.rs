//  build standalone binary for raspi:
// Tier 1 (guaranteed to work):     cargo build --target aarch64-unkown-linux-gnu           added to project
// Tier 2 (guaranteed to build):    cargo build --target armv7-unknown-linux-gnueabihf      added to project
// Tier 2                           cargo build --target armv7-unknown-linux-gnueabihf      not yet added to project, has linux kernel 4.15, instead of the v3.2 the option above has
// https://doc.rust-lang.org/nightly/rustc/platform-support.html
// Raspi Zero 2W has a 64-bit Arm Cortex-A53, which is ARMv8 64bit.. so the Tier 1 option should work

// Steps:
//  1. Connect Controller
//      - that is in pairing mode
//      - scan for controllers already known to the system
//  2. Read Controller Input
//      2.1 Find Controller(s) in list of devices
//      2.2 Read inputs (maybe two controllers can be multithreaded)
//      - linux command to compare results: evtest (non-root!)

use std::{path::PathBuf, thread, time::Duration};

fn main() {
    println!("Start");
    let mut controllers: Vec<(PathBuf, String)> = Vec::new();

    loop {
        controllers = find_controllers();
        thread::sleep(Duration::from_secs(5));
    }
}

/// - Returns all found controllers, might be 0
/// - Information as Tuple:
///     0. PathBuf: Path in /dev/input
///     1. String: MAC address with : already in between pairs)
fn find_controllers() -> Vec<(PathBuf, String)> {
    // this was the original from their example.. but the collect is unnecessary??
    // let devices = evdev::enumerate().map(|tuple| tuple.1).collect::<Vec<_>>();
    // purpose of map: results of enumerate are the
    // tuple (PathBuf, Device), of this tuple, take only the Device

    // Get all devices the user has access to
    println!("Searching for controllers");
    let devices = evdev::enumerate();

    // What to search for in the list of devices, that could be a controller
    let known_device_names = [String::from("Wireless Controller")];

    // (Path in /dev/input/, MAC address of controller)
    let mut usable_controllers: Vec<(PathBuf, String)> = Vec::new();

    // Find which devices could be game controllers
    for device in devices {
        let device_info: evdev::Device = device.1;
        let device_path: PathBuf = device.0;

        let device_name: String = match device_info.name() {
            None => String::from(""),
            Some(name) => String::from(name),
        };
        let device_mac: String = match device_info.unique_name() {
            None => String::from(""),
            Some(mac) => String::from(mac),
        };

        if (!known_device_names.contains(&device_name)) || (device_mac.len() != 17) {
            println!("Device is no controller or mac is not readable");
            continue;
        }

        println!("Controller '{device_name}' ({device_mac}) detected");

        let controller_tuple: (PathBuf, String) = (device_path, device_mac);
        usable_controllers.push(controller_tuple);
    }

    // TODO Check if there are multiple controllers with the same mac and only choose one of them,
    // so the second slot is available for a different controller

    let controllers = usable_controllers.clone();
    return controllers;

    // match usable_controllers.len() {
    //     2.. => {
    //         return [
    //             Some(usable_controllers[0].clone()),
    //             Some(usable_controllers[1].clone()),
    //         ]
    //     }
    //     1 => return [Some(usable_controllers[0].clone()), None],
    //     0 => return [None, None],
    //     _ => unreachable!(),
    // };
}
