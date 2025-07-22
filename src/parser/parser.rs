use crate::tokenizer::tokens::Token;
use crate::tokenizer::tokens::TokenType;

pub enum NodeType {
    Value,
    Bool(bool),
    Integer(i128), 
    String(String),
    Block,
    Struct,
    Function,
}

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}


pub struct Parser {
    token_list: Array<Token>,
    ast: Node,
}
