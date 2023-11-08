use crate::{
    compiled::{Compiled, Int},
    errors::Error::{self, StackUnderflow, UnknownWord},
    reader::read,
};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Parsed {
    Word(String),
    Binding((String, Compiled)),
}

pub struct Forth {
    pub stack: Vec<Int>,
    pub(crate) dictionary: HashMap<String, Compiled>,
}

impl Forth {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
            dictionary: HashMap::new(),
        }
    }

    pub fn parse_and_eval(&mut self, buffer: &str) -> Result<(), Error> {
        let mut chars = buffer.chars();
        match read(&mut chars).expect("nothing was read") {
            Parsed::Word(ref word) => self.evaluate(word),
            Parsed::Binding((name, def)) => {
                self.dictionary.insert(name, def);
                Ok(())
            }
        }
    }

    /// Execute the word. If the word does not exist in the dictionary, try parsing it as a number and pushing
    /// it into the stack.
    pub fn evaluate(&mut self, word: &str) -> Result<(), Error> {
        match self.dictionary.get(word).cloned() {
            Some(compiled) => compiled.execute(self),
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
