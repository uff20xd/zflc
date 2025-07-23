use crate::tokenizer::tokens::Token;
use crate::tokenizer::tokens::TokenType;

#[derive(Clone, Debug)]
pub enum NodeType {
    // { Bool | Integer | Float | String | List | Expr }
    Value,

    // { }
    Bool(bool),

    // { }
    Integer(i128), 

    // { }
    Float,

    // { }
    String(String),

    // { Value+ }
    List,

    // { MathExpr | Block | BoolExpr | FunctionCall | VarDecleration }
    Expr,

    // { Value ~ MathOperator ~ Value }
    MathExpr,

    // { Plus | Minus | Mult | Divide }
    MathOperator,

    // { Value ~ MathOperator ~ Value }
    BoolExpr,

    // { EqualTo | LesserThan | GreaterThan | GreaterOrEqualTo | LesserThanOrEqualTo }
    BoolOperator,

    //
    Block,

    Struct,

    Function,

    If,

    For,
}

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}


pub struct Parser {
    token_list: Array<Token>,
    ast: Node,
}
