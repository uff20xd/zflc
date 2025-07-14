pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Return,
    FunctionDefinition,
    Identifier,
    Semi,
}

pub struct Token {
    line: u64,
    pos: u64,
    token_type: TokenType,
}

