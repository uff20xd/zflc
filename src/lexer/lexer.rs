use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenType;
use crate::lexer::tokens::Keyword;

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
        if self.source_code.len() <= self.line { return false; }

        true
    }

    pub fn back(&mut self) -> bool {
        if 0 == self.pos {
            self.line -= 1;
            self.pos = self.source_code[self.line].len();
        } else {
            self.pos -= 1;
        }
        true
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut token_buffer: String = String::new();
        let mut line_buffer;
        let mut pos_buffer;
        let mut current_char: char;
        let mut _lexing = true;

        loop {
            current_char = self.get_char();
            if current_char == ';' {
                tokens.push(
                    Token {
                        token_type: TokenType::Semi,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            } 
            else if current_char == '{' {
                tokens.push(
                    Token {
                        token_type: TokenType::LeftBrace,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char == '}' {
                tokens.push(
                    Token {
                        token_type: TokenType::RightBrace,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char == '(' {
                tokens.push(
                    Token {
                        token_type: TokenType::LeftParen,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char == ')' {
                tokens.push(
                    Token {
                        token_type: TokenType::RightParen,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char == '[' {
                tokens.push(
                    Token {
                        token_type: TokenType::LeftBracket,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char == ':' {
                tokens.push(
                    Token {
                        token_type: TokenType::Colon,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char == '.' {
                tokens.push(
                    Token {
                        token_type: TokenType::Period,
                        line: self.line,
                        pos: self.pos,
                    }
                );
            }
            else if current_char.is_numeric() {
                token_buffer.push(current_char);
                line_buffer = self.line;
                pos_buffer = self.pos;

                if !self.next() { 
                    tokens.push(
                        Token {
                            token_type: TokenType::IntegerLiteral(token_buffer.parse::<i128>()?),
                            line: line_buffer,
                            pos: pos_buffer,
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
                tokens.push(
                    Token {
                        token_type: TokenType::IntegerLiteral(token_buffer.parse::<i128>()?),
                        line: line_buffer,
                        pos: pos_buffer,
                    }
                );
                token_buffer.truncate(0);
                if !self.back() {panic!("Shouldnt get this one. On line: {}", line!())};
            }
            else if current_char == '\"' {
                token_buffer.push(current_char);
            }
            else {
                pos_buffer = self.pos;
                line_buffer = self.line;
                let mut is_next = true;
                loop {

                    println!("{}", current_char);

                    if (current_char == '\t' || current_char == ' ' || is_punctuation(current_char) ) || !is_next {

                        println!("{}", token_buffer.clone());

                        if token_buffer == "return" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Return),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;

                        } 
                        else if token_buffer == "fn" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Function),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "type" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Type),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "let" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Let),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "mut" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Mutable),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "struct" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Struct),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "if" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::If),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "else" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Else),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "while" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::While),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "for" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::For),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "get" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Get),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        }
                        else if token_buffer == "pack" {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Keyword(Keyword::Pack),
                                    line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;
                        } 
                        else if token_buffer.trim() == "" {
                            token_buffer.truncate(0);
                            break;
                        }
                        else {
                            tokens.push(
                                Token {
                                    token_type: TokenType::Ident(token_buffer.clone()), line: line_buffer,
                                    pos: pos_buffer,
                                }
                            );
                            token_buffer.truncate(0);
                            break;

                        }

                    } else {
                        token_buffer.push(current_char)
                    }

                    is_next = self.next();
                    if is_next {
                        current_char = self.get_char();
                    }
                }
            }

            if !self.next() { break; };
        }
        Ok(tokens)
    }
}


fn is_punctuation(ch: char) -> bool {
    match ch {
        '{' => { true },
        '}' => { true },
        '(' => { true },
        ')' => { true },
        '[' => { true },
        ']' => { true },
        _ => { false },
    }
}
