use std::io;
use std::sync::mpsc::{self};

use crate::worker_thread::worker;

mod worker_thread;

fn main() {
    shell_loop();
}

fn shell_loop() {
    loop {
        match standard_input() {
            Ok(cmd) if !cmd.is_empty() => {
                worker_thread::worker::try_revc(&cmd);
            }
            Err(e) => println!("{}",e),
            _ => return
        }
    }
}

fn standard_input() -> Result<String, String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    Ok(buffer.trim().to_string())
}        