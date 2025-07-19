#[derive(Debug)]
pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Return,
    Keyword,
    FunctionDefinition,
    Identifier,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub pos: usize,
    pub token_type: TokenType,
}

