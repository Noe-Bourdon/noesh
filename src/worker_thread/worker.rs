


// pub fn try_revc(rx: Receiver<String>) {
//     //  メッセージが来るまで待ってきたらprintln!する
//     for receiver in rx {
//         println!("message{}", receiver);
//     }  
// }

enum Token {
    Word, //単語
    Pipe, // |
    Redirect,// >>
    And, // &&
    Or, // ||
    Semicolon, // ;
    Quotes, // ''
    Background, //&
    Subshell, // ( )
    Blank, //空白
}   

enum LexerState {
    Normal, //通常
    InWord, // 単語の中

    AfterEscape, // \の後

    InNextRedirect, // >を読んだ次が>>
    InNextAnd, // &を読んだ次が&&
    InNextOr, // |を読んだ次が||

    InSingleQuote, // 'の中
    InDoubleQuote, // "の中
}

struct Lexer {
    parts: Vec<Token>,
    _state: LexerState,
}

pub fn try_revc(cmd: &str) {
    println!("受信{}",cmd);    
}

