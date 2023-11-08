use crate::{
    compiled::{Compiled, Int},
    errors::Error::{self, Redefined, StackUnderflow, UnknownWord},
    reader::{read, Parsed},
};
use std::{collections::HashMap, iter::Peekable, str::Chars};

/// The Forth interpreter that walks over the code and executes it
pub struct Forth {
    pub stack: Vec<Int>,
    dictionary: HashMap<String, Compiled>,
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
    pub(crate) fn eval_word(&mut self, word: &str) -> Result<(), Error> {
        match self.get_word(word) {
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

    /// Go to next word and evaluate it
    fn eval_next_word(&mut self, chars: &mut Peekable<Chars<'_>>) -> Option<Result<(), Error>> {
        use Parsed::{Binding, Constant, ToPrint, Word};
        let result = match read(chars)? {
            Word(ref word) => self.eval_word(word),
            Binding((ref name, compiled)) => self.define_word(name, compiled),
            ToPrint(string) => {
                print!("{} ", string);
                Ok(())
            }
            Constant(ref name) => match self.pop() {
                Ok(value) => self.define_word(name, Compiled::Constant(value)),
                Err(msg) => Err(msg),
            },
        };
        Some(result)
    }

    /// Evaluate a string
    pub fn eval_string(&mut self, string: &str) -> Result<(), Error> {
        let chars = &mut string.chars().peekable();
        while let Some(result) = self.eval_next_word(chars) {
            result?;
        }
        Ok(())
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
    pub(crate) fn define_word(&mut self, name: &str, value: Compiled) -> Result<(), Error> {
        if let Some(_) = self.dictionary.insert(name.to_string(), value) {
            return Err(Redefined(name.to_string()));
        }
        Ok(())
    }

    /// Get the compiled object associated to the word.
    pub(crate) fn get_word(&self, name: &str) -> Option<Compiled> {
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
