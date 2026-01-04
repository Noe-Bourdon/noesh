use std::io;
use std::sync::mpsc::{self, Receiver, Sender};

mod Worker_thread;

fn main() {
    let (tx,rx) = mpsc::channel();

   
    loop {
        let thread_tx = tx.clone();
        let command = read_buffer();
        std::thread::spawn(move || {
            match thread_tx.send(command) {
                Ok(_) => println!(""),
                Err(e) => println!("error: {:?}",e),
            }
        })
    }
    
}

/// 標準入力
fn read_buffer() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line.");
    command.trim().to_string()
}