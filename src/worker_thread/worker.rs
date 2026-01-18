
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

///
impl Lexer {
    ///入力文字列の現在地から１文字づつ読む関数
    pub fn next_char(&mut self, cmd: &str) -> Option<char> {
        let mut iter = cmd[self.position..].chars();
        let  ch = iter.next()?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    ///レキサーを格状態に振り分ける関数
    fn Lexar_allocation(&mut self, cmd: &str) -> Result<(), String> {
        //文字があるのかないのかをチェック
        let ch = match self.next_char(cmd) {
            Some(s) => s,
            None => return Ok(()),
        };

        //それぞれのレキサー状態に関数に振り分ける
        match self._state {
            LexerState::Normal => self.Lexer_nomal(cmd, ch),
            LexerState::InWord => self.Lexer_inword(ch, cmd),
            LexerState::AfterEscape => self.Lexer_afterescape(ch, cmd),
            LexerState::InNextRedirect => self.Lexer_innextredirect(ch, cmd),
            LexerState::InNextAnd => self.Lexer_innextand(ch, cmd),
            LexerState::InNextOr => self.Lexer_innextor(ch, cmd),
            LexerState::InSingleQuote => self.Lexer_insinglequote(ch, cmd),
            LexerState::InDoubleQuote => self.Lexer_indoublequote(ch, cmd),
        }
    }

    fn Lexer_nomal(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        //空白の場合はトークンを空白に追加する
        if ch.is_whitespace() {
            self.parts.push(Token::Blank);
        } 

        if ch.is_alphabetic() {
            self.parts.push(Token::Word);
            self._state = LexerState::InWord;
            self.store.push(self.position);
        }
        Ok(())

    }

    fn Lexer_inword(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_afterescape(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_innextredirect(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_innextand(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_innextor(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_insinglequote(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_indoublequote(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        todo!()
    }

}
             
