use std::io;
use std::sync::mpsc::{self, Receiver, Sender};


fn main()  {
    let stdin = read_buffer();
    loop {
        println!("> {}", stdin);
    }
}

/// 標準入力
fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}