use crate::lexer::{self, Lexer};



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



