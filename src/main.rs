use std::io;
use std::sync::mpsc::{self};

mod worker_thread;

fn main() {
    let (tx,_rx) = mpsc::channel();

    loop {
        let thread_tx = tx.clone();
        let command = read_buffer();
        let send_value = command.clone();
        std::thread::spawn(move || {
            match thread_tx.send(send_value) {
                Ok(_) => println!(""),
                Err(e) => println!("error: {:?}",e),
            }           
        });
       worker_thread::worker::try_revc(&command);
    }
    
    
}

/// 標準入力
fn read_buffer() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line.");
    command.trim().to_string()
} 


