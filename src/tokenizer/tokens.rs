pub enum TokenType {
    IntegerLiteral(i64),
    StringLiteral(i64),
    Return,
    FunctionDefinition,
    Identifier,
    Semi,
}

pub struct Token {
    line: usize,
    pos: usize,
    token_type: TokenType,
}

