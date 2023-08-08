use std::{
    process::{exit, Command},
    thread,
    time::{Duration, Instant},
};

use crate::structs::*;
use hidapi::{HidApi, HidDevice};

/// modifies the given gamepad such that the pressed buttons (xyab for xbox) and dpad buttons are correctly set
fn eval_byte_8(gamepad: &mut UniversalGamepad, byte8: u8) {
    // ABXY Buttons
    if byte8 & 0b10000000 != 0 {
        gamepad.buttons.upper = true;
    }
    if byte8 & 0b01000000 != 0 {
        gamepad.buttons.right = true;
    }
    if byte8 & 0b00100000 != 0 {
        gamepad.buttons.lower = true;
    }
    if byte8 & 0b00010000 != 0 {
        gamepad.buttons.left = true;
    }

    // dpad is counting up, top = 0, top right = 1, right = 2, right bottom = 3, bottom = 4, bottom left = 5, left = 6, left top = 7

    match 0b00001111 & byte8 {
        0 => gamepad.dpad.up = true,
        1 => {
            gamepad.dpad.up = true;
            gamepad.dpad.right = true;
        }
        2 => gamepad.dpad.right = true,
        3 => {
            gamepad.dpad.right = true;
            gamepad.dpad.down = true;
        }
        4 => gamepad.dpad.down = true,
        5 => {
            gamepad.dpad.left = true;
            gamepad.dpad.down = true;
        }
        6 => gamepad.dpad.left = true,
        7 => {
            gamepad.dpad.left = true;
            gamepad.dpad.up = true;
        }
        8 => (), // no dpad pressed
        _ => (), // because of & OP this is never the case
    }
}

fn interpret_input(input_buf: [u8; 14]) -> UniversalGamepad {
    let mut gamepad: UniversalGamepad = UniversalGamepad {
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

    eval_byte_8(&mut gamepad, input_buf[8]);
    gamepad
}

fn terminal_output(
    benchm_average: Duration,
    gamepad: UniversalGamepad,
    show_all_keys: bool,
    show_benchmark: bool,
) {
    let _ = Command::new("clear").status();

    if show_benchmark {
        println!("avg execution time: {:.0?}", benchm_average);
    }

    if show_all_keys {
        println!("Lx: {:?}", gamepad.sticks.left.x);
        println!("Ly: {:?}", gamepad.sticks.left.y);
        println!("L : {:?}", gamepad.sticks.left.pressed);

        println!("Rx: {:?}", gamepad.sticks.right.x);
        println!("Ry: {:?}", gamepad.sticks.right.y);
        println!("R : {:?}", gamepad.sticks.right.pressed);

        println!("Tl: {:?}", gamepad.triggers.left);
        println!("Tr: {:?}", gamepad.triggers.right);
        println!("Bl: {:?}", gamepad.bumpers.left);
        println!("Br: {:?}", gamepad.bumpers.right);

        println!("X: {:?}", gamepad.buttons.lower);
        println!("O: {:?}", gamepad.buttons.right);
        println!("□: {:?}", gamepad.buttons.left);
        println!("∆: {:?}", gamepad.buttons.upper);

        println!("↑: {:?}", gamepad.dpad.up);
        println!("→: {:?}", gamepad.dpad.right);
        println!("↓: {:?}", gamepad.dpad.down);
        println!("←: {:?}", gamepad.dpad.left);
    }
}

// FIXME: The functions so far only work if connected via usb,
// testing is needed to see how that can be generalized
pub fn read_ps5_usb(device: &HidDevice) {
    // if false, calls to read may return nothing, but also dont block
    match device.set_blocking_mode(false) {
        Ok(_) => (),
        Err(err) => panic!("HidError: {:?}", err),
    }

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
        // also: reading in blocking mode might be problematic if the gamepad is disconnected => infinite wait
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

        let gamepad = interpret_input(input_buf);

        // End Benchmark and Calc Average
        benchm_durations[benchm_index] = benchmark.elapsed();
        benchm_average = benchm_durations.iter().sum::<Duration>() / benchm_durations.len() as u32;
        benchm_index = (benchm_index + 1) % BENCHMARK_SAMPLES;

        // Output if wanted
        terminal_output(benchm_average, gamepad, true, true);

        thread::sleep(Duration::from_micros(1500)); // <= 1500 is fine for now delay
    }
}
