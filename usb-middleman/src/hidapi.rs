use std::{
    process::{Command, exit},
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

        // Sony is 1356
        if device.vendor_id() == 1356 {
            println!("{:#?}", device);
            vid = 1356;
            pid = device.product_id();

            // FIXME: The functions so far only work if connected via usb, 
            // testing is needed to see how that can be generalized

            println!("bus type {:?}", device.bus_type());
            println!("interface nr {:?}", device.interface_number());
            println!("product {:?}", device.product_string());
            println!("release {:?}", device.release_number());
            println!("usage {:?}", device.usage());
            println!("usage page {:?}", device.usage_page());
        }
    }

    exit(0);

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

/// modifies the given controller such that the pressed buttons (xyab for xbox) and dpad buttons are correctly set
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

fn interpret_input(input_buf: [u8; 14]) -> UniversalController {
    let mut controller: UniversalController = UniversalController {
        sticks: Sticks {
            left: Stick {
                x: input_buf[1],
                y: input_buf[2],
                pressed: match input_buf[9] {
                    64 => true,
                    _ => false,
                },
            },
            right: Stick {
                x: input_buf[3],
                y: input_buf[4],
                pressed: match input_buf[9] {
                    128 => true,
                    _ => false,
                },
            },
        },
        triggers: Triggers {
            left: input_buf[5],
            right: input_buf[6],
        },
        bumpers: Bumpers {
            left: match input_buf[9] {
                1 => true,
                _ => false,
            },
            right: match input_buf[9] {
                2 => true,
                _ => false,
            },
        },
        buttons: MainButtons::allfalse(),
        dpad: DPad::allfalse(),
        specials: SpecialButtons {
            touchpad: match input_buf[11] {
                2 => true,
                _ => false,
            },
            right: match input_buf[11] {
                32 => true,
                _ => false,
            },
            left: match input_buf[10] {
                16 => true,
                _ => false,
            },
            logo: match input_buf[11] {
                1 => true,
                _ => false,
            },
        },
    };

    eval_byte_8(&mut controller, input_buf[8]);
    controller
}

fn terminal_output(
    benchm_average: Duration,
    controller: UniversalController,
    show_all_keys: bool,
    show_benchmark: bool,
) {
    let _ = Command::new("clear").status();

    if show_benchmark {
        println!("avg execution time: {:?}", benchm_average);
    }

    if show_all_keys {
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
    }
}

pub fn hidapi() {
    let device = prepare_device();

    // prepare for data
    const HID_ARRAY_SIZE: usize = 14;
    let mut input_buf = [0 as u8; HID_ARRAY_SIZE];

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
        let _valid_bytes_count: usize = match device.read_timeout(&mut input_buf[..], 0) {
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

        let controller = interpret_input(input_buf);

        // End Benchmark and Calc Average
        benchm_durations[benchm_index] = benchmark.elapsed();
        benchm_average = benchm_durations.iter().sum::<Duration>() / benchm_durations.len() as u32;
        benchm_index = (benchm_index + 1) % BENCHMARK_SAMPLES;

        // Output if wanted
        terminal_output(benchm_average, controller, true, true);

        thread::sleep(Duration::from_micros(1500)); // the lower this can be, the better the delay
    }
}
