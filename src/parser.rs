use crate::lexer::{Token, TokenType};
use crate::errors::{ParseError};

pub enum AstNode {
    Number(f64),
    UnaryOp {
        operator: TokenType,
        operand: Box<AstNode>,
    },
    BinaryOp {
        operator: TokenType,
        operand_1: Box<AstNode>,
        operand_2: Box<AstNode>,
    },
}

trait Operator {
    fn get_precedence(&self) -> u8;
    fn is_right_associative(&self) -> bool {
        return false;
    }
}

impl Operator for TokenType {
    fn get_precedence(&self) -> u8 {
        match self {
            TokenType::Negation => return 4,
            TokenType::Exponentiation => return 3,
            TokenType::Multiplication => return 2,
            TokenType::Division => return 2,
            TokenType::Addition => return 1,
            TokenType::Subtraction => return 1,
            _ => return 0,
        }
    }

    fn is_right_associative(&self) -> bool {
        match self {
            TokenType::Exponentiation => return true,
            _ => return false,
        }
    }
}

fn parse_expression(tokens: &Vec<Token>, pos: usize, min_precedence: u8) -> Result<(AstNode, usize), ParseError> {
    let (mut left, mut pos) = match parse_primary(tokens, pos) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    while pos < tokens.len() {
        let operator = &tokens[pos];
        let precedence = operator.token_type.get_precedence();
        if precedence < min_precedence || precedence == 0 { break; }

        let next_min_precedence = if operator.token_type.is_right_associative() {
            precedence 
        } else {
            precedence + 1 
        };

        let (right, new_position) = match parse_expression(tokens, pos + 1, next_min_precedence) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        left = AstNode::BinaryOp {
            operator: operator.token_type.clone(),
            operand_1: Box::new(left),
            operand_2: Box::new(right),
        };

        pos = new_position;
    }

    Ok((left, pos))
}

fn parse_primary(tokens: &Vec<Token>, pos: usize) -> Result<(AstNode, usize), ParseError> {
    if pos >= tokens.len() {
        return Err(ParseError::UnexpectedEndOfInput);
    }

    match &tokens[pos].token_type {
        TokenType::Number => Ok((AstNode::Number(tokens[pos].lexeme.parse::<f64>().unwrap()), pos + 1)),
        
        TokenType::LeftParenthesis => {
            let (expression, new_position) = match parse_expression(tokens, pos + 1, 0) {
                Ok(result) => result,
                Err(error) => return Err(error),
            };
            
            if new_position >= tokens.len() || tokens[new_position].token_type != TokenType::RightParenthesis {
                return Err(ParseError::MissingClosingParenthesis);
            }

            Ok((expression, new_position + 1))
        },
        
        TokenType::Negation => {
            let (operand, new_position) = match parse_primary(tokens, pos + 1) {
                Ok(result) => result,
                Err(error) => return Err(error),
            };
            Ok((
                AstNode::UnaryOp {
                    operator: TokenType::Negation,
                    operand: Box::new(operand)
                },
                new_position,
            ))
        },
        
        _ => {
            return Err(ParseError::UnexpectedToken(tokens[pos].lexeme.clone()));
        },
    }
}

pub fn construct_ast(tokens: &Vec<Token>) -> Result<AstNode, ParseError> {
    let (ast, pos) = match parse_expression(tokens, 0, 0) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    if pos < tokens.len() {
        return Err(ParseError::UnexpectedTokensAtEnd);
    }

    Ok(ast)
}
