use terminal_calculator::lexer::tokenise;
use terminal_calculator::parser::construct_ast;

// Evaluate a basic AST
#[test]
fn test_evaluate_basic() {
    let input = "3 + 5";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 8.0);
}

// Evaluate an expression with implicit multiplication
#[test]
fn test_evaluate_implicit_multiplication() {
    let input = "3(4 + 5)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };

    assert_eq!(result, 27.0);
}

// Evaluate an expression with different operator precedence
#[test]
fn test_evaluate_precedence() {
    let input = "3 + 5 * 2 - 8 / 4";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 11.0);
}

// Evaluate an expression with parentheses
#[test]
fn test_evaluate_parentheses() {
    let input = "(3 + 5) * 2";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 16.0);
}

// Evaluate an expression with exponentiation
#[test]
fn test_evaluate_exponentiation() {
    let input = "2 ^ 3 + 1";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 9.0);
}

// Evaluate an expression with unary negation
#[test]
fn test_evaluate_unary_negation() {
    let input = "-3 + 5";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 2.0);
}

// Evaluate an expression with a single number
#[test]
fn test_evaluate_single_number() {
    let input = "42";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 42.0);
}

#[test]
fn test_evaluate_division_by_zero() {
    let input = "1 / 0";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = ast.evaluate();
    assert!(result.is_err());
}

// Evaluate an expression with a function
#[test]
fn test_evaluate_function() {
    let input = "sin(0)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 0.0);
}
// Evaluate an expression with a function and arguments
#[test]
fn test_evaluate_function_with_args() {
    let input = "sin(0) + cos(0)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, 1.0);
}

// Evaluate an expression with negation
#[test]
fn test_evaluate_negation() {
    let input = "-(3 + 5)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, -8.0);
}

// Evaluate an expression with a negation of a function
#[test]
fn test_evaluate_negation_of_function() {
    let input = "-cos(0)";
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    let ast = match construct_ast(&tokens) {
        Ok(result) => result,
        Err(error) => panic!("ParseError: {:?}", error),
    };
    let result = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => panic!("EvaluationError: {:?}", error),
    };
    assert_eq!(result, -1.0);
}