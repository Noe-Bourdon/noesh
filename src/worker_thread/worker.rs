


enum Token {
    Word,         // 単語
    Pipe,         // |
    Redirect,     // >>
    And,          // &&
    Or,           // ||
    Semicolon,    // ;
    SingleQuote,  // '
    DoubleQuote,  // "
    Background,   // &
    Subshell,     // ( )
    Blank,        // 空白
}

enum LexerState {
    Normal,            // 通常
    InWord,            // 単語の中
    AfterEscape,       // \ の後
    InNextRedirect,    // > の次が >> か判定
    InNextAnd,         // & の次が && か判定
    InNextOr,          // | の次が || か判定
    InSingleQuote,     // ' の中
    InDoubleQuote,     // " の中
}

pub struct Lexer {
    parts: Vec<Token>, //パーツの種類を見る
    _state: LexerState, //状態の種類を見る
    position: usize, //どこを読んでいるか
    store: Vec<usize>, //読み込みデータの置き場
}

///
impl Lexer {
    pub fn new() -> Self {
        Self { parts: Vec::new(), _state: LexerState::Normal, position: 0, store: Vec::new() }
    }
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
        let ch = match self.next_char(&cmd) {
            Some(s) => s,
            None => return Ok(()),
        };

        //それぞれのレキサー状態に関数に振り分ける
        match self._state {
            LexerState::Normal => self.Lexer_nomal(cmd, ch),
            LexerState::InWord => self.Lexer_inword(cmd, ch),
            LexerState::AfterEscape => self.Lexer_afterescape(cmd, ch),
            LexerState::InNextRedirect => self.Lexer_innextredirect(cmd, ch),
            LexerState::InNextAnd => self.Lexer_innextand(cmd, ch),
            LexerState::InNextOr => self.Lexer_innextor(cmd, ch),
            LexerState::InSingleQuote => self.Lexer_insinglequote(cmd, ch),
            LexerState::InDoubleQuote => self.Lexer_indoublequote(cmd, ch),
        }
    }

    fn Lexer_nomal(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        match ch {
            //単語の場合トークン確定
            ch if ch.is_alphabetic() => {
                self.parts.push(Token::Word);
                self._state = LexerState::InWord;
                self.store.push(self.position - ch.len_utf8());
            }
            //パイプはスキップ
            '>' => self._state = LexerState::InNextRedirect,
            '&' => self._state = LexerState::InNextAnd,
            '|' => self._state = LexerState::InNextOr,
            ';' => self.parts.push(Token::Semicolon), //セミコロントークン確定
            '\'' => self._state = LexerState::InSingleQuote,
            '"' => self._state = LexerState::InDoubleQuote,
            //空白の場合トークン確定
            ch if ch.is_whitespace() => {
                self.parts.push(Token::Blank);
            }
            _ => {},
        }
        Ok(())
    }

    fn Lexer_inword(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()  
    }

    fn Lexer_afterescape(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_innextredirect(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_innextand(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_innextor(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_insinglequote(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()
    }

    fn Lexer_indoublequote(&mut self, _cmd: &str, _ch: char) -> Result<(), String> {
        todo!()
    }
}
             
