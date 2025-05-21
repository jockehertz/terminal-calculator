use terminal_calculator::parser::{construct_ast, AstNode};
use terminal_calculator::lexer::{tokenise, TokenType};
use terminal_calculator::errors::ParseError;

// Parses a basic expression
#[test]
fn test_parse_expression_basic() {
    let input = "3 + 5";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::BinaryOp {
        operator: TokenType::Addition,
        operand_1: Box::new(AstNode::Number(3.0)),
        operand_2: Box::new(AstNode::Number(5.0)),
    });
}

// Parses an expression with only one number
#[test]
fn test_parse_single_number() {
    let input = "42";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::Number(42.0));
}

// Parses an expression containing operators of different precedence
#[test]
fn test_parse_expression_precedence() {
    let input = "3 + 5 * 2 - 8 / 4";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::BinaryOp {
        operator: TokenType::Subtraction,
        operand_1: Box::new(AstNode::BinaryOp {
            operator: TokenType::Addition,
            operand_1: Box::new(AstNode::Number(3.0)),
            operand_2: Box::new(AstNode::BinaryOp {
                operator: TokenType::Multiplication,
                operand_1: Box::new(AstNode::Number(5.0)),
                operand_2: Box::new(AstNode::Number(2.0)),
            }),
        }),
        operand_2: Box::new(AstNode::BinaryOp {
            operator: TokenType::Division,
            operand_1: Box::new(AstNode::Number(8.0)),
            operand_2: Box::new(AstNode::Number(4.0)),
        }),
    });
}

// Parses an expression with parentheses
#[test]
fn test_parse_expression_with_parentheses() {
    let input = "3 + 5 * (2 - 8)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::BinaryOp {
        operator: TokenType::Addition,
        operand_1: Box::new(AstNode::Number(3.0)),
        operand_2: Box::new(AstNode::BinaryOp {
            operator: TokenType::Multiplication,
            operand_1: Box::new(AstNode::Number(5.0)),
            operand_2: Box::new(AstNode::BinaryOp {
                operator: TokenType::Subtraction,
                operand_1: Box::new(AstNode::Number(2.0)),
                operand_2: Box::new(AstNode::Number(8.0)),
            }),
        }),
    });
}

// Tokenises and parses an expression with implicit multiplication
#[test]
fn test_parse_expression_with_implicit_multiplication() {
    let input = "3(4 + 5)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::BinaryOp {
        operator: TokenType::Multiplication,
        operand_1: Box::new(AstNode::Number(3.0)),
        operand_2: Box::new(AstNode::BinaryOp {
            operator: TokenType::Addition,
            operand_1: Box::new(AstNode::Number(4.0)),
            operand_2: Box::new(AstNode::Number(5.0)),
        }),
    });
}

// Tokenises and parses an expression with a unary operator
#[test]
fn test_parse_expression_with_unary_operator() {
    let input = "-3 + 5";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::BinaryOp {
        operator: TokenType::Addition,
        operand_1: Box::new(AstNode::UnaryOp {
            operator: TokenType::Negation,
            operand: Box::new(AstNode::Number(3.0)),
        }),
        operand_2: Box::new(AstNode::Number(5.0)),
    });
}

// Parses an expression with a unary operator and parentheses
#[test]
fn test_parse_expression_with_unary_operator_and_parentheses() {
    let input = "-(3 + 5)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::UnaryOp {
        operator: TokenType::Negation,
        operand: Box::new(AstNode::BinaryOp {
            operator: TokenType::Addition,
            operand_1: Box::new(AstNode::Number(3.0)),
            operand_2: Box::new(AstNode::Number(5.0)),
        }),
    });
}

// Parses an expression with multiple exponentiations
#[test]
fn test_parse_expression_with_exponentiation() {
    let input = "2 ^ 3 ^ 2";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    assert_eq!(ast, AstNode::BinaryOp {
        operator: TokenType::Exponentiation,
        operand_1: Box::new(AstNode::Number(2.0)),
        operand_2: Box::new(AstNode::BinaryOp {
            operator: TokenType::Exponentiation,
            operand_1: Box::new(AstNode::Number(3.0)),
            operand_2: Box::new(AstNode::Number(2.0)),
        }),
    });
}

// Parses an expression with an unexpected token and returns an error, this should not get past the lexer.
#[test]
fn test_parse_expression_unexpected_token() {
    let input = "3 + 5 @ 2";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = construct_ast(&tokens);
    assert!(ast.is_err());
    assert_eq!(ast.unwrap_err(), ParseError::UnexpectedToken("@".to_string()));
}

// Parses an incomplete expression and returns an error
#[test]
fn test_parse_expression_missing_token_at_end() {
    let input = "3 + ";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = construct_ast(&tokens);
    assert!(ast.is_err());
    assert_eq!(ast.unwrap_err(), ParseError::UnexpectedEndOfInput);
}

// Parses an expression missing a closing parenthesis, and returns an error
#[test]
fn test_parse_expression_missing_closing_parenthesis() {
    let input = "3 + (5 * 2";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = construct_ast(&tokens);
    assert!(ast.is_err());
    assert_eq!(ast.unwrap_err(), ParseError::MissingClosingParenthesis);
}
