use crate::errors::{EvaluationError, LexerError};
use libm::{sin, cos, tan};
use std::f64::consts::{PI, FRAC_PI_2};
use crate::evaluator::{Function, CONSTS};
use unicode_ident::{is_xid_start, is_xid_continue};

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    Number,
    Identifier,
    Keyword(Function),
    Equals,

// OPERATORS
    Negation,
    Exponentiation,
    Multiplication,
    Division,
    Addition,
    Subtraction,

    // DELIMITERS
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    // PUNCTUATION
    Exclamation,
    Comma,
    Question,
    Colon,
    Semicolon,
}

impl TokenType {
/*    pub fn is_operator(&self) -> bool {
        match self {
            TokenType::Negation |
            TokenType::Exponentiation |
            TokenType::Multiplication |
            TokenType::Division |
            TokenType::Addition |
            TokenType::Subtraction => true,
            _ => false,
        }
    }
    pub fn is_delimiter(&self) -> bool {
        match self {
            TokenType::LeftParenthesis |
            TokenType::RightParenthesis |
            TokenType::LeftBracket |
            TokenType::RightBracket |
            TokenType::LeftBrace |
            TokenType::RightBrace => true,
            _ => false,
        }
    }
    pub fn is_punctuation(&self) -> bool {
        match self {
            TokenType::Exclamation |
            TokenType::Comma |
            TokenType::Question |
            TokenType::Colon |
            TokenType::Semicolon => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            TokenType::Number => true,
            _ => false,
        }
    }
    pub fn is_identifier(&self) -> bool {
        match self {
            TokenType::Identifier => true,
            _ => false,
        }
    } 
    pub fn is_unary(&self) -> bool {
        match self {
            TokenType::Negation => true,
            _ => false,
        }
    }
    pub fn is_binary(&self) -> bool {
        match self {
            TokenType::Exponentiation |
            TokenType::Multiplication |
            TokenType::Division |
            TokenType::Addition |
            TokenType::Subtraction => true,
            _ => false,
        }
    }
*/
    pub fn apply_unary(&self, operand: f64) -> Result<f64, EvaluationError> {
        match self {
            TokenType::Negation => Ok(-operand),
            _ => return Err(EvaluationError::InvalidOperation),
        }
    }

    pub fn apply_function(&self, value: f64) -> Result<f64, EvaluationError> {
        match self {
            TokenType::Keyword(function) => match function {
                Function::Sin => Ok(sin(value)),// TODO: sin(value)
                Function::Cos => Ok(cos(value)),// TODO: cos(value)
                Function::Tan => {
                    let k = value * (2.0 / PI);
                    if ((value - k * FRAC_PI_2).abs() < 1e-10) && (k as i64 % 2 != 0) { 
                        Err(EvaluationError::Undefined)
                    } else {
                        Ok(tan(value))
                    }
                }
            }
            _ => return Err(EvaluationError::NotAFunction)
        }
    }
    pub fn apply_binary(&self, operand_1: f64, operand_2: f64) -> Result<f64, EvaluationError> {
        match self {
            TokenType::Exponentiation => Ok(operand_1.powf(operand_2)),
            TokenType::Multiplication => Ok(operand_1 * operand_2),
            TokenType::Division => {
                if operand_2 != 0.0 {
                    return Ok(operand_1 / operand_2);
                } else {
                    return Err(EvaluationError::DivisionByZero);
                }
            }
            TokenType::Addition => Ok(operand_1 + operand_2),
            TokenType::Subtraction => Ok(operand_1 - operand_2),
            _ => return Err(EvaluationError::InvalidOperation),
        }
    }
}

trait TokenVector {
    fn push_word(&mut self, word: &str) -> Option<LexerError>;
}

