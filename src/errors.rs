pub enum ParseError {
    UnexpectedEndOfInput,
    MissingClosingParenthesis,
    UnexpectedToken(String),
    UnexpectedTokensAtEnd,
    //InvalidNumber(String),
}
pub enum InputError {
    ReadError,
    EmptyInput,
}
