use crate::errors::Error::StackUnderflow;
use crate::forth::{
    Definition::{self, Callable, Constant},
    Forth, ForthResult,
};

const BUILDINS: &[(&str, Definition)] = &[
    // constants
    ("true", Constant(-1)),
    ("false", Constant(0)),
    // callables
    ("+", Callable(add)),
    ("cr", Callable(cr)),
    ("dup", Callable(dup)),
    ("drop", Callable(drop)),
    ("swap", Callable(swap)),
];

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity and
    /// a dictionary of predefined words.
    pub fn new(capacity: usize) -> Self {
        let mut forth = Forth::with_capacity(capacity);
        for (key, val) in BUILDINS {
            forth.dictionary.insert(key.to_string(), val.clone());
        }
        forth
    }
}

/// `+ (a b -- c)`
fn add(forth: &mut Forth) -> ForthResult {
    let a = forth.pop()?;
    let b = forth.pop()?;
    forth.stack.push(a.saturating_add(b));
    Ok(())
}

/// `cr (--)`
fn cr(_: &mut Forth) -> ForthResult {
    println!();
    Ok(())
}

/// `swap (a b -- b a)`
fn swap(forth: &mut Forth) -> ForthResult {
    let n = forth.stack.len();
    if n < 2 {
        return Err(StackUnderflow);
    }
    forth.stack.swap(n - 1, n - 2);
    Ok(())
}

/// `dup (a -- a a)`
fn dup(forth: &mut Forth) -> ForthResult {
    if let Some(val) = forth.stack.last() {
        forth.push(*val);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

/// `drop (a --)`
fn drop(forth: &mut Forth) -> ForthResult {
    forth.pop()?;
    Ok(())
}
