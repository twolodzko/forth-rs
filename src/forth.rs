use crate::{
    errors::Error::{self, Redefined, StackUnderflow},
    expressions::{Expr, Int},
    parser::{self, new_reader, Reader},
};
use std::collections::HashMap;

/// The Forth interpreter that walks over the code and executes it
pub struct Forth {
    pub stack: Vec<Int>,
    dictionary: HashMap<String, Expr>,
}

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity.
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
            dictionary: HashMap::new(),
        }
    }

    /// Evaluate a string
    pub fn eval_string(&mut self, string: &str) -> Result<(), Error> {
        let reader = &mut new_reader(string);
        while let Some(result) = self.eval_next(reader) {
            result.map_err(|err| {
                // clear stack on error
                self.stack.clear();
                err
            })?;
        }
        Ok(())
    }

    /// Go to next word and evaluate it
    pub(crate) fn eval_next(&mut self, reader: &mut Reader) -> Option<Result<(), Error>> {
        let expr = parser::next(reader)?;
        Some(expr.execute(self))
    }

    /// Push value to the stack.
    pub(crate) fn push(&mut self, value: Int) {
        self.stack.push(value)
    }

    /// Pop value from the stack.
    pub(crate) fn pop(&mut self) -> Result<Int, Error> {
        self.stack.pop().ok_or(StackUnderflow)
    }

    /// Define a new word, return an error on redefinition.
    pub(crate) fn define_word(&mut self, name: &str, value: Expr) -> Result<(), Error> {
        if self.dictionary.insert(name.to_string(), value).is_some() {
            return Err(Redefined(name.to_string()));
        }
        Ok(())
    }

    /// Get the compiled object associated to the word.
    pub(crate) fn get_word(&self, name: &str) -> Option<Expr> {
        self.dictionary.get(name).cloned()
    }

    /// The list of all the defined words.
    pub(crate) fn words(&self) -> Vec<String> {
        let mut words = self
            .dictionary
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        words.sort();
        words
    }
}
