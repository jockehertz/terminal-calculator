use crate::parser::AstNode;
use crate::errors::{EvaluationError};
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, f64>,
}

#[derive(PartialEq, Debug)]
pub enum EvalResult {
    Value(f64),
    Assignment(String, f64),
}

pub const CONSTS: [&str; 6] = ["pi", "e", "phi", "tau", "sqrt2", "sqrt3"];

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: f64) -> Option<EvaluationError> {
        if CONSTS.contains(&name.as_str()) {
            return Some(EvaluationError::CannotAssignAConstant(name));
        }
        self.variables.insert(name, value);
        return None;
    }

    pub fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }

    pub fn init_consts(&mut self) {
        self.variables.insert("pi".to_string(), std::f64::consts::PI);
        self.variables.insert("e".to_string(), std::f64::consts::E);
        self.variables.insert("phi".to_string(), 1.618033988749895);
        self.variables.insert("tau".to_string(), std::f64::consts::TAU);
        self.variables.insert("sqrt2".to_string(), 1.4142135623730951);
        self.variables.insert("sqrt3".to_string(), 1.7320508075688772);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    Sin,
    Cos,
    Tan,
}

impl AstNode {
    pub fn evaluate(&self, environment: &mut Environment) -> Result<EvalResult, EvaluationError> {
        match self {
            AstNode::Number(value) => Ok(EvalResult::Value(*value)),
            AstNode::UnaryOp {operator, operand} => {
                let a: f64 = match operand.evaluate(environment) {
                    Ok(EvalResult::Value(result)) => result,
                    Ok(EvalResult::Assignment(name, value)) => return Ok(EvalResult::Assignment(name, value)),
                    Err(error) => return Err(error),
                };

                match operator.apply_unary(a) {
                    Ok(result) => Ok(EvalResult::Value(result)),
                    Err(error) => Err(error),
                }
            }
            AstNode::BinaryOp {operator, operand_1, operand_2} => {
                let a: f64 = match operand_1.evaluate(environment) {
                    Ok(EvalResult::Value(result)) => result,
                    Ok(EvalResult::Assignment(name, value)) => return Ok(EvalResult::Assignment(name, value)),
                    Err(error) => return Err(error),
                };

                let b: f64 = match operand_2.evaluate(environment) {
                    Ok(EvalResult::Value(result)) => result,
                    Ok(EvalResult::Assignment(name, value)) => return Ok(EvalResult::Assignment(name, value)),
                    Err(error) => return Err(error),
                };

                match operator.apply_binary(a, b) {
                    Ok(result) => Ok(EvalResult::Value(result)),
                    Err(error) => Err(error),
                }
            }
            AstNode::Function {function, args} => {
                let a: f64 = match args.evaluate(environment) {
                    Ok(EvalResult::Value(result)) => result,
                    Ok(EvalResult::Assignment(name, value)) => return Ok(EvalResult::Assignment(name, value)),
                    Err(error) => return Err(error),
                };
                
                match function.apply_function(a) {
                    Ok(result) => Ok(EvalResult::Value(result)),
                    Err(error) => Err(error),
                }
            }
            AstNode::Assignment {name, value} => {
                let a: f64 = match value.evaluate(environment) {
                    Ok(EvalResult::Value(result)) => result,
                    Ok(EvalResult::Assignment(name, value)) => return Ok(EvalResult::Assignment(name, value)),
                    Err(error) => return Err(error),
                };
                match environment.set_variable(name.clone(), a) {
                    Some(error) => return Err(error),
                    None => (),
                };
                Ok(EvalResult::Assignment(name.clone(), a))
            }
            AstNode::Variable(name) => {
                match environment.get_variable(name) {
                    Some(value) => Ok(EvalResult::Value(*value)),
                    None => Err(EvaluationError::UndefinedVariable(name.clone())),
                }
            }
        }
    }
}
