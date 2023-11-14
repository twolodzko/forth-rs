use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    StackUnderflow,
    UnknownWord(String),
    Redefined(String),
    InvalidAddress,
    DivisionByZero,
    CompileTimeWord,
    CustomError(String),
    ParsingError(String),
    InvalidName(String),
    MissingArgument,
    Leave,
    Exit,
    Quit,
    Recurse,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let msg = match self {
            StackUnderflow => "stack underflow".into(),
            UnknownWord(word) => format!("{} is an unknown word", word),
            Redefined(name) => format!("{} was redefined", name),
            InvalidAddress => "invalid memory address".into(),
            DivisionByZero => "division by zero".into(),
            CompileTimeWord => "interpreting a compile-only word".into(),
            InvalidName(name) => format!("{} is an invalid name", name),
            MissingArgument => "argument is missing".into(),
            CustomError(msg) => msg.into(),
            ParsingError(msg) => msg.into(),
            Exit | Quit | Leave | Recurse => unreachable!(),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}
