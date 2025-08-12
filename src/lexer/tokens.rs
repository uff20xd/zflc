#[derive(Debug, Clone)]
pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Keyword(Keyword),
    Ident(String),
    Semi,
    BoolOperator(BoolOperator),

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    EqualTo,
    LesserThan,
    GreaterThan,
    GreaterThanOrEqualTo,
    LesserThanOrEqualTo,
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Return,
    Function,
    Public,
    Struct,
    Enum,
    Type,
    Get,
    For,
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

    pub fn is_math_operator(&self) -> bool {
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
