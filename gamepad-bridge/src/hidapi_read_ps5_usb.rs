use std::{
    thread,
    time::{Duration, Instant},
};

use hidapi::HidDevice;

const HID_ARRAY_SIZE: usize = 48;

// FIXME: The functions so far only work if connected via usb,
// testing is needed to see how that can be generalized
pub fn read_ps5_usb(device: &HidDevice) {
    // if false, calls to read may return nothing, but also dont block
    match device.set_blocking_mode(false) {
        Ok(_) => (),
        Err(err) => panic!("HidError: {:?}", err),
    }

    // prepare for data

    let mut input_buf = [0 as u8; HID_ARRAY_SIZE];

    // prepare benchmark
    const BENCHMARK_SAMPLES: usize = 1000;
    let mut benchm_durations: [Duration; BENCHMARK_SAMPLES] = [Duration::from_secs(0); BENCHMARK_SAMPLES];
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

        // let gamepad = interpret_input(input_buf);

        // End Benchmark and Calc Average
        benchm_durations[benchm_index] = benchmark.elapsed();
        benchm_average = benchm_durations.iter().sum::<Duration>() / benchm_durations.len() as u32;
        benchm_index = (benchm_index + 1) % BENCHMARK_SAMPLES;

        // Output if wanted
        // terminal_output(benchm_average, gamepad, true, true);

        thread::sleep(Duration::from_micros(1500)); // <= 1500 is fine for now delay
    }
}