impl TokenVector for Vec<Token> {
    fn push_word(&mut self, word: &str) -> Option<LexerError> {
        let mut split_index = 0;
        let mut decimal_found = false;

        for (i, c) in word.char_indices() {
            if c.is_ascii_digit() {
                split_index = i + c.len_utf8();
            } else if c == '.' && !decimal_found {
                decimal_found = true;
                split_index = i + c.len_utf8();
            } else {
                break;
            }
        }
        let (number_part, rest) = word.split_at(split_index);
        if !rest.is_empty() {
            let token_type = match get_token_type(&rest) {
                Ok(token_type) => token_type,
                Err(error) => return Some(error),
            };                
            if matches!(token_type, TokenType::Identifier | TokenType::Keyword(_)) && !rest.is_empty() {
                if let Some(prev) = self.last() {
                    if !prev.lexeme.is_empty() {
                        match prev.token_type {
                            TokenType::Number
                            | TokenType::Identifier
                            | TokenType::RightParenthesis
                            | TokenType::RightBracket
                            | TokenType::RightBrace => self.push(
                                Token::new(TokenType::Multiplication, String::from("*"))
                            ),
                            _ => (),
                        }
                    }
                };
            }
        }
        
        if !number_part.is_empty() {
            self.push(Token::new(TokenType::Number, number_part.to_string()));
            if !rest.is_empty() {
                let rest_token_type = match get_token_type(&rest) {
                    Ok(token_type) => token_type,
                    Err(error) => return Some(error),
                };
                self.push(Token::new(TokenType::Multiplication, String::from("*")));
                if CONSTS.contains(&rest.to_lowercase().as_str()) {
                    self.push(Token::new(rest_token_type, rest.to_lowercase().to_string()));
                } else {
                    self.push(Token::new(rest_token_type, rest.to_string()));
                }
            }
        } else {
            let token_type = match get_token_type(&word) {
                Ok(token_type) => token_type,
                Err(error) => return Some(error),
            };
            if CONSTS.contains(&word.to_lowercase().as_str()) {
                self.push(Token::new(token_type, word.to_lowercase().to_string()));
            } else {
                self.push(Token::new(token_type, word.to_string()));
            }
        }

        return None;
        
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}

pub fn tokenise(string: String) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = vec![];
    let mut word = String::new();

    for char in string.chars() {
        match char {
            ' ' | '\n' | '\t' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                word.clear();
            }

            // EQUALS (ASSIGNMENT)
            '=' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Equals, char.to_string()));
                word.clear();
            }

            //OPERATORS
            '^' => {
                if !word.is_empty() {
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Exponentiation, char.to_string()));
                word.clear();
            }
            '*' => {
                if !word.is_empty() {
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Multiplication, char.to_string()));
                word.clear();
            }
            '/' => {
                if !word.is_empty() {
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Division, char.to_string()));
                word.clear();
            } 
            '+' => {
                if !word.is_empty() {
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Addition, char.to_string()));
                word.clear();
            }
            '-' => {
                if !word.is_empty() {
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) | Ok(TokenType::Identifier) => tokens.push(
                            Token::new(TokenType::Subtraction, char.to_string())
                            ),
                        Err(error) => return Err(error),
                        _ => tokens.push(Token::new(TokenType::Negation, char.to_string())),
                    }
                } else {
                    tokens.push(Token::new(TokenType::Negation, char.to_string()));
                }
                word.clear();
            }

            // DELIMITERS
            '(' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) 
                        | Ok(TokenType::Identifier) 
                        | Ok(TokenType::RightParenthesis)
                        | Ok(TokenType::RightBrace) => tokens.push(
                            Token::new(TokenType::Multiplication, String::from("*"))
                        ),
                        Err(error) => return Err(error),
                        _ => (),
                    }
                }
                tokens.push(Token::new(TokenType::LeftParenthesis, char.to_string()));    
                word.clear();
            }
            ')' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::RightParenthesis, char.to_string()));
                word.clear();
            }
            '{' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) 
                        | Ok(TokenType::Identifier) 
                        | Ok(TokenType::RightParenthesis)
                        | Ok(TokenType::RightBracket)
                        |  Ok(TokenType::RightBrace) => tokens.push(
                            Token::new(TokenType::Multiplication, String::from("*"))
                        ),
                        Err(error) => return Err(error),
                        _ => (),
                    }
                }
                tokens.push(Token::new(TokenType::LeftBrace, char.to_string()));
                word.clear();
            }
            '}' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::RightBrace, char.to_string()));
                word.clear();
            }
            '[' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) 
                        | Ok(TokenType::Identifier)
                        | Ok(TokenType::RightParenthesis) 
                        | Ok(TokenType::RightBracket)
                        | Ok(TokenType::RightBrace) => tokens.push(
                            Token::new(TokenType::Multiplication, String::from("*"))
                        ),
                        Err(error) => return Err(error),
                        _ => (),
                    }
                }
                tokens.push(Token::new(TokenType::LeftBracket, char.to_string()));
                word.clear();
            } 
            ']' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::RightBracket, char.to_string()));
                word.clear();
            }

            // PUNCTUATION
            '!' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Exclamation, char.to_string()));
                word.clear();
            }
            ',' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Comma, char.to_string()));
                word.clear();
            }
            '?' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Question, char.to_string()));
                word.clear();
            }
            ':' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Colon, char.to_string()));
                word.clear();
            }
            ';' => {
                if !word.is_empty() { 
                    match tokens.push_word(&word) {
                        Some(error) => return Err(error),
                        None => (),
                    };
                }
                tokens.push(Token::new(TokenType::Semicolon, char.to_string()));
                word.clear();
            }
            _ => {
                word.push(char);
            }
        }
    }
    if !word.is_empty() { 
        match tokens.push_word(&word) {
            Some(error) => return Err(error),
            None => (),
        };
    }
    return Ok(tokens);
}

