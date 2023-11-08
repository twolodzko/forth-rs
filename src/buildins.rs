use crate::{
    compiled::Compiled::{self, Callable, Constant},
    errors::Error::{self, StackUnderflow},
    forth::Forth,
};

const BUILDINS: &[(&str, Compiled)] = &[
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
fn add(forth: &mut Forth) -> Result<(), Error> {
    let a = forth.pop()?;
    let b = forth.pop()?;
    forth.stack.push(a.saturating_add(b));
    Ok(())
}

/// `cr (--)`
fn cr(_: &mut Forth) -> Result<(), Error> {
    println!();
    Ok(())
}

/// `swap (a b -- b a)`
fn swap(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.stack.len();
    if n < 2 {
        return Err(StackUnderflow);
    }
    forth.stack.swap(n - 1, n - 2);
    Ok(())
}

/// `dup (a -- a a)`
fn dup(forth: &mut Forth) -> Result<(), Error> {
    if let Some(val) = forth.stack.last() {
        forth.push(*val);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

/// `drop (a --)`
fn drop(forth: &mut Forth) -> Result<(), Error> {
    forth.pop()?;
    Ok(())
}
