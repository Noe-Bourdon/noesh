


// pub fn try_revc(rx: Receiver<String>) {
//     //  メッセージが来るまで待ってきたらprintln!する
//     for receiver in rx {
//         println!("message{}", receiver);
//     }  
// }


pub fn try_revc(cmd: &str) {
    println!("受信{}",cmd);
}