use terminal_calculator::lexer::{tokenise, Token, TokenType};
use terminal_calculator::evaluator::Function;

// Tokenises a basic input
#[test]
fn test_tokeniser_basic() {
    let input = "3 + 5 * (2 - 8)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "3".to_string()},
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "5".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Subtraction, lexeme: "-".to_string() },
        Token { token_type: TokenType::Number, lexeme: "8".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];

    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };

    assert_eq!(tokens, expected_tokens);
}

// Tokenises a string with a single number
#[test]
fn test_tokeniser_single_number() {
    let input = "42";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "42".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

// Tokenises a string with implicit multiplication
#[test]
fn test_tokeniser_with_implicit_multiplication() {
    let input = "3(4 + 5)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "4".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "5".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokeniser_function() {
    let input = "sin(2)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokeniser_function_with_implicit_multiplication() {
    let input = "2cos(0)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Keyword(Function::Cos), lexeme: "cos".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "0".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokeniser_identifier_with_implicit_multiplication() {
    let input = "2x";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "x".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

// Tokenises an expression with an unexpected token and returns an error.
#[test]
#[should_panic(expected = "LexerError: InvalidIdentifier(\"@\")")]
fn test_tokenise_expression_unexpected_token() {
    let input = "3 + 5 @ 2";
    let _tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
}

#[test]
fn test_tokenise_function() {
    let input = "sin(3)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_nested_functions() {
    let input = "sin(cos(3))";
    let expected_tokens = vec![
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Keyword(Function::Cos), lexeme: "cos".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_function_with_implicit_multiplication() {
    let input = "2sin(3)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_function_with_negation() {
    let input = "-sin(3)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Negation, lexeme: "-".to_string() },
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_function_with_implicit_multiplication_and_negation() {
    let input = "-2sin(3)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Negation, lexeme: "-".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_function_with_implicit_multiplication_and_negation_with_parentheses() {
    let input = "-2(sin(3))";
    let expected_tokens = vec![
        Token { token_type: TokenType::Negation, lexeme: "-".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_function_with_implicit_multiplication_and_negation_with_parentheses_and_addition() {
    let input = "-2(sin(3) + 5)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Negation, lexeme: "-".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "5".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_unicode_identifier() {
    let input = "π + 5";
    let expected_tokens = vec![
        Token { token_type: TokenType::Identifier, lexeme: "π".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "5".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_in_operations() {
    let input = "x + y";
    let expected_tokens = vec![
        Token { token_type: TokenType::Identifier, lexeme: "x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifier_with_implicit_multiplication() {
    let input = "2x + 3y";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifier_with_implicit_multiplication_and_parentheses() {
    let input = "2(x + y)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifier_with_implicit_multiplication_and_parentheses_and_function() {
    let input = "2(sin(x + y))";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Keyword(Function::Sin), lexeme: "sin".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_with_underscore_in_operations() {
    let input = "x_1 + y_2";
    let expected_tokens = vec![
        Token { token_type: TokenType::Identifier, lexeme: "x_1".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y_2".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_with_underscore_with_implicit_multiplication() {
    let input = "2x_1 + 3y_2";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "x_1".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y_2".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_with_underscore_with_implicit_multiplication_and_parentheses() {
    let input = "2(x_1 + y_2)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "x_1".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "y_2".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_beginning_with_underscore() {
    let input = "_x + _y";
    let expected_tokens = vec![
        Token { token_type: TokenType::Identifier, lexeme: "_x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "_y".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_beginning_with_underscore_with_implicit_multiplication() {
    let input = "2_x + 3_y";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "_x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "_y".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_identifiers_beginning_with_underscore_with_implicit_multiplication_and_parentheses() {
    let input = "2(_x + _y)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "_x".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "_y".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_underscore_outside_parenthesis() {
    let input = "2_(3 + 5)";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Identifier, lexeme: "_".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::LeftParenthesis, lexeme: "(".to_string() },
        Token { token_type: TokenType::Number, lexeme: "3".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "5".to_string() },
        Token { token_type: TokenType::RightParenthesis, lexeme: ")".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_decimal_number() {
    let input = "3.14 + 2.71";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "3.14".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2.71".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_decimal_leading_dot() {
    let input = ".5 + 2";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: ".5".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_decimal_trailing_dot() {
    let input = "3. + 2";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "3.".to_string() },
        Token { token_type: TokenType::Addition, lexeme: "+".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenise_decimal_with_exponent() {
    let input = "3.14*10^2";
    let expected_tokens = vec![
        Token { token_type: TokenType::Number, lexeme: "3.14".to_string() },
        Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
        Token { token_type: TokenType::Number, lexeme: "10".to_string() },
        Token { token_type: TokenType::Exponentiation, lexeme: "^".to_string() },
        Token { token_type: TokenType::Number, lexeme: "2".to_string() },
    ];
    let tokens = match tokenise(input.to_string()) {
        Ok(result) => result,
        Err(error) => panic!("LexerError: {:?}", error),
    };
    assert_eq!(tokens, expected_tokens);
}