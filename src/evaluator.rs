use crate::parser::AstNode;
use crate::errors::{EvaluationError};
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, f64>,
}

pub enum EvalResult {
    Value(f64),
    Assignment(String, f64),
    Error(EvaluationError),
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
    pub fn evaluate(&self, environment: &mut Environment) -> EvalResult {
        match self {
            AstNode::Number(value) => EvalResult::Value(*value),
            AstNode::UnaryOp {operator, operand} => {
                let a: f64 = match operand.evaluate(environment) {
                    EvalResult::Value(result) => result,
                    EvalResult::Assignment(name, value) => return EvalResult::Assignment(name, value),
                    EvalResult::Error(error) => return EvalResult::Error(error),
                };

                match operator.apply_unary(a) {
                    Ok(result) => EvalResult::Value(result),
                    Err(error) => EvalResult::Error(error),
                }
            }
            AstNode::BinaryOp {operator, operand_1, operand_2} => {
                let a: f64 = match operand_1.evaluate(environment) {
                    EvalResult::Value(result) => result,
                    EvalResult::Assignment(name, value) => return EvalResult::Assignment(name, value),
                    EvalResult::Error(error) => return EvalResult::Error(error),
                };

                let b: f64 = match operand_2.evaluate(environment) {
                    EvalResult::Value(result) => result,
                    EvalResult::Assignment(name, value) => return EvalResult::Assignment(name, value),
                    EvalResult::Error(error) => return EvalResult::Error(error),
                };

                match operator.apply_binary(a, b) {
                    Ok(result) => EvalResult::Value(result),
                    Err(error) => EvalResult::Error(error),
                }
            }
            AstNode::Function {function, args} => {
                let a: f64 = match args.evaluate(environment) {
                    EvalResult::Value(result) => result,
                    EvalResult::Assignment(name, value) => return EvalResult::Assignment(name, value),
                    EvalResult::Error(error) => return EvalResult::Error(error),
                };
                
                match function.apply_function(a) {
                    Ok(result) => EvalResult::Value(result),
                    Err(error) => EvalResult::Error(error),
                }
            }
            AstNode::Assignment {name, value} => {
                let a: f64 = match value.evaluate(environment) {
                    EvalResult::Value(result) => result,
                    EvalResult::Assignment(name, value) => return EvalResult::Assignment(name, value),
                    EvalResult::Error(error) => return EvalResult::Error(error),
                };
                match environment.set_variable(name.clone(), a) {
                    Some(error) => return EvalResult::Error(error),
                    None => (),
                };
                EvalResult::Assignment(name.clone(), a)
            }
            AstNode::Variable(name) => {
                match environment.get_variable(name) {
                    Some(value) => EvalResult::Value(*value),
                    None => EvalResult::Error(EvaluationError::UndefinedVariable(name.clone())),
                }
            }
        }
    }
}
