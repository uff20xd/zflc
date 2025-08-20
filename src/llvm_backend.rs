use crate::parser::parser::Node;
use crate::parser::parser::NodeType;

pub struct llvm_code {
    source_code: Vec<Vec<char>>,
    ast: Node,
}
