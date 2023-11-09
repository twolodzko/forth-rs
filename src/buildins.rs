use crate::{
    errors::Error::{self, StackUnderflow},
    expressions::{
        Expr::{self, Callable, Constant, Dummy},
        Int,
    },
    forth::Forth,
};

const BUILDINS: &[(&str, Expr)] = &[
    // constants
    ("true", Constant(-1)),
    ("false", Constant(0)),
    // math
    ("+", Callable(add)),
    ("-", Callable(sub)),
    ("*", Callable(mul)),
    ("/", Callable(div)),
    ("mod", Callable(rem)),
    ("/mod", Callable(div_rem)),
    ("abs", Callable(abs)),
    ("negate", Callable(negate)),
    // comparisons
    ("=", Callable(eq)),
    ("<>", Callable(ne)),
    ("<", Callable(lt)),
    (">", Callable(gt)),
    ("and", Callable(and)),
    ("or", Callable(or)),
    // stack
    ("dup", Callable(dup)),
    ("drop", Callable(drop)),
    ("swap", Callable(swap)),
    ("rot", Callable(rot)),
    ("over", Callable(over)),
    // i/o
    ("cr", Callable(cr)),
    (".", Callable(dot)),
    ("emit", Callable(emit)),
    // helpers
    (".s", Callable(print_stack)),
    ("words", Callable(words)),
    // compile-only words and the words handled specially by parser
    ("if", Dummy),
    ("then", Dummy),
    ("else", Dummy),
    (";", Dummy),
    (":", Dummy),
    ("variable", Dummy),
    ("constant", Dummy),
    (".(", Dummy),
    (".\"", Dummy),
    // ("do", Dummy),
    // ("begin", Dummy),
    // ("loop", Dummy),
    // ("+loop", Dummy),
    // ("again", Dummy),
    // ("while", Dummy),
    // ("until", Dummy),
    // ("leave", Dummy),
];

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity and
    /// a dictionary of predefined words.
    pub fn new(capacity: usize) -> Self {
        let mut forth = Forth::with_capacity(capacity);
        for (key, val) in BUILDINS {
            forth
                .define_word(key, val.clone())
                .expect("there should be no duplicate definitions");
        }
        forth
    }

    #[inline]
    fn pop2(&mut self) -> Result<(Int, Int), Error> {
        Ok((self.pop()?, self.pop()?))
    }
}

/// `+ (n1 n2 -- sum)`
fn add(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(b.saturating_add(a));
    Ok(())
}

/// `- (n1 n2 -- diff)`
fn sub(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(b.saturating_sub(a));
    Ok(())
}

/// `* (n1 n2 -- prod)`
fn mul(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(b.saturating_mul(a));
    Ok(())
}

/// `/ (n1 n2 -- quot)`
fn div(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(b / a);
    Ok(())
}

/// `mod (n1 n2 -- rem)`
fn rem(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(b % a);
    Ok(())
}

/// `/mod (n1 n2 -- rem quot)`
fn div_rem(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(b / a);
    forth.stack.push(b % a);
    Ok(())
}

/// `abs (n -- u)`
fn abs(forth: &mut Forth) -> Result<(), Error> {
    let num = forth.pop()?;
    forth.push(num.abs());
    Ok(())
}

/// `negate (-n|+n -- +n|-n)`
fn negate(forth: &mut Forth) -> Result<(), Error> {
    let num = forth.pop()?;
    forth.push(-num);
    Ok(())
}

/// `= (n1 n2 -- flag)`
fn eq(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(if b == a { -1 } else { 0 });
    Ok(())
}

/// `<> (n1 n2 -- flag)`
fn ne(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(if b != a { -1 } else { 0 });
    Ok(())
}

/// `< (n1 n2 -- flag)`
fn lt(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(if b < a { -1 } else { 0 });
    Ok(())
}

/// `> (n1 n2 -- flag)`
fn gt(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(if b > a { -1 } else { 0 });
    Ok(())
}

/// `and (n1 n2 -- n3)`
fn and(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(if b != 0 { a } else { b });
    Ok(())
}

/// `or (n1 n2 -- n3)`
fn or(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.stack.push(if b != 0 { b } else { a });
    Ok(())
}

/// `swap (n1 n2 -- n2 n1)`
fn swap(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.stack.len();
    if n < 2 {
        return Err(StackUnderflow);
    }
    forth.stack.swap(n - 1, n - 2);
    Ok(())
}

/// `dup (n -- n n)`
fn dup(forth: &mut Forth) -> Result<(), Error> {
    if let Some(val) = forth.stack.last() {
        forth.push(*val);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

/// `drop (n --)`
fn drop(forth: &mut Forth) -> Result<(), Error> {
    forth.pop()?;
    Ok(())
}

/// `rot (n1 n2 n3 -- n2 n3 n1)`
fn rot(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.stack.len();
    if n < 3 {
        return Err(StackUnderflow);
    }
    forth.stack.swap(n - 2, n - 3);
    forth.stack.swap(n - 1, n - 2);
    Ok(())
}

/// `over (n1 n2 -- n1 n2 n1)`
fn over(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.stack.len();
    match forth.stack.get(n - 2) {
        None => Err(StackUnderflow),
        Some(val) => {
            forth.push(*val);
            Ok(())
        }
    }
}

/// `cr (--)`
fn cr(_: &mut Forth) -> Result<(), Error> {
    println!();
    Ok(())
}

/// `. (n --)`
fn dot(forth: &mut Forth) -> Result<(), Error> {
    print!("{}", forth.pop()?);
    Ok(())
}

/// `emit (n --)`
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
    print!(" <{}> {}{}", forth.stack.len(), stack, dots);
    Ok(())
}

/// `words (--)`
fn words(forth: &mut Forth) -> Result<(), Error> {
    print!("{}", forth.words().join(" "));
    Ok(())
}
