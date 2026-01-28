


#[derive(Debug, PartialEq)]
///トークン
enum Token {
    Word(String), //単語
    Pipe, // |
    And,  //　&&
}

///レキサーの状態
enum LexerState {
    Nomarl,    //通常
    InWord,    //単語
    InNextAnd, //&の次が&&かを判定
}

///管理状態
pub struct Lexer {
    parts: Vec<Token>,
    _state: LexerState,
    position: usize,
    store: Vec<usize>,
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

        //コマンドのpositonを進めながら、１文字ずつ読みって状態返還を行うループ
        while self.position < cmd.len() {
            let ch = self.new_state(cmd).unwrap();
            match self._state {
                LexerState::Nomarl => self.Lexar_Nomal(cmd, ch).unwrap(),
                LexerState::InWord => self.Lexar_InWord(cmd, ch).unwrap(),
                LexerState::InNextAnd => self.Lexar_NextAnd(cmd, ch).unwrap(),
            }
        }
        Ok(())
    }

    fn Lexar_Nomal(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        match ch {
            ch if ch.is_alphanumeric() => {
                //Stringを新しく作ってるだけで値が入るわけではない
                self.parts.push(Token::Word(String::new()));
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
        } else {
            let start = self.store.pop().unwrap();
            let end = self.position - ch.len_utf8();
            let word = &cmd[start..end];
            self.parts.push(Token::Word(word.to_string()));
        }

        Ok(self._state = LexerState::Nomarl) 
    }

    fn Lexar_NextAnd(&mut self, cmd: &str, ch: char) -> Result<(), String> {
        let mut iter = cmd.chars();
        if ch == '&' {
            iter.next();
            self.parts.push(Token::And);
            self._state = LexerState::Nomarl;
            self.store.push(self.position - ch.len_utf8());
        }
        Ok(())
    }
}

//テスト
#[cfg(test)]
mod lexer {
    use crate::lexer::{self, Lexer, Token};

    #[test]
    fn test_pipe() {
        let mut lexer = Lexer::new();
        lexer.Lexar_allocation("echo hello | grep h").unwrap();
        let e = vec![
            lexer::Token::Word("echo".into()),
            lexer::Token::Word("hello".into()),
            lexer::Token::Pipe,
            lexer::Token::Word("grep".into()),
            lexer::Token::Word("h".into()),
        ];

        assert_eq!(lexer.parts, e);
    }
}
