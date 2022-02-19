use std::io::{self, Write};
use text_io::read;

pub fn log_command(cmd: &str) {
    println!("\x1b[2;32m[cmd]\x1b[0m {}", cmd);
}

pub fn log_info(info: &str) {
    println!("\x1b[2m[info]\x1b[0m {}", info);
}

pub fn log_err(err: &str) {
    println!("\x1b[1;31m[err]\x1b[0m {}", err);
}

pub fn question(prompt: &str) -> String {
    print!("\x1b[2;34m[question]\x1b[0m {} \x1b[1m", prompt);
    io::stdout().flush().expect("Could not flush stdout");
    let result = read!("{}\n");
    print!("\x1b[0m");
    result
}