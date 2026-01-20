use std::io;

use crate::worker_thread::worker::Lexer;

mod worker_thread;

fn main() {
    shell_loop();
}

fn shell_loop() {
    loop {
        match standard_input() {
            Ok(cmd) if !cmd.is_empty() => {
                let mut lexer = Lexer::new();
                lexer.next_char(&cmd); 
            }
            Err(e) => println!("{}", e),
            _ => return,
        }
    }
}

fn standard_input() -> Result<String, String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    Ok(buffer.trim().to_string())
}
