use crate::{
    errors::Error::{self, CustomError, Exit, Leave, Quit, Redefined, StackUnderflow},
    expressions::Expr,
    numbers::Int,
    parser::Parser,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// The Forth interpreter that walks over the code and executes it.
pub struct Forth {
    /// Stack for storing the data.
    pub data_stack: Vec<Int>,
    /// The additional temporary memory.
    pub(crate) return_stack: Vec<Int>,
    /// Dictionary mapping words to functions, constants, etc.
    pub(crate) dictionary: HashMap<String, Expr>,
    /// Memory for storing data related to named variables.
    pub(crate) memory: Vec<Int>,
}

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity.
    pub(crate) fn empty(capacity: usize) -> Self {
        Self {
            data_stack: Vec::with_capacity(capacity),
            return_stack: Vec::new(),
            dictionary: HashMap::new(),
            memory: Vec::new(),
        }
    }

    /// Evaluate a string.
    pub fn eval_string(&mut self, string: &str) -> Result<(), Error> {
        let mut parser = Parser::from(string);
        while let Some(result) = self.eval_next(&mut parser) {
            result.or_else(|err| {
                self.data_stack.clear();
                if err == Quit || err == Exit || err == Leave {
                    return Ok(());
                }
                Err(err)
            })?;
        }
        Ok(())
    }

    /// Evaluate a file.
    pub fn eval_file(&mut self, path: &str) -> Result<(), Error> {
        let file = File::open(path).map_err(|msg| CustomError(msg.to_string()))?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.map_err(|msg| CustomError(msg.to_string()))?;
            self.eval_string(&line)?;
        }
        Ok(())
    }

    /// Go to next word and evaluate it.
    #[inline]
    pub(crate) fn eval_next(&mut self, parser: &mut Parser) -> Option<Result<(), Error>> {
        match parser.next()? {
            Ok(expr) => Some(expr.execute(self)),
            Err(msg) => Some(Err(msg)),
        }
    }

    /// Push value to the stack.
    #[inline]
    pub(crate) fn push(&mut self, value: Int) {
        self.data_stack.push(value)
    }

    /// Pop value from the stack.
    #[inline]
    pub(crate) fn pop(&mut self) -> Result<Int, Error> {
        self.data_stack.pop().ok_or(StackUnderflow)
    }

    /// Pop two values from the stack, return them in the order they were entered into the stack.
    #[inline]
    pub(crate) fn pop2(&mut self) -> Result<(Int, Int), Error> {
        let b = self.pop()?;
        let a = self.pop()?;
        Ok((a, b))
    }

    /// Define a new word, return an error on redefinition.
    #[inline]
    pub(crate) fn define_word(&mut self, name: &str, value: Expr) -> Result<(), Error> {
        if self.dictionary.insert(name.into(), value).is_some() {
            return Err(Redefined(name.into()));
        }
        Ok(())
    }

    /// Get the compiled object associated to the word.
    #[inline]
    pub(crate) fn get_word(&self, name: &str) -> Option<Expr> {
        self.dictionary.get(name).cloned()
    }

    /// The list of all the defined words.
    #[inline]
    pub(crate) fn words(&self) -> Vec<String> {
        let mut words = self.dictionary.keys().map(|s| s.into()).collect::<Vec<_>>();
        words.sort();
        words
    }
}
