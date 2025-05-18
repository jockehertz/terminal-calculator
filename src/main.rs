mod errors;
mod lexer;
mod parser;
mod evaluator;
use crate::lexer::{Token, TokenType};
use crate::parser::AstNode;
use crate::errors::{ParseError, InputError};
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
}

enum Command {
    Exit,
    Debug,
    Evaluate(String),
}

// Displays a welcome message and starts the REPL 
fn main() {
    let mut context = Context { debug_mode: false };
    let argv: Vec<String> = env::args().collect();
    if argv.len() > 1 {
        if argv[1] == "--debug" || argv[1] == "-d" {
            context.debug_mode = true;
        }
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

    match input.trim() {
        "exit" => return Ok(Command::Exit),
        "debug" | "dbg" => return Ok(Command::Debug),
        _ => {
            if input.trim().is_empty() {
                return Err(InputError::EmptyInput);
            } else {
                return Ok(Command::Evaluate(input));
            }
        }
    }
}

// Evaluates the input
fn evaluate(input: &str, context: &Context) -> () {
    debug_println!(context, "\nInput: {}", input); 
    debug_println!(context, "Tokenising..."); 
    
    let tokens: Vec<Token> = lexer::tokenise(input.to_owned());
    
    debug_println!(context, "Tokenisation complete.");
    debug_println!(context, "Tokens:");
    
    if context.debug_mode {
        print_tokens(&tokens);
    } 

    debug_println!(context, "Generating AST...");

    let ast: AstNode = match parser::construct_ast(&tokens) {
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

    let result: f64 = ast.evaluate();
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