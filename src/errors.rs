use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    StackUnderflow,
    UnknownWord(String),
    Redefined(String),
    ParsingError,
    InvalidAddress,
    LeaveLoop,
    DivisionByZero,
    CompileTimeWord,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let msg = match self {
            StackUnderflow => "stack underflow".to_string(),
            UnknownWord(word) => format!("{} is an unknown word", word),
            Redefined(name) => format!("{} was redefined", name),
            ParsingError => "parsing error".to_string(),
            InvalidAddress => "invalid memory address".to_string(),
            LeaveLoop => "leaving the loop".to_string(),
            DivisionByZero => "division by zero".to_string(),
            CompileTimeWord => "interpreting a compile-only word".to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}
