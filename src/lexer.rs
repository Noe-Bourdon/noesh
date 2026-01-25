use nix::sys::signal::Signal::SIGCONT;



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


    fn Lexar_allocation(&mut self, cmd: &str) -> Result<(), String> {
        //文字があるのかないのかをチェック
        let ch = match self.new_state(&cmd) {
            Some(c) => c,
            None => return Ok(()), 
        };

        match self._state {
            LexerState::Nomarl => self.Lexar_Nomal(cmd, ch),
            LexerState::InWord => self.Lexar_InWord(cmd, ch),
            LexerState::InNextAnd => self.Lexar_NextAnd(cmd, ch),
        }
    }

    fn Lexar_Nomal(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        match ch {
            ch if ch.is_alphanumeric() => {
                self.parts.push(Token::Word);
                self._state = LexerState::InWord;
                self.store.push(self.position - ch.len_utf8());
            }
            //パイプが来た場合パイプ決定
            '|' => self.parts.push(Token::Pipe),
            '&' => self._state = LexerState::InNextAnd,
            _ => eprintln!("すいません。対応していません"),
        } 

        Ok(())
    }

    fn is_invalid_char(&mut self, ch: char) -> Option<char> {
        const INVALID: &str = r#"!@#$%^*-_=+./"#;
        INVALID.contains(ch);
        Some(ch)
    }

    fn Lexar_InWord(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        let invalid_char = self.is_invalid_char(ch).unwrap();

        if ch == invalid_char {
            eprintln!("無効な文字です");
        }

        Ok(())
    }

    fn Lexar_NextAnd(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        let mut iter = cmd.chars();
        if ch == '&' {
            iter.next();
            self.parts.push(Token::And);
            self._state = LexerState::Nomarl;
        }
        Ok(())
    }
}