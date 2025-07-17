use crate::tokenizer::tokens::Token;
use crate::tokenizer::tokens::TokenType;
pub struct Lexer {
    source_code: Vec<String>,
    start_slice: u64,
    end_slice: u64,
    token_list: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        todo!()
    }
}
