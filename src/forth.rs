use crate::errors::Error::{self, StackUnderflow, UnknownWord};
use crate::special::{Function, IfThenElse};
use std::collections::HashMap;
use std::str::Chars;

pub type Int = i32;

#[allow(dead_code)] // FIXME
#[derive(Clone)]
pub enum Definition {
    Variable(Int),
    Constant(Int),
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    Function(Function),
    IfThenElse(IfThenElse),
}

pub struct Forth {
    buffer: String,
    buffer_position: usize,
    stack: Vec<Int>,
    dictionary: HashMap<String, Definition>,
}

impl Forth {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: String::new(),
            buffer_position: 0,
            stack: Vec::with_capacity(capacity),
            dictionary: HashMap::new(),
        }
    }

    pub fn eval(&mut self, buffer: &str) -> Result<(), Error> {
        let start: usize;
        let end: usize;
        let mut chars = buffer.chars();

        // skip leading spaces
        for c in &mut chars {
            if c.is_whitespace() {
                break;
            }
        }

        match read_word(&mut chars).expect("nothing was read").as_ref() {
            ":" => unimplemented!(),
            ".\"" => unimplemented!(),
            ".(" => unimplemented!(),
            word => {}
        }
        Ok(())
    }

    /// Execute the word. If the word does not exist in the dictionary, try parsing it as a number and pushing
    /// it into the stack.
    pub fn execute(&mut self, word: &str) -> Result<(), Error> {
        use self::Definition::*;
        match self.dictionary.get(word) {
            Some(Callable(callable)) => callable(self),
            Some(Constant(val)) => {
                self.push(*val);
                Ok(())
            }
            Some(Function(func)) => func.clone().execute(self),
            Some(_) => unimplemented!(),
            None => {
                if let Ok(num) = word.parse::<Int>() {
                    self.push(num);
                    Ok(())
                } else {
                    Err(UnknownWord(word.to_string()))
                }
            }
        }
    }

    /// Push value to the stack
    pub(crate) fn push(&mut self, value: Int) {
        self.stack.push(value);
    }

    /// Pop value from the stack
    pub(crate) fn pop(&mut self) -> Result<Int, Error> {
        self.stack.pop().ok_or(StackUnderflow)
    }
}

fn read_word(chars: &mut Chars<'_>) -> Option<String> {
    let mut word = String::new();
    for c in chars {
        if c.is_whitespace() {
            break;
        }
        word.push(c);
    }
    if word.is_empty() {
        return None;
    }
    Some(word)
}
