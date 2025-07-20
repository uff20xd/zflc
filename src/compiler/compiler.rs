use tokenizer::tokens::Token;
use tokenizer::tokens::TokenType;
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
        output.push_str("
format ELF64 executable

segment readable executable
entry main
main:
");
        loop {
            match current_token.token_type {
                TokenType::Return => {
                    output.push_str("   mov rax,60\n");
                    if let Some(token) = self.next() {
                        match token.token_type {
                            TokenType::IntegerLiteral(value) => {
                                let instruction = format!("    mov rdi,{}", value);
                                output.push_str(&instruction);
                            },
                            _ => {panic!("The return statement requires an integerlitera as its next token! at:{}:{}", token.line, token.pos)}
                        }
                    }
                    else {panic!("Code ends unexpectedly after char:{} line:{}", current_token.pos, current_token.line)}

                    if let Some(token) = self.next() {
                        match token.token_type {
                            TokenType::Semi => {
                                output.push_str("   syscall\n");
                            },
                            _ => {panic!("The return statement requires an integerlitera as its next token! at:{}:{}", token.line, token.pos)}
                        }
                    }

                    else {panic!("Code ends unexpectedly after char:{} line:{}", current_token.pos, current_token.line)}

                },
                _ => {panic!("not implemented this keyword yet")}
            }

            if let Some(token) = self.next() { current_token = token; }
            else {break;}
        }
        output
    }
}
