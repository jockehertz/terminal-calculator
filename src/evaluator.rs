use crate::lexer::TokenType;
use crate::parser::AstNode;

// Unary operations (arity 1, such as negation)
pub trait UnaryOperation {
    fn operate(&self, operand: f64) -> f64;
    fn is_unary(&self) -> bool {
        return true;
    }
    fn is_binary(&self) -> bool {
        return false;
    }
}

// Binary operations (arity 2, such as addition)
pub trait BinaryOperation {
    fn operate(&self, operand_1: f64, operand_2: f64) -> f64;
    fn is_unary(&self) -> bool {
        return false;
    }
    fn is_binary(&self) -> bool {
        return true;
    }
}

impl UnaryOperation for TokenType {
    fn operate(&self, operand: f64) -> f64 {
        match self {
            TokenType::Exponentiation |
                TokenType::Multiplication |
                TokenType::Division |
                TokenType::Addition |
                TokenType::Subtraction => return 0.0,
            TokenType::Negation => return 0.0 - operand,
            _ => panic!("Not an operator"),
        }
    }
}

impl BinaryOperation for TokenType {
    fn operate(&self, operand_1: f64, operand_2: f64) -> f64 {
        match self {
            TokenType::Addition => return operand_1 + operand_2,
            TokenType::Subtraction => return operand_1 - operand_2,
            TokenType::Multiplication => return operand_1 * operand_2,
            TokenType::Division => {
                if operand_2 != 0.0 {
                    return operand_1 / operand_2;
                } else {
                    panic!("Division by zero");
                }
            }
            TokenType::Exponentiation => return operand_1.powf(operand_2),
            TokenType::Negation => panic!("Not a binary operation."),
            _ => panic!("Not an operator"),
        }
    }
}

impl AstNode {
    pub fn evaluate(self) -> f64 {
        match self {
            AstNode::Number(value) => value,
            AstNode::UnaryOp {operator: operator, operand: operand } => {
                let a: f64 = operand.evaluate();
                UnaryOperation::operate(&operator, a)
            }
            AstNode::BinaryOp {operator: operator, operand_1: operand_1, operand_2: operand_2 } => {
                let a: f64 = operand_1.evaluate();
                let b: f64 = operand_2.evaluate();
                BinaryOperation::operate(&operator, a, b)
            }
        }
    }
}
