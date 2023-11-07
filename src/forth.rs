use crate::custom::*;
use crate::errors::Error::{self, StackUnderflow, UnknownWord};
use std::collections::HashMap;

pub type Int = i32;
pub type ForthResult = Result<(), Error>;

#[allow(dead_code)] // FIXME
#[derive(Clone)]
pub enum Definition {
    Variable(Int),
    Constant(Int),
    Callable(fn(forth: &mut Forth) -> ForthResult),
    Function(Function),
}

pub struct Forth {
    pub stack: Vec<Int>,
    pub(crate) dictionary: HashMap<String, Definition>,
}

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity.
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
            dictionary: HashMap::new(),
        }
    }

    /// Execute the word. If the word does not exist in the dictionary, try parsing it as a number and pushing
    /// it into the stack.
    pub fn execute(&mut self, word: &str) -> ForthResult {
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
