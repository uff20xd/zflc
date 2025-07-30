use lexer::tokens::Token;
use lexer::tokens::TokenType;
pub struct Compiler {
    tokens: Vec<Token>,
    token_pointer: usize,
    output: String,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            output: String::new(),
            token_pointer: 0,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.token_pointer += 1;
        if self.token_pointer > self.tokens.len() - 1 {
            return None;
        }
        Some(self.tokens[self.token_pointer].clone())
    }

    pub fn compile(&mut self) -> String {
        let mut current_token: Token = self.tokens[self.token_pointer].clone();
        let mut output: String = String::new();
        output
    }
}
