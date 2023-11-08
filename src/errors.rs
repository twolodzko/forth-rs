use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    StackUnderflow,
    UnknownWord(String),
    CompileOnlyWord,
    Redefined(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let msg = match self {
            StackUnderflow => "stack underflow".to_string(),
            UnknownWord(word) => format!("{} is an unknown word", word),
            CompileOnlyWord => "compile-only word".to_string(),
            Redefined(name) => format!("{} was redefined", name),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}
