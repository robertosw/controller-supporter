use std::process::{exit, Command};

/// Print the given `msg` onto terminal output and exit with given `exit_code`
pub fn print_and_exit(msg: &str, exit_code: i32) -> ! {
    println!("{msg}");
    exit(exit_code);
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
