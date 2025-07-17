use crate::tokenizer::tokens::Token;
use crate::tokenizer::tokens::TokenType;
pub struct Lexer {
    source_code: Vec<Vec<char>>,
    pos: usize,
    line: usize,
    token_list: Vec<Token>,
}


impl Lexer {
    pub fn new(raw_source_code: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut source_code: Vec<Vec<char>> = Vec::new();
        for line in raw_source_code {
            source_code.push(line.chars().collect::<Vec<char>>());
        }
        let ret = Self {
            source_code,
            line: 0,
            pos: 0,
            token_list: Vec::new(),
        };
        Ok(ret)
    }
    fn get_char(&self) -> char {
        self.source_code[self.line][self.pos]
    }

    fn next(&mut self) -> bool {
        self.pos += 1;
        if self.source_code[self.line].len() - 1 < self.pos {
            self.line += 1;
            self.pos = 0;
        }
        if self.source_code.len() - 1 < self.line { return false; }

        true
    }

    pub fn back(&mut self) -> bool {
        self.pos -= 1;
        if 0 > self.pos {
            self.line -= 1;
            self.pos = self.source_code[self.line].len();
        }
        if 0 > self.line { return false; }
        true
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut token_buffer: String = String::new();
        let mut line_buffer;
        let mut pos_buffer;
        let mut current_char: char;

        loop {
            current_char = self.get_char();
            if current_char == ';' {
                tokens.push(
                    Token {
                        token_type: TokenType::Semi,
                        line: self.line,
                        pos: self.pos,
                    }
                )
            } 
            else if current_char.is_numeric() {
                token_buffer.push(current_char);
                line_buffer = self.line;
                pos_buffer = self.pos;
                
                if !self.next() { 
                    tokens.push(
                        Token {
                            token_type: IntegerLiteral(token_buffer.parse::<i64>()?),

                        }
                    );
                    break; 
                }

                current_char = self.get_char();

                while current_char.is_numeric() {
                    token_buffer.push(current_char);
                    if !self.next() { break; }
                    current_char = self.get_char();
                }
                tokens.push
                if !self.back() {panic!("Shouldnt get this one. On line: {}", line!())};
            }

            if !self.next() { break; };
        }
        tokens
    }
}
