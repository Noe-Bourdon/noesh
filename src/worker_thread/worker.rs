use nix::sys::statvfs::vfs::NOATIME;


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
    ///入力文字列の現在地から１文字づつ読む関数
    fn next_char(&mut self, cmd: &str) -> Option<char> {
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
            LexerState::Normal => Lexer_nomal(ch, cmd),
            LexerState::InWord => Lexer_inword(ch, cmd),
            LexerState::AfterEscape => Lexer_afterescape(ch, cmd),
            LexerState::InNextRedirect => Lexer_innextredirect(ch, cmd),
            LexerState::InNextAnd => Lexer_innextand(ch, cmd),
            LexerState::InNextOr => Lexer_innextor(ch, cmd),
            LexerState::InSingleQuote => Lexer_insinglequote(ch, cmd),
            LexerState::InDoubleQuote => Lexer_indoublequote(ch, cmd),
        }
    }
}