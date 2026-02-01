use std::io;

//ファイルをインポート
mod lexer;

//

fn main() {
    shell_loop();
}

fn shell_loop() {
    loop {
        match standard_input() {
            Ok(cmd) if !cmd.is_empty() => {
                let mut send = lexer::Lexer::new();
                while let Some(ch) = send.new_state(&cmd) {
                    print!("{}",ch);   
                }
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
