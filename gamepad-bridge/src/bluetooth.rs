use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    os::fd::AsFd,
    process::{exit, Command, ExitStatus, Stdio},
    thread,
    time::Duration,
};

/// I tried out the bluez/bluer crate but I never really found out how to connect to devices correctly
/// The crate is way to over-complicated for what I am trying to do, so I'll stick to the terminal

pub fn bluetooth() {
    // basic procedure:
    //     1. turn on bluetooth
    //     2. turn on scanning and read until e.g. "controller" is in name
    //     3. stop scanning and connect via mac address

    power_on();

    let _ = scan_on();
}

fn scan_on() -> std::io::Result<()> {
    // Start the command with its output captured
    // stdbuf -o0 bluetoothctl scan on > output.txt
    let child_cmd = Command::new("stdbuf")
        .args(["-o0", "bluetoothctl", "scan", "on"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    // Get the stdout stream of the child process
    let stdout = child_cmd.stdout.expect("Failed to capture stdout");
    let stderr = child_cmd.stderr.expect("Failed to capture stderr");

    // Create a buffered reader for the stdout stream
    let reader_out = BufReader::new(stdout);
    let reader_err = BufReader::new(stderr);

    // Read the output line by line
    for line in reader_out.lines() {
        match line {
            Ok(line) => println!("Output out: {}", line),
            Err(err) => println!("Error out: {}", err),
        }
    }
    for line in reader_err.lines() {
        match line {
            Ok(line) => println!("Output err: {}", line),
            Err(err) => println!("Error err: {}", err),
        }
    }

    Ok(())
}

/// turns bluetooth on
fn power_on() {
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
