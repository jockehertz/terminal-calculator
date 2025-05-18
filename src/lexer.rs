use crate::errors::{EvaluationError, LexerError};
#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    Number,
    Identifier,

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
                    match get_token_type(&word.trim()) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                word.clear();
            }

            //OPERATORS
            '^' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Exponentiation, char.to_string()));
                word.clear();
            }
            '*' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Multiplication, char.to_string()));
                word.clear();
            }
            '/' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Division, char.to_string()));
                word.clear();
            } 
            '+' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Addition, char.to_string()));
                word.clear();
            }
            '-' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
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
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) | Ok(TokenType::Identifier) => tokens.push(
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
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::RightParenthesis, char.to_string()));
                word.clear();
            }
            '{' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) | Ok(TokenType::Identifier) => tokens.push(
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
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::RightBrace, char.to_string()));
                word.clear();
            }
            '[' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                let prev = match tokens.last() {
                    Some(token) => token.lexeme.clone(),
                    None => String::new(),
                };
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        Ok(TokenType::Number) | Ok(TokenType::Identifier) => tokens.push(
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
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::RightBracket, char.to_string()));
                word.clear();
            }

            // PUNCTUATION
            '!' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Exclamation, char.to_string()));
                word.clear();
            }
            ',' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Comma, char.to_string()));
                word.clear();
            }
            '?' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Question, char.to_string()));
                word.clear();
            }
            ':' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
                }
                tokens.push(Token::new(TokenType::Colon, char.to_string()));
                word.clear();
            }
            ';' => {
                if !word.is_empty() { 
                    match get_token_type(&word) {
                        Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
                        Err(error) => return Err(error),
                    }
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
        match get_token_type(&word) {
            Ok(token_type) => tokens.push(Token::new(token_type, word.clone())),
            Err(error) => return Err(error),
        }
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

        _ if {
            let mut chars = token.chars();
            match chars.next() {
                Some(c) if c.is_ascii_alphabetic() || c == '_' => chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
                _ => false,
            }
        } => Ok(TokenType::Identifier),

        _ => {
            return Err(LexerError::InvalidToken(token.to_string()));
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

}