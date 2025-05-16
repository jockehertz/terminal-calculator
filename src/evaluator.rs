use crate::parser::AstNode;

impl AstNode {
    pub fn evaluate(&self) -> f64 {
        match self {
            AstNode::Number(value) => *value,
            AstNode::UnaryOp {operator, operand} => {
                let a: f64 = operand.evaluate();
                operator.apply_unary(a)
            }
            AstNode::BinaryOp {operator, operand_1, operand_2} => {
                let a: f64 = operand_1.evaluate();
                let b: f64 = operand_2.evaluate();
                operator.apply_binary(a, b)
            }
        }
    }
}
