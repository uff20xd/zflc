use crate::tokenizer::tokens::Token;
use crate::tokenizer::tokens::TokenType;

#[derive(Clone, Debug)]
pub enum NodeType {

    // { (FunctionDeclaration | Type) }
    Program,

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

    // { Expr+ }
    Block,

    //{ Ident ~ (Ident ~ Type)+ }
    Struct,

    // { Ident ~ (Ident ~ Ident)+ ~ Ident ~ Block }
    FunctionDeclaration,

    // { Ident ~ (Ident ~ Ident)+}
    FunctionCall,

    // { BoolExpr ~ Block }
    If,

    // { Ident ~ BoolExpr ~ Expr ~ Block }
    For,

    // { Ident }
    Type,

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
    token_list: Vec<Token>,
    token_pointer: usize,
    token_list_len: usize,
    ast: Node,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Self {
            token_list,
            token_list_len: token_list.len(),
            token_pointer: 0,
            ast: Node::new(),
        }
    }

    #[inline(always)]
    fn get_next_token(self) -> Option<Token> {
        if self.token_pointer >= self.token_list_len - 1 {
            None
        }
        self.token_pointer += 1;
        Some(self.token_list[self.token_pointer])
    }

    #[inline(always)]
    fn get_current_token(self) -> Option<Token> {
        if self.token_pointer >= self.token_list_len {
            None
        }
        Some(self.token_list[self.token_pointer])
    }


    fn parse_value(&mut self) -> Node {
        let mut node = Node::new(NodeType::Value);
        let mut token = self.get_current_token()
            .expect(format!("Token Stream ends unexpectedly at {}:{}", self.tokens[self.tokens.len( - 1)].pos, self.tokens[self.tokens.len() - 1].line))

        let child = match token.token_type {
            TokenType::IntegerLiteral => {self.parse_string()},
            TokenType::StringLiteral => {self.parse_integer()},
            _ => {},
        }
        token.add_child(child);

        node
    }

    fn parse_string() -> Node {
        let mut token = self.get_current_token();
        let mut node = Node::new(NodeType::Value);

        node
    }

    pub fn parse_ident(&mut self) -> Self {

    }

    pub fn parse() -> Node {
        let mut ast = Node::new();

        loop {

            if let Some(token) = self.get_next_token() {

            }
            else {
                break;
            }
        }

        self.ast = ast.clone();
        ast
    }
}
