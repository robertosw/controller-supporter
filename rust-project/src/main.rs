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

#[derive(Clone)]
struct GameControllerSimple {
    path: PathBuf,
    mac: String,
}

struct GameController {
    path: PathBuf,
    mac: String,
    thread_handle: Option<JoinHandle<()>>,
}

struct GameControllerCollection {
    first: Option<GameController>,
    second: Option<GameController>,
}
impl GameControllerCollection {
    /// How many controllers are being used?
    fn len(&self) -> u8 {
        let mut _count: u8 = 0;

        match &self.first {
            None => (),
            Some(_) => _count += 1,
        };
        match &self.second {
            None => (),
            Some(_) => _count += 1,
        };

        return _count;
    }

    /// Is this Controller already known?
    fn contains(&self, new_ctrl: GameControllerSimple) -> bool {
        let mut _first_contains: bool;
        let mut _second_contains: bool;

        match &self.first {
            None => _first_contains = false,
            Some(ctrl) => {
                if ctrl.mac == new_ctrl.mac {
                    _first_contains = true;
                }
                _first_contains = false;
            }
        };
        match &self.second {
            None => _second_contains = false,
            Some(ctrl) => {
                if ctrl.mac == new_ctrl.mac {
                    _second_contains = true;
                }
                _second_contains = false;
            }
        };

        return _first_contains || _second_contains;
    }

    /// WARNING: If the collection is already full, nothing will be changed <br>
    /// Adds the given controller to the collection in the top most place
    fn add(&mut self, new_ctrl: GameControllerSimple) {
        match &self.first {
            None => {
                self.first = Some(GameController {
                    path: new_ctrl.path,
                    mac: new_ctrl.mac,
                    thread_handle: None,
                });
                return;
            }
            Some(_) => (),
        };

        match &self.second {
            None => {
                self.second = Some(GameController {
                    path: new_ctrl.path,
                    mac: new_ctrl.mac,
                    thread_handle: None,
                });
                return;
            }
            Some(_) => (),
        };
    }
}

// TODO: The same controller is in both slots currently
// TODO: Add a "count" variable to the collection which is updated in add/remove/len
// TODO: the collection needs a remove method
// TODO: Move struct and impl into seperate file

fn main() {
    let mut loading_show: String = String::from("..");
    let process = Process::new(process::id()).unwrap();
    let mut ctrls: GameControllerCollection = GameControllerCollection {
        first: None,
        second: None,
    };
    let mut ctrl_count: u8 = 0;
    // let mut ctrls: [Option<GameController>; 2] = [None, None];

    loop {
        if ctrl_count < 2 {
            let new_ctrls: Vec<GameControllerSimple> = get_controllers();

            // Check if results of find_controllers contain new controllers
            for new_ctrl in new_ctrls {
                match ctrls.len() {
                    2 => ctrl_count = 2, // two controllers already connected, nothing else to do
                    1 => {
                        ctrl_count = 1;

                        // If new_ctrl is not already in controllers, add it
                        match ctrls.contains(new_ctrl.clone()) {
                            false => ctrls.add(new_ctrl.clone()),
                            true => (),
                        }
                    }
                    0 => {
                        ctrl_count = 0;
                        ctrls.add(new_ctrl.clone());
                    }
                    _ => (), // do nothing
                };
            }
        }

        output_info(&mut loading_show, &process, &ctrls, ctrl_count.clone());
        handle_threads(&mut ctrls);

        // wait some time before checking for new devices
        thread::sleep(Duration::from_secs(2));
    }
}

fn read_controller_input(_controller: GameControllerSimple) {
    println!("fake input read :D");
}

/// - Checks the thread handle to see if all controllers have running threads
/// - Creates threads if necessary
/// - Takes care that no controller is assigned two or more threads
fn handle_threads(ctrls: &mut GameControllerCollection) {
    match &mut ctrls.first {
        None => (),
        Some(ctrl) => _create_new_thread(ctrl),
    }
    match &mut ctrls.second {
        None => (),
        Some(ctrl) => _create_new_thread(ctrl),
    }

    fn _create_new_thread(controller: &mut GameController) {
        match controller.thread_handle {
            None => {
                // Copy Controller for the new thread
                let threaded_controller: GameControllerSimple = GameControllerSimple {
                    path: controller.path.clone(),
                    mac: controller.mac.clone(),
                };

                // Create thread and link it in collection
                controller.thread_handle = Some(thread::spawn(move || {
                    read_controller_input(threaded_controller);
                }));
            }
            Some(_) => (),
        };
    }
}

/// Output Information to Terminal <br>
/// - Show that programm is active with little animation
/// - Show RAM usage
/// - Show connected controllers by their mac address
fn output_info(
    loading_show: &mut String,
    process: &Process,
    ctrls: &GameControllerCollection,
    ctrl_count: u8,
) {
    let _ = Command::new("clear").status();

    match (loading_show.len() < 7) && (ctrl_count < 2) {
        true => {
            println!("Searching for controllers{loading_show}");
            loading_show.push('.');
        }
        false => println!("Scanning stopped, disconnect one controller to restart scan"),
    }

    let memory_usage = process.memory_info().unwrap().rss() as f64 / (1024.0 * 1024.0);
    // Memory in MB
    println!("Memory usage: {:.2} MB", memory_usage);
    println!("");

    // Show in terminal what is connected
    match &ctrls.first {
        None => (),
        Some(ctrl) => {
            println!("{:?} connected", ctrl.mac.clone());
        }
    }
    match &ctrls.second {
        None => (),
        Some(ctrl) => {
            println!("{:?} connected", ctrl.mac.clone());
        }
    }
}

/// Returns all found controllers, might be 0
fn get_controllers() -> Vec<GameControllerSimple> {
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
