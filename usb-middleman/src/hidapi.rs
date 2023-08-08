use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
};

use hidapi::HidApi;
use crate::hidapi_structs::*;

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
fn eval_byte_8(controller: &mut UniversalController, byte8: u8) {}

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
        let valid_bytes_count = match device.read_timeout(&mut buf[..], 0) {
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

        let controller: UniversalController = UniversalController {
            sticks: Sticks {
                left: Stick {
                    x: buf[1],
                    y: buf[2],
                    pressed: false,
                },
                right: Stick {
                    x: buf[3],
                    y: buf[4],
                    pressed: false,
                },
            },
            triggers: Triggers {
                left: buf[5],
                right: buf[6],
            },
            bumpers: Bumpers ::allfalse(),
            buttons: MainButtons::allfalse(),
            specials: SpecialButtons::allfalse(),
        };

        benchm_durations[benchm_index] = benchmark.elapsed();
        benchm_average = benchm_durations.iter().sum::<Duration>() / benchm_durations.len() as u32;
        benchm_index = (benchm_index + 1) % BENCHMARK_SAMPLES;

        // Output handling
        let _ = Command::new("clear").status();
        println!("avg execution time: {:?}", benchm_average);

        println!("{:?}", &buf[8..valid_bytes_count]);
        println!("Lx: {:?}", controller.sticks.left.x);
        println!("Ly: {:?}", controller.sticks.left.y);
        println!("Rx: {:?}", controller.sticks.right.x);
        println!("Ry: {:?}", controller.sticks.right.y);
        println!("Tl: {:?}", controller.triggers.left);
        println!("Tr: {:?}", controller.triggers.right);

        thread::sleep(Duration::from_micros(1500)); // the lower this can be, the better the delay
    }
}