fn get_token_type(token: &str) -> Result<TokenType, LexerError> {
    match token {
        "^" => Ok(TokenType::Exponentiation),
        "*" => Ok(TokenType::Multiplication),
        "/" => Ok(TokenType::Division),
        "+" => Ok(TokenType::Addition),
        "-" => Ok(TokenType::Subtraction),

        "(" => Ok(TokenType::LeftParenthesis),
        ")" => Ok(TokenType::RightParenthesis),
        "{" => Ok(TokenType::LeftBrace),
        "}" => Ok(TokenType::RightBrace),
        "[" => Ok(TokenType::LeftBracket),
        "]" => Ok(TokenType::RightBracket),

        "!" => Ok(TokenType::Exclamation),
        "," => Ok(TokenType::Comma),
        "?" => Ok(TokenType::Question),
        ":" => Ok(TokenType::Colon),
        ";" => Ok(TokenType::Semicolon),

        _ if token.parse::<f64>().is_ok() => Ok(TokenType::Number),

        "sin" => Ok(TokenType::Keyword(Function::Sin)),
        "cos" => Ok(TokenType::Keyword(Function::Cos)),
        "tan" => Ok(TokenType::Keyword(Function::Tan)),
        _ if {
            let mut chars = token.chars();
            match chars.next() {
                Some(c) if c == '_' || is_xid_start(c) => chars.all(is_xid_continue),
                _ => false,      
            }
        } => Ok(TokenType::Identifier),

        _ => {
            return Err(LexerError::InvalidIdentifier(token.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_type_number() {
        assert_eq!(get_token_type("3").unwrap(), TokenType::Number);
    }

    #[test]
    fn test_get_token_type_identifier() {
        assert_eq!(get_token_type("variable").unwrap(), TokenType::Identifier);
    }

    #[test]
    fn test_get_token_type_operator() {
        assert_eq!(get_token_type("+").unwrap(), TokenType::Addition);
        assert_eq!(get_token_type("-").unwrap(), TokenType::Subtraction);
        assert_eq!(get_token_type("*").unwrap(), TokenType::Multiplication);
        assert_eq!(get_token_type("/").unwrap(), TokenType::Division);
        assert_eq!(get_token_type("^").unwrap(), TokenType::Exponentiation);
    }

    #[test]
    fn test_get_token_type_delimiter() {
        assert_eq!(get_token_type("(").unwrap(), TokenType::LeftParenthesis);
        assert_eq!(get_token_type(")").unwrap(), TokenType::RightParenthesis);
        assert_eq!(get_token_type("{").unwrap(), TokenType::LeftBrace);
        assert_eq!(get_token_type("}").unwrap(), TokenType::RightBrace);
        assert_eq!(get_token_type("[").unwrap(), TokenType::LeftBracket);
        assert_eq!(get_token_type("]").unwrap(), TokenType::RightBracket);
    }

    #[test]
    fn test_get_token_type_punctuation() {
        assert_eq!(get_token_type("!").unwrap(), TokenType::Exclamation);
        assert_eq!(get_token_type(",").unwrap(), TokenType::Comma);
        assert_eq!(get_token_type("?").unwrap(), TokenType::Question);
        assert_eq!(get_token_type(":").unwrap(), TokenType::Colon);
        assert_eq!(get_token_type(";").unwrap(), TokenType::Semicolon);
    }

    #[test]
    fn test_tokenise() {
        let input = "3 + 5 * (2 - 8)".to_string();
        let tokens = tokenise(input).unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].token_type, TokenType::Addition);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[3].token_type, TokenType::Multiplication);
        assert_eq!(tokens[4].token_type, TokenType::LeftParenthesis);
        assert_eq!(tokens[5].token_type, TokenType::Number);
        assert_eq!(tokens[6].token_type, TokenType::Subtraction);
        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[8].token_type, TokenType::RightParenthesis);
    }

    #[test]
    fn test_get_token_functions() {
        assert_eq!(get_token_type("sin").unwrap(), TokenType::Keyword(Function::Sin));
        assert_eq!(get_token_type("cos").unwrap(), TokenType::Keyword(Function::Cos));
        assert_eq!(get_token_type("tan").unwrap(), TokenType::Keyword(Function::Tan));
    }

    #[test]
    fn test_push_word_basic() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("foo");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        }
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Identifier, lexeme: "foo".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_number() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("2");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Number, lexeme: "2".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_leading_number() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("2foo");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Number, lexeme: "2".to_string() },
            Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
            Token { token_type: TokenType::Identifier, lexeme: "foo".to_string() },
        ];
        assert_eq!(tokens, expected_tokens)
    }

    #[test]
    fn test_push_utf8() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("foo_bar_π");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Identifier, lexeme: "foo_bar_π".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_with_double_digit_number() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("42");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Number, lexeme: "42".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_with_decimal_number() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("3.14");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Number, lexeme: "3.14".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_with_decimal_number_and_identifier() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("3.14foo");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Number, lexeme: "3.14".to_string() },
            Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
            Token { token_type: TokenType::Identifier, lexeme: "foo".to_string() },
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_with_trailing_number() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("foo42");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Identifier, lexeme: "foo42".to_string() },
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[should_panic(expected = "LexerError: InvalidIdentifier(\"🍕\")")]
    #[test]
    fn test_push_word_with_leading_emoji() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("🍕");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Identifier, lexeme: "🍕".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_with_leading_underscore() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("_foo");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Identifier, lexeme: "_foo".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_with_leading_number_and_underscore() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("3_foo");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Number, lexeme: "3".to_string() },
            Token { token_type: TokenType::Multiplication, lexeme: "*".to_string() },
            Token { token_type: TokenType::Identifier, lexeme: "_foo".to_string() },
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_push_word_only_underscore() {
        let mut tokens: Vec<Token> = vec![];
        let word = String::from("_");
        match tokens.push_word(&word) {
            Some(error) => panic!("LexerError: {:?}", error),
            None => (),
        };
        let expected_tokens: Vec<Token> = vec![
            Token { token_type: TokenType::Identifier, lexeme: "_".to_string() }
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
