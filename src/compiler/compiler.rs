use tokenizer::tokens::Token;
use tokenizer::tokens::TokenType;
pub struct compiler {
    tokens: Vec<Token>,
    token_pointer: usize,
    output: String,
}

impl compiler {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            output: Vec::new(),
        }
    }

    pub fn next(&mut self) -> bool {
        self.token_pointer += 1;
        if self.token_pointer > self.tokens.len() - 1 {
            return false;
        }
        true
    }

    pub fn get_token(&mut self) -> Token {
        self.tokens[token_pointer]
    }

    pub fn compile(&mut self) -> String {
        let mut current_token: Token;
        let mut output: String = String::new();

        output
    }
}
