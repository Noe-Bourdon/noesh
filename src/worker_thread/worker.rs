
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
    parts: Vec<Token>, //パーツの種類を見る
    _state: LexerState, //状態の種類を見る
    position: usize, //どこを読んでいるか
    store: Vec<usize>, //読み込みデータの置き場
}

pub fn try_revc(cmd: &str) {
    println!("受信{}",cmd);    
}

///
impl Lexer {
    ///１つ進めるだけの関数
    fn next_char(&mut self, cmd: &str) -> Option<char> {
        if self.position >= cmd.len() {
            return  None;
        }

        let ch = cmd[self.position];
        self.position += 1;

        Some(ch)
    }
}