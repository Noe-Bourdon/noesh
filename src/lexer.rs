

//トークン
enum Token {
    Word,   //単語
    Pipe,   // |
    And,    //　&&
}

enum LexerState {
    Nomarl,     //通常 
    InWord,     //単語
    InNextAnd,  //&の次が&&かを判定
}

struct Lexer {
    parts: Vec<Token>,
    _state: LexerState,
    position: usize,
    store: Vec<usize>
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            parts: Vec::new(),
            _state: LexerState::Nomarl,
            position: 0,
            store: Vec::new(), 
        }
    }
    
    ///positionを使って、cmdの文字をひとずつ進める関数
    pub fn new_state(&mut self, cmd: &str) -> Option<char> {
        let mut iter = cmd[self.position..].chars();
        let ch = iter.next()?;
        self.position += ch.len_utf8();
        Some(ch)
    }
}