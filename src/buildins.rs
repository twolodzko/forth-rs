use crate::{
    errors::Error::{self, StackUnderflow},
    forth::Forth,
    objects::{
        Int,
        Object::{self, Callable, CompileOnly, Constant},
    },
};

const BUILDINS: &[(&str, Object)] = &[
    // constants
    ("true", Constant(-1)),
    ("false", Constant(0)),
    // callables
    ("+", Callable(add)),
    ("cr", Callable(cr)),
    ("dup", Callable(dup)),
    ("drop", Callable(drop)),
    ("swap", Callable(swap)),
    (".", Callable(dot)),
    ("emit", Callable(emit)),
    (".s", Callable(print_stack)),
    ("words", Callable(words)),
    // compile-only words
    ("if", CompileOnly),
    ("then", CompileOnly),
    ("else", CompileOnly),
    (";", CompileOnly),
    (":", CompileOnly),
    ("variable", CompileOnly),
    ("constant", CompileOnly),
    (".(", CompileOnly),
    (".\"", CompileOnly),
    // ("do", CompileOnly),
    // ("begin", CompileOnly),
    // ("loop", CompileOnly),
    // ("+loop", CompileOnly),
    // ("again", CompileOnly),
    // ("while", CompileOnly),
    // ("until", CompileOnly),
];

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity and
    /// a dictionary of predefined words.
    pub fn new(capacity: usize) -> Self {
        let mut forth = Forth::with_capacity(capacity);
        for (key, val) in BUILDINS {
            let _ = forth.define_word(key, val.clone());
        }
        forth
    }

    #[inline]
    fn pop2(&mut self) -> Result<(Int, Int), Error> {
        Ok((self.pop()?, self.pop()?))
    }
}

/// `+ (a b -- c)`
fn add(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
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

/// `.s (--)`
fn print_stack(forth: &mut Forth) -> Result<(), Error> {
    let show_max = 10;
    let stack = forth
        .stack
        .iter()
        .take(show_max)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let n = forth.stack.len();
    let dots = if n > show_max { "..." } else { "" };
    print!("<{}> {}{}", forth.stack.len(), stack, dots);
    Ok(())
}

/// `words (--)`
fn words(forth: &mut Forth) -> Result<(), Error> {
    print!("{}", forth.words().join(" "));
    Ok(())
}

/// `. (-- a)`
fn dot(forth: &mut Forth) -> Result<(), Error> {
    print!("{} ", forth.pop()?);
    Ok(())
}

/// `emit (a --)`
fn emit(forth: &mut Forth) -> Result<(), Error> {
    let val = forth.pop()?;
    if let Ok(u) = val.try_into() {
        if let Some(c) = char::from_u32(u) {
            print!("{}", c);
            return Ok(());
        }
    }
    print!("�");
    Ok(())
}
