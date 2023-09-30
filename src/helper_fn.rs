use std::process::Command;
use std::time::Duration;
use std::time::Instant;

// use rand::Rng;

/// Input values:
/// 1. Message
/// 2. Exit code (i32)
///
/// Print the given message onto terminal output and exit with the given exit code
///
/// Like this: `print_and_exit!("This failed", 1)`
#[macro_export]
macro_rules! print_and_exit {
    ($msg: expr, $code: expr) => {{
        let code: i32 = $code;
        println!("{}", $msg);
        exit(code);
    }};
}

/// Input values:
/// 1. Message
/// 2. Error
/// 3. Exit code (i32)
///
/// Print the given message and error onto terminal output and exit with the given exit code
///
/// Like this: `print_and_exit!("This failed", io::Error, 1)`
#[macro_export]
macro_rules! print_error_and_exit {
    ($msg: expr, $err: expr, $code: expr) => {{
        let code: i32 = $code;
        println!("{} - {:#?}", $msg, $err);
        exit(code);
    }};
}

/// always runs command as sudo
pub fn run_cmd(current_dir: Option<&str>, cmd: &str) -> Result<(), ()> {
    // println!("\n$ sudo {cmd}");

    let args: Vec<&str> = cmd.split_whitespace().collect();
    let output: std::process::Output;

    if current_dir == None {
        output = match Command::new("sudo").args(args).output() {
            Ok(output) => output,
            Err(error) => {
                println!("Error: {:?}", error);
                return Err(());
            }
        };
    } else {
        let dir = current_dir.expect("run_cmd: `current_dir` could not be unwrapped, but was not None ");
        output = match Command::new("sudo").args(args).current_dir(dir).output() {
            Ok(output) => output,
            Err(error) => {
                println!("Error: {:?}", error);
                return Err(());
            }
        };
    }

    let _stdout = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(error) => {
            println!("! stdout of command {:?} could not be parsed: {:?}", cmd, error);
            return Err(());
        }
    };
    let _stderr = match String::from_utf8(output.stderr) {
        Ok(string) => string,
        Err(error) => {
            println!("! stderr of command {:?} could not be parsed: {:?}", cmd, error);
            return Err(());
        }
    };

    if !_stdout.is_empty() {
        println!("> {:?}", _stdout);
    }

    if !_stderr.is_empty() {
        println!("! {:?}", _stderr);
    }

    return Ok(());
}

/// Tested values:
/// - Interval: 1000 µs
/// - Rounds: 10 000x
/// - Code max "runtime": 500 µs = interval / 2   <br>
/// (its not actually running, just sleeping)
///
/// Results:
/// - Code ran 9 996x out of 10 000x
/// - Average deviation from interval is 18ns
///
/// This benchmark version is mainly here to experiment a bit
fn _single_thread_interval_benchmarked(interval: Duration) {
    // its safe to use u128 for nanoseconds
    // 2^64 ns are ~580 years
    // so 2^128 are 580² years

    let start: Instant = Instant::now();
    let interval_ns = interval.as_nanos();
    let mut interval_counts_before: u128 = 0;
    let mut code_ran: bool = false;

    let mut diffs: Vec<Duration> = Vec::new();

    const ROUNDS: u128 = 100000;

    let mut code_counter = 0;

    // replace with loop later
    while interval_counts_before < ROUNDS {
        // First run the code, which might take longer than the given interval
        // in which case this loops waits for the next interval
        {
            if code_ran == false {
                // program code that should be run once per cycle here

                // _random_wait(Duration::from_micros(350), Duration::from_micros(500));
                code_counter += 1;
            }
            code_ran = true;
        }

        let now: Instant = Instant::now();
        let elapsed_ns: u128 = (now - start).as_nanos();
        let interval_counts_now: u128 = elapsed_ns / interval_ns;

        // By how many ns does the current run deviate from the cycle
        let diff_from_interval_ns: u128 = elapsed_ns % interval_ns;

        let is_next_interval: bool = interval_counts_now > interval_counts_before;
        let is_close_enough: bool = (diff_from_interval_ns as f32 / interval_ns as f32) <= 0.001; // This values decides alot, tests it out

        if is_next_interval && is_close_enough {
            let expected: Instant = start + (interval * interval_counts_now as u32);
            diffs.push(now - expected);

            {
                // code that is supposed to be timed, here
            }

            interval_counts_before = interval_counts_now;
            code_ran = false;
        }
    }

    //  benchmark results

    let mut avg_ns = 0;
    let mut avg_perc: f64 = 0.0;

    for difference in diffs {
        avg_ns += difference.as_nanos();
        let error_percent: f64 = (difference.as_nanos() as f64 / interval.as_nanos() as f64) * 100.0;
        avg_perc += error_percent;
    }

    println!("");
    println!("Code ran {}x, target was {}x", code_counter, ROUNDS);
    println!("Avg ABS  {} ns", avg_ns / ROUNDS);
    println!("Avg PERC {:2.3?} %", avg_perc / ROUNDS as f64);
}

// fn _random_wait(min: Duration, max: Duration) {
//     let min_ns = min.as_nanos() as u64;
//     let max_ns = max.as_nanos() as u64;

//     let range: std::ops::RangeInclusive<u64> = min_ns..=max_ns;

//     let mut rng = rand::thread_rng();
//     let wait_time = Duration::from_nanos(rng.gen_range(range));

//     // println!("waiting for {:?}", wait_time);
//     thread::sleep(wait_time);
// }
