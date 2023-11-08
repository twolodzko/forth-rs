use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    StackUnderflow,
    UnknownWord(String),
    CompileOnlyWord(String),
    Missing(char),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let msg = match self {
            StackUnderflow => "stack underflow".to_string(),
            UnknownWord(word) => format!("unknown word: {}", word),
            CompileOnlyWord(word) => format!("this is a compile-only word: {}", word),
            Missing(char) => format!("missing: {}", char),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}
