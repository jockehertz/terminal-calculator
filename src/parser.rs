use crate::lexer::{Token, TokenType};
use crate::errors::{ParseError};

#[derive(Debug, PartialEq)]
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
    Function {
        function: TokenType,
        args: Box<AstNode>,
    },
    Assignment {
        name: String,
        value: Box<AstNode>,
    },
    Variable(String),
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
            TokenType::Keyword(_) => return 4,
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
        TokenType::Number => {
            let number = match tokens[pos].lexeme.parse::<f64>() {
                Ok(num) => num,
                Err(_) => return Err(ParseError::UnexpectedToken(tokens[pos].lexeme.clone())),
            };
            return Ok((AstNode::Number(number), pos + 1));
        }
        
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
                    operand: Box::new(operand),
                },
                new_position,
            ))
        },

        TokenType::Keyword(function) => {
            let (value, new_position) = match parse_primary(tokens, pos + 1) {
                Ok(result) => result,
                Err(error) => return Err(error),
            };
            Ok((
                    AstNode::Function {
                        function: TokenType::Keyword(function.clone()),
                        args: Box::new(value),
                    },
                    new_position,
            ))

        }

        TokenType::Identifier => {
            let name = tokens[pos].lexeme.clone();
            if pos + 1 < tokens.len() && tokens[pos + 1].token_type == TokenType::Equals {
                let (value, new_position) = match parse_expression(tokens, pos + 2, 0) {
                    Ok(result) => result,
                    Err(error) => return Err(error),
                };
                Ok((
                    AstNode::Assignment {
                        name,
                        value: Box::new(value),
                    },
                    new_position,
                ))
            } else {
                Ok((AstNode::Variable(name), pos + 1))
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::Function;

    fn num(n: &str) -> Token {
        Token { token_type: TokenType::Number, lexeme: n.to_string() }
    }
    fn op(token_type: TokenType, lexeme: &str) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
        }
    }

    // Parses a number in parentheses
    #[test]
    fn test_parse_primary_number() {
        let tokens = vec![num("42")];
        let (ast, pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(ast, AstNode::Number(42.0));
        assert_eq!(pos, 1);
    }

    // 
    #[test]
    fn test_parse_primary_parenthesis() {
        let tokens = vec![
            op(TokenType::LeftParenthesis, "("),
            num("7"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (ast, pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(ast, AstNode::Number(7.0));
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_parse_expression_addition() {
        let tokens = vec![
            num("1"),
            op(TokenType::Addition, "+"),
            num("2"),
        ];
        let (ast, pos) = match parse_expression(&tokens, 0, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(
            ast,
            AstNode::BinaryOp {
                operator: TokenType::Addition,
                operand_1: Box::new(AstNode::Number(1.0)),
                operand_2: Box::new(AstNode::Number(2.0)),
            }
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_parse_expression_precedence() {
        let tokens = vec![
            num("1"),
            op(TokenType::Addition, "+"),
            num("2"),
            op(TokenType::Multiplication, "*"),
            num("3"),
        ];
        let (ast, pos) = match parse_expression(&tokens, 0, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(
            ast,
            AstNode::BinaryOp {
                operator: TokenType::Addition,
                operand_1: Box::new(AstNode::Number(1.0)),
                operand_2: Box::new(AstNode::BinaryOp {
                    operator: TokenType::Multiplication,
                    operand_1: Box::new(AstNode::Number(2.0)),
                    operand_2: Box::new(AstNode::Number(3.0)),
                }),
            }
        );
        assert_eq!(pos, 5);
    }

    #[test]
    fn test_parse_simple_function() {
        let tokens = vec![
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            num("0"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (ast, pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(
            ast,
            AstNode::Function {
                function: TokenType::Keyword(Function::Sin),
                args: Box::new(AstNode::Number(0.0)),
            }
        );
        assert_eq!(pos, 4);
    }

    #[test]
    fn test_parse_function_containing_other_operation() {
        let tokens = vec![
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            num("0"),
            op(TokenType::Addition, "+"),
            num("1"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (ast, pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(
            ast,
            AstNode::Function {
                function: TokenType::Keyword(Function::Sin),
                args: Box::new(AstNode::BinaryOp {
                    operator: TokenType::Addition,
                    operand_1: Box::new(AstNode::Number(0.0)),
                    operand_2: Box::new(AstNode::Number(1.0)),
                }),
            }
        );
        assert_eq!(pos, 6);
    }

    #[test]
    fn test_parse_function_containing_incomplete_expression() {
        let tokens = vec![
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            num("0"),
            op(TokenType::Addition, "+"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let result = parse_primary(&tokens, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_expression_with_function_and_unary_negation() {
        let tokens = vec![
            op(TokenType::Negation, "-"),
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            num("1"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (ast, pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(
            ast,
            AstNode::UnaryOp {
                operator: TokenType::Negation,
                operand: Box::new(AstNode::Function {
                    function: TokenType::Keyword(Function::Sin),
                    args: Box::new(AstNode::Number(1.0)),
                }),
            }
        );
        assert_eq!(pos, 5);
    }

    #[test]
    fn test_parse_nested_functions() {
        let tokens = vec![
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            op(TokenType::Keyword(Function::Cos), "Cos"),
            op(TokenType::LeftParenthesis, "("),
            num("0"),
            op(TokenType::RightParenthesis, ")"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (ast, pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
        assert_eq!(
            ast,
            AstNode::Function {
                function: TokenType::Keyword(Function::Sin),
                args: Box::new(AstNode::Function {
                    function: TokenType::Keyword(Function::Cos),
                    args: Box::new(AstNode::Number(0.0)),
                }),
            }
        );
        assert_eq!(pos, 7);
    }

    #[should_panic(expected = "ParseError: UnexpectedToken(\")\"")]
    #[test]
    fn test_parse_function_with_empty_parentheses() {
        let tokens = vec![
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (_ast, _pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
    }

    #[should_panic(expected = "ParseError: UnexpectedToken(\"+\")")]
    #[test]
    fn test_parse_function_with_unexpected_token() {
        let tokens = vec![
            op(TokenType::Keyword(Function::Sin), "Sin"),
            op(TokenType::LeftParenthesis, "("),
            num("0"),
            op(TokenType::Addition, "+"),
            op(TokenType::Addition, "+"),
            num("1"),
            op(TokenType::RightParenthesis, ")"),
        ];
        let (_ast, _pos) = match parse_primary(&tokens, 0) {
            Ok(result) => result,
            Err(error) => panic!("ParseError: {:?}", error),
        };
    }

    #[test]
    fn test_parse_primary_error() {
        let tokens = vec![];
        let result = parse_primary(&tokens, 0);
        assert!(result.is_err());
    }
}
