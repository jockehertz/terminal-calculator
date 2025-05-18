use terminal_calculator::lexer::{Token, TokenType, tokenise};
use terminal_calculator::parser::{AstNode, construct_ast};
use terminal_calculator::errors::{ParseError, InputError, EvaluationError, LexerError};
use std::io::{stdin, stdout, Write};
use std::env;

macro_rules! debug_println {
    ($ctx:expr, $($arg:tt)*) => {
        if $ctx.debug_mode {
            println!($($arg)*);
        }
    };
}

struct Context {
    debug_mode: bool,
    included_tokens: String,
}

#[derive(Debug, PartialEq)]
enum Command {
    Exit,
    Debug,
    Evaluate(String),
}

// Displays a welcome message and starts the REPL 
fn main() {
    let mut context = Context { debug_mode: false, included_tokens: String::new() };
    let argv: Vec<String> = env::args().collect();
    if argv.len() > 1 {
        context = parse_args(argv);
    }

    if context.included_tokens.len() > 0 {
        evaluate(&context.included_tokens, &context);
        return;
        
    }
    println!("Welcome to the beginnings of my terminal-based calculator!");
    println!("For now, this is only a REPL which tokenises string inputs, feel free to try it!\n");
    repl(&mut context);
}

// READ-EVALUATE-PRINT-LOOP (REPL)
fn repl(context: &mut Context) -> () {
    let mut running: bool = true;
    while running {
        let input: Command = match input(None) {
            Ok(input) => input,
            Err(error) => {
                match error {
                    InputError::ReadError => {
                        println!("Error: Could not read input.");
                        continue;
                    }
                    InputError::EmptyInput => {
                        continue;
                    }
                }
            }
        };
        
        match input {
            Command::Exit => running = false,
            Command::Debug => {
                context.debug_mode = !context.debug_mode;
                if context.debug_mode {
                    println!("Debug mode enabled.");
                } else {
                    println!("Debug mode disabled.");
                }
            }
            Command::Evaluate(input) => evaluate(&input, &context),
        }
    }
}

// Takes an input from the user (READ)
fn input(output: Option<&str>) -> Result<Command, InputError> {
    let mut input = String::new();
    match output {
        Some(text) => print!("> {}", text),
        None => print!("> "),
    }
    let _ = stdout().flush();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => return Err(InputError::ReadError),
    }

    match parse_command(input) {
        Ok(command) => Ok(command),
        Err(error) => Err(error),
    }
}

fn parse_command(input: String) -> Result<Command, InputError> {
    let input = input.trim();
    match input {
        "exit" => return Ok(Command::Exit),
        "debug" | "dbg" => return Ok(Command::Debug),
        _ => if input.is_empty() {
            return Err(InputError::EmptyInput);
        } else {
            return Ok(Command::Evaluate(input.to_string()));
        },
    }
}

// Evaluates the input
fn evaluate(input: &str, context: &Context) -> () {
    debug_println!(context, "\nInput: {}", input); 
    debug_println!(context, "Tokenising..."); 
    
    let tokens: Vec<Token> = match tokenise(input.to_owned()) {
        Ok(tokens) => tokens,
        Err(error) => {
            match error {
                LexerError::InvalidToken(token) => {
                    println!("LexerError: Invalid token in input: {}", token);
                    return;
                }
            }
        }
    };
    
    debug_println!(context, "Tokenisation complete.");
    debug_println!(context, "Tokens:");
    
    if context.debug_mode {
        print_tokens(&tokens);
    } 

    debug_println!(context, "Generating AST...");

    let ast: AstNode = match construct_ast(&tokens) {
        Ok(ast) => ast,
        Err(error) => {
            match error {
                ParseError::UnexpectedEndOfInput => {
                    println!("ParseError: Unexpected end of input.");
                    return;
                }
                ParseError::MissingClosingParenthesis => {
                    println!("ParseError: Missing closing parenthesis.");
                    return;
                }
                ParseError::UnexpectedToken(token) => {
                    println!("ParseError: Unexpected token: {}", token);
                    return;
                }
                ParseError::UnexpectedTokensAtEnd => {
                    println!("ParseError: Unexpected tokens at end of input.");
                    return;
                }
                /* ParseError::InvalidNumber(token) => {
                    println!("ParseError: Invalid number: {}", token);
                    return;
                } */
            }
        }
    };

    debug_println!(context, "AST Generated.\n");

    let result: f64 = match ast.evaluate() {
        Ok(result) => result,
        Err(error) => {
            match error {
                EvaluationError::DivisionByZero => {
                     println!("EvaluationError: Division by zero.");
                     return;
                }
                EvaluationError::InvalidOperation => {
                    println!("EvaluationError: Invalid operation.");
                    return;
                }
                // EvaluationError::InvalidInput => {
                //     println!("EvaluationError: Invalid input.");
                //     return;
                // }
            }
        }
    };
    println!("Result: {}", result);
}

fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        match token.token_type {
            TokenType::Number => println!("Type: Number, Lexeme: {}", token.lexeme),
            TokenType::Identifier => println!("Type: Identifier, Lexeme: {}", token.lexeme),

            // OPERATORS
            TokenType::Negation => println!(
                "Type: Unary Operator, Negation, Lexeme: {}",
                token.lexeme,
                ),
            TokenType::Exponentiation => println!(
                "Type: Binary Operator, Exponentiation, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Multiplication => println!(
                "Type: Binary Operator, Multiplication, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Division => println!(
                "Type: Binary Operator, Division, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Addition => println!(
                "Type: Binary Operator, Addition, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Subtraction => println!(
                "Type: Binary Operator, Subtraction, Lexeme: {}", 
                token.lexeme,
                ), 

            // PUNCTUATION
            TokenType::Semicolon => println!(
                "Type: Punctuation, Semicolon, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Comma => println!(
                "Type: Punctuation, Comma, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Colon => println!(
                "Type: Punctuation, Colon, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Exclamation => println!(
                "Type: Punctuation, Exclamation mark, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::Question => println!(
                "Type: Punctuation, Question mark, Lexeme: {}", 
                token.lexeme,
                ),

            // DELIMITERS
            TokenType::LeftParenthesis => println!(
                "Type: Delimiter, Left Parenthesis, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::RightParenthesis => println!(
                "Type: Delimiter, Right Parenthesis, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::LeftBracket => println!(
                "Type: Delimiter, Left Bracket, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::RightBracket => println!(
                "Type: Delimiter, Right Bracket, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::LeftBrace => println!(
                "Type: Delimiter, Left Brace, Lexeme: {}", 
                token.lexeme,
                ),
            TokenType::RightBrace => println!(
                "Type: Delimiter, Right Brace, Lexeme: {}", 
                token.lexeme,
                ),
        }
    }
    println!("Token printing complete.");
    print!("\n");
}

fn parse_args(args: Vec<String>) -> Context {
    let mut context = Context { debug_mode: false, included_tokens: String::new() };
    if args.len() > 1 {
        if args[1] == "--debug" {
            context.debug_mode = true;
            println!("Debug mode enabled.");
            context.included_tokens = args[2..].join(" ");
        } else if match args.last() {
            Some(last) => last == "--debug",
            None => false,
        } {
            context.debug_mode = true;
            context.included_tokens = args[1..args.len() - 1].join(" ");
        } else {
            context.included_tokens = args[1..].join(" ");
        }
    }
    context
}

#[cfg(test)]
mod tests {
    use super::*;
    use terminal_calculator::errors::InputError;

    // Checks that CLI arguments are parsed correctly
    #[test]
    fn test_cli_arg_parsing_1() {
        let args = vec!["calc".to_string(), "--debug".to_string(), "3 + 5".to_string()];
        let context = parse_args(args);
        assert_eq!(context.debug_mode, true);
        assert_eq!(context.included_tokens, "3 + 5");
    }

    // Checks that CLI arguments are parsed correctly
    #[test]
    fn test_cli_arg_parsing_2() {
        let args = vec!["calc".to_string(), "3 + 5".to_string(), "--debug".to_string()];
        let context = parse_args(args);
        assert_eq!(context.debug_mode, true);
        assert_eq!(context.included_tokens, "3 + 5");
    }

    // Checks that CLI arguments with only --debug are parsed correctly
    #[test]
    fn test_cli_arg_parsing_only_debug() {
        let args = vec!["calc".to_string(), "--debug".to_string()];
        let context = parse_args(args);
        assert_eq!(context.debug_mode, true);
        assert_eq!(context.included_tokens, "");
    }

    // Checks that CLI arguments with no --debug are parsed correctly
    #[test]
    fn test_cli_arg_parsing_no_debug() {
        let args = vec!["calc".to_string(), "3 + 5".to_string()];
        let context = parse_args(args);
        assert_eq!(context.debug_mode, false);
        assert_eq!(context.included_tokens, "3 + 5");
    }

    // Checks that CLI arguments with no --debug and no input are parsed correctly
    #[test]
    fn test_cli_arg_parsing_no_debug_no_input() {
        let args = vec!["calc".to_string()];
        let context = parse_args(args);
        assert_eq!(context.debug_mode, false);
        assert_eq!(context.included_tokens, "");
    }

    // Checks that the input is read correctly
    #[test]
    fn test_input_reading() {
        let input = "3 + 5";
        let command = parse_command(input.to_string()).unwrap();
        assert_eq!(command, Command::Evaluate("3 + 5".to_string()));
    }

    // Checks that parse_command correctly identifies the exit command
    #[test]
    fn test_input_reading_exit() {
        let input = "exit";
        let command = parse_command(input.to_string()).unwrap();
        assert_eq!(command, Command::Exit);
    }

    // Checks that parse_command correctly identifies the debug command
    #[test]
    fn test_input_reading_debug() {
        let input = "debug";
        let command = parse_command(input.to_string()).unwrap();
        assert_eq!(command, Command::Debug);
    }

    // Checks that parse_command correctly identifies the debug command (short form)
    #[test]
    fn test_input_reading_debug_short() {
        let input = "dbg";
        let command = parse_command(input.to_string()).unwrap();
        assert_eq!(command, Command::Debug);
    }

    // Checks that parse_command returns an error for empty input
    #[test]
    fn test_input_reading_empty() {
        let input = "";
        let command = parse_command(input.to_string());
        assert_eq!(command, Err(InputError::EmptyInput));
    }
}