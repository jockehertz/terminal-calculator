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
