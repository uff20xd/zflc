use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenType;

#[derive(Clone, Debug)]
pub enum NodeType {

    // { (FunctionDeclaration | Type) }
    Program,

    // Only used for parsing purposes
    None,

    // { Bool | Integer | Float | String | List | Expr | Ident }
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

    // { Value | Block | FunctionCall | VarDecleration }
    Expr,

    // { Value ~ MathOperator ~ Value }
    MathExpr,

    // { Plus | Minus | Mult | Divide }
    MathOperator,

    Plus,
    Minus,
    Mult,
    Divide,
    Modulo,

    // { Value ~ MathOperator ~ Value }
    BoolExpr,

    // { EqualTo | LesserThan | GreaterThan | GreaterThanOrEqualTo | LesserThanOrEqualTo }
    BoolOperator,

    EqualTo,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqualTo,
    LesserThanOrEqualTo,

    // { Expr+ }
    Block,

    // { ident }
    VarDecleration,

    //{ Ident ~ (Ident ~ Type)+ }
    Struct,

    //{ Ident | (Ident ~ TypeMod+) }
    Type,

    //{ Mut | Pointer | Constans | Static }
    TypeMode,

    Mut,
    Pointer,
    Constans,
    Static,

    // { Ident ~ FunctionInputs ~ Ident ~ Block }
    FunctionDeclaration,

    // { (Ident ~ Ident)+ }
    FunctionInputs,

    // { Ident ~ (Value)+ }
    FunctionCall,

    // { BoolExpr ~ Block }
    If,

    // { Ident ~ BoolExpr ~ Expr ~ Block }
    For,

    // { }
    Ident(String),
}

#[derive(Clone, Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}
 
impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            children: Vec::new(),
        }
    }
    #[inline(always)]
    pub fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }

    pub fn add_child(&mut self, value: Node) {
        self.children.push(value);
    }

    pub fn get_children(&self) -> Vec<Node> {
        self.children.clone()
    }
}


#[derive(Clone, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    token_pointer: usize,
    token_list_len: usize,
    ast: Node,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            token_list_len: tokens.len(),
            token_pointer: 0,
            ast: Node::new(NodeType::Program),
        }
    }

    #[inline(always)]
    fn get_pnth(&mut self, plus: usize) -> Option<Token> {
        if self.token_pointer + plus >= self.token_list_len { return None; }

        Some(self.tokens[self.token_pointer + plus].clone())
    }

    #[inline(always)]
    fn get_next_token(&mut self) -> Option<Token> {
        if self.token_pointer >= self.token_list_len - 1 {
            return None;
        }
        self.token_pointer += 1;
        Some(self.tokens[self.token_pointer].clone())
    }

    #[inline(always)]
    fn get_current_token(&self) -> Option<Token> {
        if self.token_pointer >= self.token_list_len {
            return None;
        }
        Some(self.tokens[self.token_pointer].clone())
    }
}
