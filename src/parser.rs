use crate::lexer::{self, Lexer};



#[derive(Debug)]
pub enum AST {
   Word(String), 
   Pipe,
   And,
}

impl AST {
    pub fn a() {
        let mut lexer = Lexer::new();
        lexer.lexar_allocation(&cmd).unwrap();

        
    }
}
