use crate::{
    compiled::{Compiled, Int},
    errors::Error::{self, CompileOnlyWord, StackUnderflow, UnknownWord},
    reader::{read, Parsed},
};
use std::{collections::HashMap, iter::Peekable, str::Chars};

/// The Forth interpreter that walks over the code and executes it
pub struct Forth {
    pub stack: Vec<Int>,
    pub(crate) dictionary: HashMap<String, Compiled>,
}

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
            dictionary: HashMap::new(),
        }
    }

    /// Execute the word. If the word does not exist in the dictionary, try parsing it as a number and pushing
    /// it into the stack.
    pub(crate) fn eval_word(&mut self, word: &str) -> Result<(), Error> {
        // those should not be evaluated
        if matches!(word, "if" | "do" | "begin") {
            return Err(CompileOnlyWord(word.to_string()));
        }

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

    fn next(&mut self, chars: &mut Peekable<Chars<'_>>) -> Option<Result<(), Error>> {
        use Parsed::{Binding, ToPrint, Word};
        let result = match read(chars)? {
            Word(ref word) => self.eval_word(word),
            Binding((name, compiled)) => {
                self.dictionary.insert(name, compiled);
                Ok(())
            }
            ToPrint(string) => {
                print!("{} ", string);
                Ok(())
            }
        };
        Some(result)
    }

    pub fn eval_string(&mut self, string: &str) -> Result<(), Error> {
        let chars = &mut string.chars().peekable();
        while let Some(result) = self.next(chars) {
            result?;
        }
        Ok(())
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
