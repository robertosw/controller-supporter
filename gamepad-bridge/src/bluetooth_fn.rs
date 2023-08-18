use std::{
    io::{BufRead, BufReader},
    process::{exit, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

// --------- bluetooth handling ---------

// basic procedure:
//     1. turn on bluetooth
//     2. turn on scanning and read until e.g. "gamepad" is in name
//     3. stop scanning and connect via mac address

// this is a bit unconventional but easier to implement, the alternative would be to talk to linux' bluez directly on dbus

// bluetoothctl steps (commands)
//     power on
//     scan on
//     (connecting to already known devices is done automatically by bluez)
//
// Check if gamepads are already known and connected
// this has to be done each second, because gamepad might get connected after the programm starts
//     devices
//     paired-devices (run only if devices list is empty or no gamepads)
//     >>> if list has 1+ gamepads -> create thread for each to use its input
//
// At the same time read output of "scan on", if gamepad was found:
//     pairable on
//     connect <mac>   after finding a gamepad with "scan on" connect to it
//     trust <mac>     trust this gamepad, so that it will be connected automatically in the future
//     pairable off

/// turns bluetooth on
pub fn bt_power_on() {
    let output_power_on = match Command::new("bluetoothctl").args(["power", "on"]).output() {
        Ok(out) => out,
        Err(err) => {
            println!("unwrapping the output failed: {:?}", err);
            exit(1);
        }
    };

    let stdout = String::from_utf8(output_power_on.stdout).ok();
    let stderr = String::from_utf8(output_power_on.stderr).ok();

    if !output_power_on.status.success() {
        println!("bluetoothctl power on failed:");
        println!("{:?}", stderr);
        exit(1);
    }

    println!("Stdout: {:?}", stdout);
    println!("Stderr: {:?}", stderr);
}

/// calling `thread_handle.join()` on this thread terminates the  `bluetoothctl scan on` command
/// so the for loop inside this function is left and the function return smoothly
pub fn bt_scan_on_threaded() -> (Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
    let scan_output: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    // spawn new thread
    let scan_clone = scan_output.clone();
    let handle = thread::spawn(move || _bt_scan_on_thread(scan_clone));

    return (scan_output, handle);
}

/// Scan the given output of `bluetoothctl scan on` for yet unknown gamepads
pub fn handle_bt_scan_output(shared_mem_scan_output: &Arc<Mutex<Vec<String>>>) {
    let output_copy: Vec<String> = _move_from_shared_mem(shared_mem_scan_output);

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
            "NEW" => {
                if _is_device_controller(line_str) == true {
                    // TODO Connect to controller
                    return;
                }
            }
            "CHG" => (),
            "DEL" => (),
            _ => (),
        }
    }
}

/// After this function returns the device has been handled, so the loop can be continued
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

/// turn bluetoothctl scanning on and write output without buffering into file with param `output_file_name` <br>
/// If no error is occured, function never ends. Should be run in seperate thread
fn _bt_scan_on_thread(scan_output: Arc<Mutex<Vec<String>>>) {
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
                println!("Output  out: {}", line);
                {
                    let mut scan_output_locked = scan_output.lock().unwrap();
                    scan_output_locked.push(line);
                    // locks are released after a block goes out of sope
                }
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

    // This part is always reached as long as this thread is terminated by .join()

    // This always returns an error, so its currently useless
    // let scan_off = match Command::new("bluetoothctl").args(["scan", "off"]).output() {
    //     Ok(output) => output,
    //     Err(_) => return,
    // };
    // println!("{:?}", scan_off);
}

/// returns the current contents of the shared memory, clears shared memory in the process
fn _move_from_shared_mem(shared_memory: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    // always unwrap after calling lock.
    // If lock fails, this thread should panic because the other thread is in a deadlock
    // If the Mutex is locked by other thread, this one waits here until free
    let mut scan_output_locked = shared_memory.lock().unwrap();
    let copy: Vec<String> = scan_output_locked.clone();
    scan_output_locked.clear();

    return copy;

    // locks are released after a block goes out of sope
}
