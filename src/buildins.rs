use crate::errors::Error;
use crate::forth::{Definition, Forth, ForthResult};

const BUILDINS: &[(&str, Definition)] = &[("+", add)];

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity and
    /// a dictionary of predefined words.
    pub fn new(capacity: usize) -> Self {
        let mut forth = Forth::with_capacity(capacity);
        for (key, val) in BUILDINS {
            forth.dictionary.insert(key.to_string(), *val);
        }
        forth
    }
}

/// `+ (a b -- c)`
fn add(forth: &mut Forth) -> ForthResult {
    let x = forth.stack.pop().ok_or(Error::StackUnderflow)?;
    let y = forth.stack.pop().ok_or(Error::StackUnderflow)?;
    forth.stack.push(x + y);
    Ok(())
}
