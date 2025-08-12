#[derive(Debug, Clone)]
pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Keyword(Keyword),
    Ident(String),
    Semi,
    MathOperator(MathOperator),
    BoolOperator(BoolOperator),
}

pub enum MathOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

pub enum BoolOperator {
    Equal,
    LesserThan,
    GreaterThan,
    GreaterThanOrEqualTo,
    LesserThanOrEqualTo
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
