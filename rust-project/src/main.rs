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

use psutil::process::Process;
use std::{
    path::PathBuf,
    process,
    process::Command,
    thread::{self, JoinHandle},
    time::Duration,
};

struct GameControllerSimple {
    path: PathBuf,
    mac: String,
}
struct GameController {
    path: PathBuf,
    mac: String,
    thread_handle: Option<JoinHandle<()>>,
}

fn main() {
    let mut loading_show: String = String::from("..");
    let process = Process::new(process::id()).unwrap();
    let mut controllers: Vec<GameController> = Vec::new();

    loop {
        let new_controllers: Vec<GameControllerSimple> = get_controllers();

        // Check if results of find_controllers contain new controllers
        for new_ctrl in new_controllers {
            match controllers.len() {
                2 => {
                    // Override the oldest (first connected) controller
                    if (controllers[0].mac != new_ctrl.mac) && (controllers[1].mac != new_ctrl.mac)
                    {
                        controllers[0].path = new_ctrl.path;
                        controllers[0].mac = new_ctrl.mac;
                    }
                }
                1 => {
                    // If new_ctrl is not already in controllers, add it
                    if controllers[0].mac != new_ctrl.mac {
                        controllers.push(GameController {
                            path: new_ctrl.path,
                            mac: new_ctrl.mac,
                            thread_handle: None,
                        });
                    }
                }
                0 => controllers.push(GameController {
                    path: new_ctrl.path,
                    mac: new_ctrl.mac,
                    thread_handle: None,
                }),
                _ => (),
            };
        }

        output_info(&mut loading_show, &process, &controllers);

        handle_threads(&mut controllers);

        // wait some time before checking for new devices
        thread::sleep(Duration::from_secs(2));
    }
}

/// - Checks the thread handle to see if all controllers have running threads
/// - Creates threads if necessary
/// - Takes care that no controller is assigned two or more threads
fn handle_threads(controllers: &mut Vec<GameController>) {
    for controller in controllers {
        // Create new thread if allowed
        match controller.thread_handle {
            Some(_) => (),
            None => {
                // Copy Controller for the new thread
                let threaded_controller: GameControllerSimple = GameControllerSimple {
                    path: controller.path.clone(),
                    mac: controller.mac.clone(),
                };

                // Create thread
                controller.thread_handle = Some(thread::spawn(move || {
                    read_controller_input(threaded_controller);
                }));
            }
        };
    }
}

fn read_controller_input(_controller: GameControllerSimple) {
    println!("fake input read :D");
}

/// Output Information to Terminal <br>
/// - Show that programm is active with little animation
/// - Show RAM usage
/// - Show connected controllers by their mac address
fn output_info(loading_show: &mut String, process: &Process, controllers: &Vec<GameController>) {
    let _ = Command::new("clear").status();
    println!("Searching for controllers{loading_show}");
    match loading_show.len() < 7 {
        true => loading_show.push('.'),
        false => *loading_show = String::from(".."),
    }
    let memory_usage = process.memory_info().unwrap().rss() as f64 / (1024.0 * 1024.0);
    // Memory in MB
    println!("Memory usage: {:.2} MB", memory_usage);
    println!("");

    for controller in controllers {
        // Show in terminal what is connected
        println!("{:?} connected", controller.mac.clone());
    }
}

/// Returns all found controllers, might be 0
fn get_controllers() -> Vec<GameControllerSimple> {
    // this was the original from their example.. but the collect is unnecessary??
    // let devices = evdev::enumerate().map(|tuple| tuple.1).collect::<Vec<_>>();
    // purpose of map: results of enumerate are the
    // tuple (PathBuf, Device), of this tuple, take only the Device

    // Get all devices the user has access to
    let devices = evdev::enumerate();

    // What to search for in the list of devices, that could be a controller
    let known_device_names = [String::from("Wireless Controller")];

    // (Path in /dev/input/, MAC address of controller)
    let mut usable_controllers: Vec<GameControllerSimple> = Vec::new();

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

        let gamecontroller: GameControllerSimple = GameControllerSimple {
            path: device_path,
            mac: device_mac,
        };
        usable_controllers.push(gamecontroller);
    }

    // TODO Check if there are multiple controllers with the same mac and only choose one of them,
    // so the second slot is available for a different controller

    return usable_controllers;
}
