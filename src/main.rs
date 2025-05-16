mod lexer;
mod parser;
mod evaluator;
use crate::lexer::{Token, TokenType};
use crate::parser::AstNode;
use std::io::{stdin, stdout, Write};

// Displays a welcome message and starts the REPL 
fn main() {
    println!("Welcome to the beginnings of my terminal-based calculator!");
    println!("For now, this is only a REPL which tokenises string inputs, feel free to try it!");
    repl();
}

// READ-EVALUATE-PRINT-LOOP (REPL)
fn repl() -> () {
    let mut running: bool = true;
    while running {
        let input: &str = &input(None);
        
        match input {
            "exit" => running = false,
            _ => evaluate(input),
        }
    }
}

// Takes an input from the user (READ)
fn input(output: Option<&str>) -> String {
    let mut input = String::new();
    match output {
        Some(text) => print!("> {}", text),
        None => print!("> "),
    }
    let _ = stdout().flush();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => panic!("Error: {}. \nUnable to read input.", error),
    }

    return input.trim().to_string();
}

// Evaluates the input
fn evaluate(input: &str) -> () {
    let tokens: Vec<Token> = lexer::tokenise(input.to_owned());
    for token in &tokens {
        match token.token_type {
            TokenType::Number => println!("Type: Number, Lexeme: {}", token.lexeme),
            TokenType::Keyword => println!("Type: Keyword, Lexeme: {}", token.lexeme),

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
            TokenType::Period => println!(
                "Type: Punctuation, Period, Lexeme: {}", 
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
    println!("Generating AST...");
    let ast: AstNode = parser::construct_ast(&tokens);
    println!("AST Generated.");
    let result: f64 = ast.evaluate();
    println!("Result: {}", result);
}

