#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEndOfInput,
    MissingClosingParenthesis,
    UnexpectedToken(String),
    UnexpectedTokensAtEnd,
    //InvalidNumber(String),
}

#[derive(Debug, PartialEq)]
pub enum InputError {
    ReadError,
    EmptyInput,
}

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    DivisionByZero,
    InvalidOperation,
    NotAFunction,
    Undefined,
    // InvalidInput,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidToken(String),
    InvalidIdentifier(String),
}
