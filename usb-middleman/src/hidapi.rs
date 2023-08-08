use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
};

use crate::hidapi_structs::*;
use hidapi::HidApi;

fn prepare_device() -> hidapi::HidDevice {
    let api = HidApi::new().unwrap();

    let mut vid: u16 = 0;
    let mut pid: u16 = 0;

    // Print out information about all connected devices
    for device in api.device_list() {
        if device.vendor_id() == 1356 {
            println!("{:#?}", device);
            vid = 1356;
            pid = device.product_id();
        }
    }

    let device = match api.open(vid, pid) {
        Ok(hid_device) => hid_device,
        Err(err) => panic!("Error: {:?}", err),
    };

    // if false, calls to read may return nothing, but also dont block
    match device.set_blocking_mode(false) {
        Ok(_) => (),
        Err(err) => panic!("HidError: {:?}", err),
    }
    device
}

/// modifies the given controller such that the pressed buttons are correctly set
fn eval_byte_8(controller: &mut UniversalController, byte8: u8) {
    // ABXY Buttons
    if byte8 & 0b10000000 != 0 {
        controller.buttons.upper = true;
    }
    if byte8 & 0b01000000 != 0 {
        controller.buttons.right = true;
    }
    if byte8 & 0b00100000 != 0 {
        controller.buttons.lower = true;
    }
    if byte8 & 0b00010000 != 0 {
        controller.buttons.left = true;
    }

    // dpad is counting up, top = 0, top right = 1, right = 2, right bottom = 3, bottom = 4, bottom left = 5, left = 6, left top = 7

    match 0b00001111 & byte8 {
        0 => controller.dpad.up = true,
        1 => {
            controller.dpad.up = true;
            controller.dpad.right = true;
        }
        2 => controller.dpad.right = true,
        3 => {
            controller.dpad.right = true;
            controller.dpad.down = true;
        }
        4 => controller.dpad.down = true,
        5 => {
            controller.dpad.left = true;
            controller.dpad.down = true;
        }
        6 => controller.dpad.left = true,
        7 => {
            controller.dpad.left = true;
            controller.dpad.up = true;
        }
        8 => (), // no dpad pressed
        _ => (), // because of & OP this is never the case
    }
}

pub fn hidapi() {
    let device = prepare_device();

    // prepare for data
    const HID_ARRAY_SIZE: usize = 14;
    let mut buf = [0 as u8; HID_ARRAY_SIZE];

    // prepare benchmark
    const BENCHMARK_SAMPLES: usize = 1000;
    let mut benchm_durations: [Duration; BENCHMARK_SAMPLES] =
        [Duration::from_secs(0); BENCHMARK_SAMPLES];
    let mut benchm_index: usize = 0;
    let mut benchm_average: Duration;

    loop {
        let benchmark = Instant::now();

        // setting -1 as timeout means waiting for the next input event, in this mode valid_bytes_count == HID_ARRAY_SIZE
        // setting 0ms as timeout, probably means sometimes the previous input event is taken, but the execution time of this whole block is 100x faster!
        // also: reading in blocking mode might be problematic if the controller is disconnected => infinite wait
        let _valid_bytes_count = match device.read_timeout(&mut buf[..], 0) {
            Ok(value) => {
                if value != HID_ARRAY_SIZE {
                    continue;
                } else {
                    value
                }
            }
            Err(_) => continue,
        };

        // first and 8. byte not needed
        // 13. and 14. byte are some sort of counter, either time or reads

        // example output: [1, 127, 128, 127, 128, 0, 0, 18,  4, 0, 0, 0]
        // explaination:    ?,  Lx,  Ly,  Rx,  Ry, ?, ?, rng, *, #, ~,

        let mut controller: UniversalController = UniversalController {
            sticks: Sticks {
                left: Stick {
                    x: buf[1],
                    y: buf[2],
                    pressed: match buf[9] {
                        64 => true,
                        _ => false,
                    },
                },
                right: Stick {
                    x: buf[3],
                    y: buf[4],
                    pressed: match buf[9] {
                        128 => true,
                        _ => false,
                    },
                },
            },
            triggers: Triggers {
                left: buf[5],
                right: buf[6],
            },
            bumpers: Bumpers {
                left: match buf[9] {
                    1 => true,
                    _ => false,
                },
                right: match buf[9] {
                    2 => true,
                    _ => false,
                },
            },
            buttons: MainButtons::allfalse(),
            dpad: DPad::allfalse(),
            specials: SpecialButtons {
                touchpad: match buf[11] {
                    2 => true,
                    _ => false,
                },
                right: match buf[11] {
                    32 => true,
                    _ => false,
                },
                left: match buf[10] {
                    16 => true,
                    _ => false,
                },
                logo: match buf[11] {
                    1 => true,
                    _ => false,
                },
            },
        };

        eval_byte_8(&mut controller, buf[8]);

        benchm_durations[benchm_index] = benchmark.elapsed();
        benchm_average = benchm_durations.iter().sum::<Duration>() / benchm_durations.len() as u32;
        benchm_index = (benchm_index + 1) % BENCHMARK_SAMPLES;

        // Output handling
        let _ = Command::new("clear").status();
        println!("avg execution time: {:?}", benchm_average);

        println!("Lx: {:?}", controller.sticks.left.x);
        println!("Ly: {:?}", controller.sticks.left.y);
        println!("L : {:?}", controller.sticks.left.pressed);

        println!("Rx: {:?}", controller.sticks.right.x);
        println!("Ry: {:?}", controller.sticks.right.y);
        println!("R : {:?}", controller.sticks.right.pressed);

        println!("Tl: {:?}", controller.triggers.left);
        println!("Tr: {:?}", controller.triggers.right);
        println!("Bl: {:?}", controller.bumpers.left);
        println!("Br: {:?}", controller.bumpers.right);

        println!("X: {:?}", controller.buttons.lower);
        println!("O: {:?}", controller.buttons.right);
        println!("□: {:?}", controller.buttons.left);
        println!("∆: {:?}", controller.buttons.upper);

        println!("↑: {:?}", controller.dpad.up);
        println!("→: {:?}", controller.dpad.right);
        println!("↓: {:?}", controller.dpad.down);
        println!("←: {:?}", controller.dpad.left);

        thread::sleep(Duration::from_micros(1500)); // the lower this can be, the better the delay
    }
}
