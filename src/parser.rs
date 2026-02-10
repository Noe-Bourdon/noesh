use nix::libc::seccomp_data;

use crate::lexer::Token;
/// ASTのパイプの場合の設計図
/// AST::Pipe (
///     Box::new(AST::Command(Command {name: echo, args: ["hello"] })),
///     Box::new(AST::Command(Command {name: grep, args: ["h"] })),
/// )　
/// 

//コマンド Command {name: echo, args: ["hello"] }
#[derive(Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
}

//AST::Pipe (
//     Box::new(AST::Command
//     Box::new(AST::Command
// )
#[derive(Debug)]
pub enum AST {
    Command(Command),
    Pipe(Box<AST>, Box<AST>),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parser(&mut self) {
       self.parser_pipe();
    }

    //値があるか見るだけの関数
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    //消費する関数
    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let tok = self.tokens[self.position].clone();
            self.position += 1;
            Some(tok)
        } else {
            None
        }
    }

    pub fn parser_pipe(&mut self) -> AST {
        let mut left = self.parser_command();

        while let Some(Token::Pipe) = self.peek() {
           self.advance();
           let right = self.parser_command();
           left = AST::Pipe(Box::new(left), Box::new(right)) 
        }
        left
    }

    pub fn parser_command(&mut self) -> AST {
        let name = match self.advance() {
            Some(Token::Word(w)) => w,
            _ => panic!("無効な値"),
        };

        let args = Vec::new();
        while let Some(Token::Word(w)) = self.peek() {
            self.advance();
            args.push(w);
        }
        AST::Command(Command { name, args})
    }


}
     