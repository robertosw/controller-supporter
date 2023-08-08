use std::path::PathBuf;

use crate::structs::*;

/// Uses system tools to find controllers in the list of connected devices and adds newly found controllers until the collection is full.
pub fn get_and_insert_controllers(ctrls: &mut GameControllerCollection) {
    let new_ctrls: Vec<GameControllerSimple> = get_controllers();

    // Check if results of find_controllers contain new controllers
    for new_ctrl in new_ctrls {
        match ctrls.len() {
            2 => (), // two controllers already connected, nothing else to do
            1 => {
                // If new_ctrl is not already in controllers, add it
                match ctrls.contains(new_ctrl.clone()) {
                    false => ctrls.add(new_ctrl.clone()),
                    true => (),
                }
            }
            0 => ctrls.add(new_ctrl.clone()),
            _ => (), // do nothing
        };
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
            // Device is no controller or the mac address is not readable
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
