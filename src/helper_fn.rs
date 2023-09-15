use std::process::Command;

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
pub fn run_cmd(current_dir: &str, cmd: &str) -> Result<(), ()> {
    println!("\n$ sudo {cmd}");
    let args: Vec<&str> = cmd.split_whitespace().collect();

    let output = match Command::new("sudo").args(args).current_dir(current_dir).output() {
        Ok(output) => output,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(());
        }
    };
    let stdout = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(error) => {
            println!("! stdout of command {:?} could not be parsed: {:?}", cmd, error);
            return Err(());
        }
    };
    let stderr = match String::from_utf8(output.stderr) {
        Ok(string) => string,
        Err(error) => {
            println!("! stderr of command {:?} could not be parsed: {:?}", cmd, error);
            return Err(());
        }
    };
    println!("> {:?}", stdout);
    println!("! {:?}", stderr);

    return Ok(());
}
