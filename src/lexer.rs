#[derive(Clone)]
#[derive(PartialEq)]
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
    pub fn apply_unary(&self, operand: f64) -> f64 {
        match self {
            TokenType::Negation => -operand,
            _ => panic!("Not a unary operator"),
        }
    }
    pub fn apply_binary(&self, operand_1: f64, operand_2: f64) -> f64 {
        match self {
            TokenType::Exponentiation => operand_1.powf(operand_2),
            TokenType::Multiplication => operand_1 * operand_2,
            TokenType::Division => {
                if operand_2 != 0.0 {
                    operand_1 / operand_2
                } else {
                    panic!("Division by zero");
                }
            }
            TokenType::Addition => operand_1 + operand_2,
            TokenType::Subtraction => operand_1 - operand_2,
            _ => panic!("Not a binary operator"),
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}

pub fn tokenise(string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut word = String::new();

    for char in string.chars() {
        match char {
            ' ' | '\n' | '\t' => {
                if !word.is_empty() { tokens.push(Token::new(get_token_type(&word.trim()), word.clone())); }
                word.clear();
            }

            //OPERATORS
            '^' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word.trim()), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Exponentiation, char.to_string()));
                word.clear();
            }
            '*' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word.trim()), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Multiplication, char.to_string()));
                word.clear();
            }
            '/' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word.trim()), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Division, char.to_string()));
                word.clear();
            } 
            '+' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word.trim()), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Addition, char.to_string()));
                word.clear();
            }
            '-' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word.trim()), word.clone())); 
                }
                let prev = tokens.last().unwrap().lexeme.clone();
                if !prev.is_empty() {
                    match get_token_type(&prev) {
                        TokenType::Number | TokenType::Identifier => tokens.push(
                            Token::new(TokenType::Subtraction, char.to_string())
                            ),
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
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                let prev = tokens.last().unwrap().lexeme.clone();
                match get_token_type(&prev) {
                    TokenType::Number | TokenType::Identifier => tokens.push(
                        Token::new(TokenType::Multiplication, String::from("*"))
                        ),
                    _ => (),
                }
                tokens.push(Token::new(TokenType::LeftParenthesis, char.to_string()));
                    
                word.clear();
            }
            ')' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::RightParenthesis, char.to_string()));
                word.clear();
            }
            '{' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::LeftBrace, char.to_string()));
                word.clear();
            }
            '}' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::RightBrace, char.to_string()));
                word.clear();
            }
            '[' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::LeftBracket, char.to_string()));
                word.clear();
            } 
            ']' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::RightBracket, char.to_string()));
                word.clear();
            }

            // PUNCTUATION
            '!' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Exclamation, char.to_string()));
                word.clear();
            }
            ',' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Comma, char.to_string()));
                word.clear();
            }
            '?' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Question, char.to_string()));
                word.clear();
            }
            ':' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Colon, char.to_string()));
                word.clear();
            }
            ';' => {
                if !word.is_empty() { 
                    tokens.push(Token::new(get_token_type(&word), word.clone())); 
                }
                tokens.push(Token::new(TokenType::Semicolon, char.to_string()));
                word.clear();
            }
            _ => {
                word.push(char);
            }
        }
    }
    if !word.is_empty() { tokens.push(Token::new(get_token_type(&word), word)); }
    return tokens;
}

fn get_token_type(token: &str) -> TokenType {
    match token {
        "^" => TokenType::Exponentiation,
        "*" => TokenType::Multiplication,
        "/" => TokenType::Division,
        "+" => TokenType::Addition,
        "-" => TokenType::Subtraction,

        "(" => TokenType::LeftParenthesis,
        ")" => TokenType::RightParenthesis,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "[" => TokenType::LeftBracket,
        "]" => TokenType::RightBracket,

        "!" => TokenType::Exclamation,
        "," => TokenType::Comma,
        "?" => TokenType::Question,
        ":" => TokenType::Colon,
        ";" => TokenType::Semicolon,

        _ if token.parse::<f64>().is_ok() => TokenType::Number,

        _ if {
            let mut chars = token.chars();
            match chars.next() {
                Some(c) if c.is_ascii_alphabetic() || c == '_' => chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
                _ => false,
            }
        } => TokenType::Identifier,

        _ => {
            println!("DEBUG: Unknown token type: {:?}", token);
            panic!("Unknown token type: {}", token);
        }
    }
}
