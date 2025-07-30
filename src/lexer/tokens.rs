#[derive(Debug, Clone)]
pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Keyword(Keyword),
    Ident(String),
    Semi,
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
