use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenType;
use crate::lexer::tokens::Keyword;

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


    fn parse_value(&mut self) -> Node {
        let mut node = Node::new(NodeType::Value);
        let mut token = self.get_current_token()
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));

        let mut child = match token.token_type {
            TokenType::IntegerLiteral(_) => { self.parse_string() },
            TokenType::StringLiteral(_) => { self.parse_integer() },
            TokenType::LeftBracket => { self.parse_list() },
            TokenType::Ident(_) => { self.parse_ident() },
            TokenType::LeftParen => { self.parse_parentheses_value() },
            _ => { panic!("Expected Value and found: {:?}", &token) },
        };

        if let Some(probe_for_expr) = self.get_pnth(1) {
            if probe_for_expr.is_math_operator() { child = self.parse_math_expr(Some(child.clone())) }
            else if probe_for_expr.is_math_operator() { child = self.parse_bool_expr(Some(child.clone())) }
        }

        node.add_child(child);

        node
    }

    fn parse_parentheses_value(&mut self) -> Node {
        let mut token = self.get_current_token()
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));
        let mut child = match token.token_type {
            TokenType::LeftParen => { 
                if let Some(_) = self.get_next_token() {}
                else { panic!("Tokenstream unexpectedly ends!") }
                self.parse_ident() 
            },
            _ => { self.parse_value() },
        };
        child
    }

    fn parse_bool_expr(&mut self, left: Option<Node>) -> Node {
        let mut node = Node::new(NodeType::BoolExpr);
        if let Some(left_node) = left {
            node.add_child(left_node.clone());
        }
        else {
            node.add_child(self.parse_value());
        }
        if let Some(token) = self.get_next_token() { node.add_child(self.parse_bool_operator()) }
        else { panic!("Unexpected end of Program") }
        node.add_child(self.parse_value());
        todo!()
    }

    fn parse_math_expr(&mut self, left: Option<Node>) -> Node {
        let mut node = Node::new(NodeType::MathExpr);
        if let Some(left_node) = left {
            node.add_child(left_node.clone());
        }
        else {
            node.add_child(self.parse_value());
        }
        if let Some(token) = self.get_next_token() { node.add_child(self.parse_math_operator()) }
        else { panic!("Unexpected end of Program") }
        node.add_child(self.parse_value());
        todo!()
    }

    fn parse_math_operator(&mut self) -> Node {
        let current_token = match self.get_current_token() {
            Some(token) => { token },
            None => { panic!("Program Suddenly Ended.") }
        };
        let node_type = match current_token.token_type {
                TokenType::Plus => { NodeType::Plus },
                TokenType::Minus => { NodeType::Minus },
                TokenType::Mult => { NodeType::Mult },
                TokenType::Divide => { NodeType::Divide },
                TokenType::Modulo => { NodeType::Modulo },
            _ => { panic!("Expected Operator, found {:?}", self.get_current_token().unwrap() ) }
        };

        Node::new(node_type)
    }

    fn parse_bool_operator(&mut self) -> Node {
        let current_token = match self.get_current_token() {
            Some(token) => { token },
            None => { panic!("Program Suddenly Ended.") }
        };
        let node_type = match current_token.token_type {
            TokenType::EqualTo => { NodeType::EqualTo },
            TokenType::LesserThan => { NodeType::LesserThan },
            TokenType::GreaterThan => { NodeType::GreaterThan },
            TokenType::GreaterThanOrEqualTo => { NodeType::GreaterThanOrEqualTo },
            TokenType::LesserThanOrEqualTo => { NodeType::LesserThanOrEqualTo },
            _ => { panic!("Expected Operator, found {:?}", self.get_current_token()) }
        };
        Node::new(node_type)
    }

    fn parse_string(&mut self) -> Node {
        let mut token = self.get_current_token()
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));

        let node_type = NodeType::String("".to_owned());
        let mut node = Node::new(node_type);

        node
    }

    fn parse_integer(&mut self) -> Node {
        let mut token = self.get_current_token()
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));
        let node_type = match token.token_type {
            TokenType::IntegerLiteral(value) => { NodeType::Integer(value) },
            _ => { panic!("Expected Integer, found {:?}", token) },
        };
        let mut node = Node::new(node_type);

        node
    }

    fn parse_ident(&mut self) -> Node {
        let token = self.get_current_token()
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));
        let ident = match token.token_type {
            TokenType::Ident(string) => { string.clone() },
            _ => { panic!("Expected Integer, found {:?}", token) },
        };
        let node = Node::new(NodeType::Ident(ident));

        node
    }

    fn parse_list(&mut self) -> Node {
        let mut node = Node::new(NodeType::List);
        let mut token = self.get_current_token()
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));
        token = (match token.token_type {
            TokenType::LeftBracket => { self.get_next_token() },
            _ => { panic!("Expected Left Bracket, found {:?}", token) },
        })
            .expect(&format!(
                "Token Stream ends unexpectedly at {}:{}",
                self.tokens[self.tokens.len() - 1].pos,
                self.tokens[self.tokens.len() - 1].line
            ));

        loop {
            match token.token_type {
                TokenType::RightBracket => {
                    break;
                },
                _ => { node.add_child(self.parse_value()) }
            }
        }

        node
    }

    fn parse_function_call(&mut self) -> Node {
        todo!()
    }

    fn parse_function_dec(&mut self) -> Node {
        todo!()
    }

    fn parse_enum(&mut self) -> Node {
        todo!()
    }

    fn parse_struct(&mut self) -> Node {
        todo!()
    }

    fn parse_get(&mut self) -> Node {
        todo!()
    }

    pub fn parse(&mut self) -> Node {
        let mut ast = Node::new(NodeType::Program);
        let mut child;
        let mut wrapped_token = self.get_current_token();

        loop {

            if let Some(token) = wrapped_token {
                child = match token.token_type {
                    TokenType::Keyword(value) => { match value {
                        Keyword::Struct => { self.parse_struct() },
                        Keyword::Function => { self.parse_function_dec() },
                        Keyword::Get => { self.parse_get() },
                        _ => { panic!("Unexpected Token: {}:{}", token.line + 1, token.pos) }
                    }},
                    _ => { panic!("Unexpected Token: {:?} {}:{}", token, token.line + 1, token.pos) },
                }
            }
            else {
                break;
            }

            wrapped_token = self.get_next_token();
        }

        self.ast = ast.clone();
        ast
    }
}
