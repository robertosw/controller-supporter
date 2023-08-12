use std::{process::{exit, Command, Stdio}, io::{BufReader, SeekFrom, Seek, BufRead, Write}, fs::OpenOptions};

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

/// turn bluetoothctl scanning on and write output without buffering into file with param `output_file_name` <br>
/// If no error is occured, function never ends. Should be run in seperate thread
pub fn bt_scan_on(output_file_name: String) {
    // TODO: Even if this command fails, this function should never return so that the thread will always scan

    let _status_del_file = Command::new("rm")
        .arg(output_file_name.clone())
        .status()
        .unwrap();

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

    let mut file = OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(output_file_name)
        .unwrap();

    let _fp_pos_from_start = file.seek(SeekFrom::End(0)).unwrap();

    // UNLESS bluetoothctl fails at some point in time, this for loop is never left,
    // because bluetoothctl scan on never ends sucessfully on its own
    for line in stdout.lines() {
        match line {
            Ok(mut line) => {
                println!("Output out: {}", line);

                line.push('\n');
                let _written_bytes = file.write_all(line.as_bytes()).unwrap();
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
}


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
