use crate::lexer::{Token, TokenType};

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

fn parse_expression(tokens: &Vec<Token>, pos: usize, min_precedence: u8) -> (AstNode, usize) {
    let (mut left, mut pos) = parse_primary(tokens, pos);

    while pos < tokens.len() {
        let operator = &tokens[pos];
        let precedence = operator.token_type.get_precedence();
        if precedence < min_precedence || precedence == 0 { break; }

        let next_min_precedence = if operator.token_type.is_right_associative() {
            precedence 
        } else {
            precedence + 1 
        };

        let (right, new_pos) = parse_expression(tokens, pos + 1, next_min_precedence);

        left = AstNode::BinaryOp {
            operator: operator.token_type.clone(),
            operand_1: Box::new(left),
            operand_2: Box::new(right),
        };

        pos = new_pos;
    }

    (left, pos)
}

fn parse_primary(tokens: &Vec<Token>, pos: usize) -> (AstNode, usize) {
    if pos >= tokens.len() {
        panic!("Unexpected end of input");
    }

    match &tokens[pos].token_type {
        TokenType::Number => (AstNode::Number(tokens[pos].lexeme.parse::<f64>().unwrap()), pos + 1),
        TokenType::LeftParenthesis => {
            let (expression, new_pos) = parse_expression(tokens, pos + 1, 0);
            
            if new_pos >= tokens.len() || tokens[new_pos].token_type != TokenType::RightParenthesis {
                panic!("Missing closing parenthesis");
            }

            (expression, new_pos + 1)
        },
        TokenType::Negation => {
            let (operand, new_position) = parse_primary(tokens, pos + 1);
            (
                AstNode::UnaryOp {
                    operator: TokenType::Negation,
                    operand: Box::new(operand)
                },
                new_position,
            )
        },
        _ => panic!("Should not be handled here"),
    }
}

pub fn construct_ast(tokens: &Vec<Token>) -> AstNode {
    let (ast, pos) = parse_expression(tokens, 0, 0);

    if pos < tokens.len() {
        panic!("Unexpected tokens at the end.");
    }

    ast
}
