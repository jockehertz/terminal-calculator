#[derive(Clone)]
#[derive(PartialEq)]
pub enum TokenType {
    Number,
    Keyword,

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
    Period,
    Comma,
    Question,
    Colon,
    Semicolon,
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
    let mut prev = String::new();
    let mut word = String::new();

    for char in string.chars() {
        match char {
            ' ' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                word = String::from("");
            }

            //OPERATORS
            '^' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Exponentiation, char.to_string()));
                word = String::new();
                prev = String::from("^");
            }
            '*' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Multiplication, char.to_string()));
                word = String::new();
                prev = String::from("*");
            }
            '/' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Division, char.to_string()));
                word = String::new();
                prev = String::from("/");
            } 
            '+' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Addition, char.to_string()));
                word = String::new();
                prev = String::from("+");
            }
            '-' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                if &prev != "" {
                    match get_token_type(&prev) {
                        TokenType::Number | TokenType::Keyword => tokens.push(
                            Token::new(TokenType::Subtraction, char.to_string())
                            ),
                        _ => tokens.push(Token::new(TokenType::Negation, char.to_string())),
                    }
                } else {
                    tokens.push(Token::new(TokenType::Negation, char.to_string()));
                }
                word = String::new();
                prev = String::from("-");
            }

            // DELIMITERS
            '(' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                match get_token_type(&prev) {
                    TokenType::Number | TokenType::Keyword => tokens.push(
                        Token::new(TokenType::Multiplication, String::from("*"))
                        ),
                    _ => (),
                }
                tokens.push(Token::new(TokenType::LeftParenthesis, char.to_string()));
                    
                word = String::new();
                prev = String::from("(");
            }
            ')' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::RightParenthesis, char.to_string()));
                word = String::new();
                prev = String::from(")");
            }
            '{' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::LeftBrace, char.to_string()));
                word = String::new();
                prev = String::from("{");
            }
            '}' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::RightBrace, char.to_string()));
                word = String::new();
                prev = String::from("}");
            }
            '[' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::LeftBracket, char.to_string()));
                word = String::new();
                prev = String::from("[");
            } 
            ']' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::RightBracket, char.to_string()));
                word = String::new();
                prev = String::from("]");
            }

            // PUNCTUATION
            '!' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Exclamation, char.to_string()));
                word = String::new();
                prev = String::from("!");
            }
            '.' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Period, char.to_string()));
                word = String::new();
                prev = String::from(".");
            } 
            ',' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Comma, char.to_string()));
                word = String::new();
                prev = String::from(",");
            }
            '?' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Question, char.to_string()));
                word = String::new();
                prev = String::from("?");
            }
            ':' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Colon, char.to_string()));
                word = String::new();
                prev = String::from(":");
            }
            ';' => {
                if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
                tokens.push(Token::new(TokenType::Semicolon, char.to_string()));
                word = String::new();
                prev = String::from(";");
            }
            _ => {
                word.push(char);
                if word != "" {
                    prev = word.clone();
                }
            }
        }
    }
    if word != "" { tokens.push(Token::new(get_token_type(&word), word)); }
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
        "." => TokenType::Period,
        "," => TokenType::Comma,
        "?" => TokenType::Question,
        ":" => TokenType::Colon,
        ";" => TokenType::Semicolon,

        _ if token.chars().all(|c| c.is_numeric()) => TokenType::Number,
        _ if token.chars().all(|c| c.is_ascii()) => TokenType::Keyword,

        _ => panic!("Unknown token: {}", token),
    }
}
