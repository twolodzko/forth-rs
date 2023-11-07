use crate::errors::Error;
use std::collections::HashMap;

pub type Int = i32;
pub type ForthResult = Result<(), Error>;
pub type Definition = fn(&mut Forth) -> ForthResult;

// trait Definition {
//     fn execute(&self, forth: &mut Forth) -> ForthResult;
// }

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
        match self.dictionary.get(word) {
            Some(word) => word(self),
            None => {
                if let Ok(num) = word.parse::<Int>() {
                    self.stack.push(num);
                    Ok(())
                } else {
                    Err(Error::UnknownWord(word.to_string()))
                }
            }
        }
    }
}
