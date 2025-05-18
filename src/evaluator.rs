use crate::parser::AstNode;
use crate::errors::EvaluationError;

impl AstNode {
    pub fn evaluate(&self) -> Result<f64, EvaluationError> {
        match self {
            AstNode::Number(value) => Ok(*value),
            AstNode::UnaryOp {operator, operand} => {
                let a: f64 = match operand.evaluate() {
                    Ok(result) => result,
                    Err(error) => return Err(error),
                };

                match operator.apply_unary(a) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(error),
                }
            }
            AstNode::BinaryOp {operator, operand_1, operand_2} => {
                let a: f64 = match operand_1.evaluate() {
                    Ok(result) => result,
                    Err(error) => return Err(error),
                };

                let b: f64 = match operand_2.evaluate() {
                    Ok(result) => result,
                    Err(error) => return Err(error),
                };

                match operator.apply_binary(a, b) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(error),
                }
            }
        }
    }
}
