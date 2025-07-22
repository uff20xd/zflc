#[derive(Debug, Clone)]
pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Keyword(String),
    Ident(String),
    Semi,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub pos: usize,
    pub token_type: TokenType,
}
