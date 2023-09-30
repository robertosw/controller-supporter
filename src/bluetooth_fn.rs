use std::io::BufRead;
use std::io::BufReader;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

use crate::helper_fn::run_cmd;
use crate::print_and_exit;

// this is a bit unconventional but easier to implement.
// the alternative would be to talk to linux' bluez directly on dbus

/// Steps:
/// - Power on
/// - Discoverable on
/// - Pairable on
/// - Wait 1 second to give any known devices a chance to connect
///
/// 1. check with `devices Connected` if any devices are already connected to the RPi
///     1. If list is empty, continue
///     2. If the list is not empty:
///         - Check if any of the mac addresses are known to be supported gamepads (mac addresses from a file)
///         - If so, check with get_hid_gamepad() if any of the devices are usable as gamepads
///           Stop if at first supported gamepad
///
/// 2. check with `devices Paired` if the RPi was already connected to any devices
///     1. If list is empty, continue
///     2. If the list is not empty:
///         1. Check if list contains any known mac addresses, for each known:
///             - Connect (and trust)
///             - see if get_hid_gamepad() finds any supported gamepads
///             - If not, disconnect and remove mac address from list
///         2. If any devices are left, find all which could be gamepads using _is_device_controller()
///             - Connect (and trust)
///             - see if get_hid_gamepad() finds any supported gamepads
///                 - If not, disconnect
///                 - If so, add to mac address list
///
/// 3. scan on
///     - track all devices like this: (mac address, name, still online? [bool], is gamepad [unknown, no, yes] )
///       and update that list with every event
///     - For each device in that list, that has `is gamepad: unknown`:
///         - if it has no name: ignore
///         - if it has a name: check if name is known as a gamepad
///             - if so, connect and trust
///             - check with get_hid_gamepad() if gamepad is supported
///             - if so, add to mac address list
///             - stop scan and return
///
/// - discoverable off
/// - pairable off

pub fn wait_for_bt_device() {
    match run_cmd("/", "bluetoothctl power on") {
        Ok(_) => {}
        Err(_) => print_and_exit!("bluetoothctl power on failed", 1),
    }
    match run_cmd("/", "bluetoothctl discoverable on") {
        Ok(_) => {}
        Err(_) => print_and_exit!("bluetoothctl discoverable on failed", 1),
    }
    match run_cmd("/", "bluetoothctl pairable on") {
        Ok(_) => {}
        Err(_) => print_and_exit!("bluetoothctl pairable on failed", 1),
    }

    _bt_scan_on();
    return;
}

// fn power_on() {
//     let output_power_on = match Command::new("bluetoothctl").args(["power", "on"]).output() {
//         Ok(out) => out,
//         Err(err) => {
//             println!("unwrapping the output failed: {:?}", err);
//             exit(1);
//         }
//     };
//
//     let stdout = String::from_utf8(output_power_on.stdout).ok();
//     let stderr = String::from_utf8(output_power_on.stderr).ok();
//
//     if !output_power_on.status.success() {
//         println!("bluetoothctl power on failed:");
//         println!("{:?}", stderr);
//         exit(1);
//     }
//
//     println!("Stdout: {:?}", stdout);
//     println!("Stderr: {:?}", stderr);
// }

fn _bt_scan_on() {
    // start scanning for devices
    let child_cmd = match Command::new("stdbuf")
        .args(["-o0", "bluetoothctl", "scan", "on"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            println!("creating terminal command went wrong. Exiting.\nError: {err}");
            exit(1);
        }
    };

    let stdout = BufReader::new(child_cmd.stdout.expect("Failed to capture stdout"));
    let stderr = BufReader::new(child_cmd.stderr.expect("Failed to capture stderr"));

    // UNLESS bluetoothctl fails at some point in time, this for loop is never left,
    // because bluetoothctl scan on never ends sucessfully on its own
    for line in stdout.lines() {
        match line {
            Ok(line) => {
                _handle_bt_scan_output(&line);
            }
            Err(err) => println!("Error out: {}", err),
        }
    }

    // so this loop is only entered if the command ends
    for line in stderr.lines() {
        match line {
            Ok(line) => println!("Output err: {}", line),
            Err(err) => println!("Error err: {}", err),
        }
    }

    // Its apparently unneccecary to call "bluetoothctl scan off",
    // this seems to be done correctly if the scan on command gets terminated
}

/// Returns `false` if the output line does not suggest that a gamepad has been found
pub fn _handle_bt_scan_output(bt_scan_output_line: &String) -> bool {
    if bt_scan_output_line.contains("Discovery started") {
        // First line of this command can be ignored
        return false;
    }

    // Find out which Event type this line is
    // Possible options are below in match
    let first_asci_upper: usize = match bt_scan_output_line.find(|c: char| c.is_ascii_uppercase()) {
        None => return false,
        Some(usize) => usize,
    };
    let line_str = bt_scan_output_line.as_str();
    let log_type = &line_str[first_asci_upper..first_asci_upper + 3];
    println!("Type: {:?}", log_type);

    match log_type {
        "NEW" => {
            _is_device_controller(&bt_scan_output_line);
            return false; // TODO
        }
        "CHG" => false,
        "DEL" => false,
        _ => false,
    }
}

/// Check if a given output line represents a gamepad / game controller
///
/// Arguments:
/// - `output_line: &str` = One output line of the command `bluetoothctl scan on`
fn _is_device_controller(output_line: &str) -> bool {
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
    let index_next_whitespace: usize = match output_line.find(|c: char| c.is_whitespace()) {
        None => return false,
        Some(usize) => usize,
    };
    let line_str = &output_line[index_next_whitespace + 1..];

    // get the descriptor and cut it off
    let (descriptor, line_str) = match line_str.split_once(char::is_whitespace) {
        Some((extracted, remainder)) => (extracted, remainder),
        None => return false,
    };
    println!("descriptor: {:?}", &descriptor);
    if descriptor != "Device" {
        println!("");
        return false;
    }

    // get mac address and device name
    let (mac_address, device_name) = match line_str.split_once(char::is_whitespace) {
        Some((extracted, remainder)) => (extracted, remainder),
        None => return false,
    };
    println!("mac: {:?}", &mac_address);
    println!("device_name: {:?}", &device_name);
    println!("");

    if device_name.contains(" controller") || device_name.contains(" Controller") {
        return true;
    }

    return false;
}
