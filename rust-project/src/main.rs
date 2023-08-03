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
//      - linux command to compare results: evtest (non-root!)

use std::path::PathBuf;

fn main() {
    println!("Start");
    read_input();
}

fn read_input() {
    // this was the original from their example.. but the collect is unnecessary??
    // let devices = evdev::enumerate().map(|tuple| tuple.1).collect::<Vec<_>>();
    // purpose of map: results of enumerate are the
    // tuple (PathBuf, Device), of this tuple, take only the Device

    // Get all devices the user has access to
    let devices = evdev::enumerate();

    // Find which devices could be game controllers
    for device in devices {
        let device_info: evdev::Device = device.1;
        let device_path: PathBuf = device.0;
        println!("device name - {:?}", device_info.name());
        println!("device path - {:?}", device_path);
    }
    // let selected_device = Device::open("/dev/input/event16");

    // let mut args = std::env::args_os();
    // args.next();
    // if let Some(dev_file) = args.next() {
    //     evdev::Device::open(dev_file).unwrap()
    // } else {
    //     let mut devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    //     // readdir returns them in reverse order from their eventN names for some reason
    //     devices.reverse();
    //     for (i, d) in devices.iter().enumerate() {
    //         println!("{}: {}", i, d.name().unwrap_or("Unnamed device"));
    //     }
    //     print!("Select the device [0-{}]: ", devices.len());
    //     let _ = std::io::stdout().flush();
    //     let mut chosen = String::new();
    //     std::io::stdin().read_line(&mut chosen).unwrap();
    //     let n = chosen.trim().parse::<usize>().unwrap();
    //     devices.into_iter().nth(n).unwrap()
    // }
}
