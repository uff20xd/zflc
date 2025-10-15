#[derive(Debug, Clone)]
pub enum TokenType {
    IntegerLiteral(i128),
    StringLiteral(String),
    Ident(String),
    Semi,
    Period,
    Colon,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Plus,
    Minus,
    Mult,
    Divide,
    Modulo,

    EqualTo,
    LesserThan,
    GreaterThan,
    GreaterThanOrEqualTo,
    LesserThanOrEqualTo,

    Return,
    Function,
    Public,
    Struct,
    Enum,
    Type,
    Pack,
    Get,
    For,
    While,
    Loop,
    If,
    Else,
    Let,
    Mutable,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub pos: usize,
    pub token_type: TokenType,
}

impl Token {
    pub fn is_math_operator(&self) -> bool {
        match self.token_type {
            TokenType::Plus => {true},
            TokenType::Minus => {true},
            TokenType::Mult => {true},
            TokenType::Divide=> {true},
            TokenType::Modulo => {true},
            _ => { false },
        }
    }

    pub fn is_bool_operator(&self) -> bool {
        match self.token_type {
            TokenType::EqualTo => { true },
            TokenType::LesserThan => { true },
            TokenType::GreaterThan => { true },
            TokenType::GreaterThanOrEqualTo => { true },
            TokenType::LesserThanOrEqualTo => { true },
            _ => { false },
        }
    }
}
