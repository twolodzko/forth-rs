use crate::{
    errors::Error::{self, DivisionByZero, InvalidAddress, LeaveLoop, StackUnderflow},
    expressions::Expr::{self, Callable, Constant, Dummy},
    forth::Forth,
};

const BUILDINS: &[(&str, Expr)] = &[
    // logic
    ("true", Constant(-1)),
    ("false", Constant(0)),
    ("not", Callable(not)),
    ("and", Callable(and)),
    ("or", Callable(or)),
    ("xor", Callable(xor)),
    // comparisons
    ("=", Callable(eq)),
    ("<>", Callable(ne)),
    ("<", Callable(lt)),
    (">", Callable(gt)),
    // math
    ("+", Callable(add)),
    ("-", Callable(sub)),
    ("*", Callable(mul)),
    ("/", Callable(div)),
    ("*/", Callable(mul_div)),
    ("*/mod", Callable(mul_div_rem)),
    ("mod", Callable(rem)),
    ("/mod", Callable(div_rem)),
    ("abs", Callable(abs)),
    ("negate", Callable(negate)),
    ("1+", Callable(add1)),
    ("1-", Callable(sub1)),
    ("2*", Callable(mul2)),
    ("2/", Callable(div2)),
    // data stack
    ("dup", Callable(dup)),
    ("drop", Callable(drop)),
    ("swap", Callable(swap)),
    ("pick", Callable(pick)),
    ("roll", Callable(roll)),
    ("rot", Callable(rot)),
    ("over", Callable(over)),
    ("depth", Callable(depth)),
    (".s", Callable(print_stack)),
    ("clearstack", Callable(clearstack)),
    // return stack
    (">r", Callable(to_return)),
    ("r>", Callable(from_return)),
    ("r@", Callable(copy_from_return)),
    // constants and variables
    ("constant", Dummy),
    ("variable", Dummy),
    ("!", Callable(set)),
    ("@", Callable(fetch)),
    // i/o
    ("cr", Callable(cr)),
    (".", Callable(dot)),
    ("emit", Callable(emit)),
    // helpers
    ("words", Callable(words)),
    ("see", Dummy),
    // compile-only words and the words handled specially by parser
    ("if", Dummy),
    ("then", Dummy),
    ("else", Dummy),
    (";", Dummy),
    (":", Dummy),
    (".(", Dummy),
    (".\"", Dummy),
    ("include", Dummy),
    // looping
    ("leave", Callable(leave)),
    ("while", Callable(while_cond)),
    ("until", Callable(until)),
    ("begin", Dummy),
    ("again", Dummy),
    ("do", Dummy),
    ("loop", Dummy),
    ("i", Callable(copy_from_return)),
    ("j", Callable(loop_j)),
];

impl Forth {
    /// Constructs a new, empty Forth server with the stack with at least the specified capacity and
    /// a dictionary of predefined words.
    pub fn new(capacity: usize) -> Self {
        let mut forth = Forth::empty(capacity);
        for (key, val) in BUILDINS {
            forth
                .define_word(key, val.clone())
                .expect("there should be no duplicate definitions");
        }
        forth
    }
}

/// `+ (n1 n2 -- sum)`
fn add(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(a.saturating_add(b));
    Ok(())
}

/// `- (n1 n2 -- diff)`
fn sub(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(a.saturating_sub(b));
    Ok(())
}

/// `* (n1 n2 -- prod)`
fn mul(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(a.saturating_mul(b));
    Ok(())
}

/// `/ (n1 n2 -- quot)`
fn div(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    if b == 0 {
        return Err(DivisionByZero);
    }
    forth.push(a / b);
    Ok(())
}

/// `mod (n1 n2 -- rem)`
fn rem(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    if b == 0 {
        return Err(DivisionByZero);
    }
    forth.push(a % b);
    Ok(())
}

/// `/mod (n1 n2 -- rem quot)`
fn div_rem(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    if b == 0 {
        return Err(DivisionByZero);
    }
    forth.push(a % b);
    forth.push(a / b);
    Ok(())
}

/// Transform i64 to i32 rounding the i64 values to the limits of i32.
#[inline]
fn saturating_i64_to_i32(value: i64) -> i32 {
    if value < i32::MIN as i64 {
        i32::MIN
    } else if value > i32::MAX as i64 {
        i32::MAX
    } else {
        value as i32
    }
}

/// `*/ (n1 n2 n3 -- n4)`
/// `n1 * n2 / n3`, but make the calculation using double precision numbers.
fn mul_div(forth: &mut Forth) -> Result<(), Error> {
    let c = forth.pop()?;
    if c == 0 {
        return Err(DivisionByZero);
    }
    let (a, b) = forth.pop2()?;
    let (a, b, c) = (a as i64, b as i64, c as i64);
    forth.push(saturating_i64_to_i32(a * b / c));
    Ok(())
}

/// `*/mod (n1 n2 n3 -- n4 n5)`
/// `n1 * n2 / n3` and `n1 * n2 % n3`, but make the calculation using double precision numbers.
fn mul_div_rem(forth: &mut Forth) -> Result<(), Error> {
    let c = forth.pop()?;
    if c == 0 {
        return Err(DivisionByZero);
    }
    let (a, b) = forth.pop2()?;
    let (a, b, c) = (a as i64, b as i64, c as i64);
    forth.push(saturating_i64_to_i32(a * b % c));
    forth.push(saturating_i64_to_i32(a * b / c));
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

/// `1+ (n -- sum)`
fn add1(forth: &mut Forth) -> Result<(), Error> {
    let a = forth.pop()?;
    forth.push(a.saturating_add(1));
    Ok(())
}

/// `1- (n -- diff)`
fn sub1(forth: &mut Forth) -> Result<(), Error> {
    let a = forth.pop()?;
    forth.push(a.saturating_sub(1));
    Ok(())
}

/// `2* (n -- prod)`
fn mul2(forth: &mut Forth) -> Result<(), Error> {
    let a = forth.pop()?;
    forth.push(a << 1);
    Ok(())
}

/// `2/ (n -- quot)`
fn div2(forth: &mut Forth) -> Result<(), Error> {
    let a = forth.pop()?;
    forth.push(a >> 1);
    Ok(())
}

/// `= (n1 n2 -- flag)`
fn eq(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(if a == b { -1 } else { 0 });
    Ok(())
}

/// `<> (n1 n2 -- flag)`
fn ne(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(if a != b { -1 } else { 0 });
    Ok(())
}

/// `< (n1 n2 -- flag)`
fn lt(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(if a < b { -1 } else { 0 });
    Ok(())
}

/// `> (n1 n2 -- flag)`
fn gt(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(if a > b { -1 } else { 0 });
    Ok(())
}

/// `not (n -- n)`
fn not(forth: &mut Forth) -> Result<(), Error> {
    let a = forth.pop()?;
    forth.push(if a != 0 { 0 } else { -1 });
    Ok(())
}

/// `and (n1 n2 -- n3)`
fn and(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(if a != 0 { b } else { a });
    Ok(())
}

/// `or (n1 n2 -- n3)`
fn or(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(if a != 0 { a } else { b });
    Ok(())
}

/// `xor (n1 n2 -- n3)`
fn xor(forth: &mut Forth) -> Result<(), Error> {
    let (a, b) = forth.pop2()?;
    forth.push(-((a != 0) as i32 ^ (b != 0) as i32));
    Ok(())
}

/// `swap (n1 n2 -- n2 n1)`
/// Swap the two values on the top of the stack.
fn swap(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.data_stack.len();
    if n < 2 {
        return Err(StackUnderflow);
    }
    forth.data_stack.swap(n - 1, n - 2);
    Ok(())
}

/// `dup (n -- n n)`
/// Duplicate value from the top of the stack.
fn dup(forth: &mut Forth) -> Result<(), Error> {
    if let Some(val) = forth.data_stack.last() {
        forth.push(*val);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

/// `pick (ni ... n0 i -- ni ... n0 ni)`
/// Copy i-th value to the top of the stack.
fn pick(forth: &mut Forth) -> Result<(), Error> {
    let index = forth.pop()?;
    let n = forth.data_stack.len();
    if index < 0 || n <= index as usize {
        return Err(StackUnderflow);
    }
    let value = forth.data_stack.get(n - index as usize).unwrap();
    forth.push(*value);
    Ok(())
}

/// `roll (ni ... n0 i -- ni-1 ... n0 ni)`
/// Move i-th value to the top of the stack.
fn roll(forth: &mut Forth) -> Result<(), Error> {
    let index = forth.pop()?;
    let n = forth.data_stack.len();
    if index < 0 || n <= index as usize {
        return Err(StackUnderflow);
    }
    let value = forth.data_stack.remove(n - index as usize);
    forth.push(value);
    Ok(())
}

/// `clearstack (--)`
fn clearstack(forth: &mut Forth) -> Result<(), Error> {
    forth.data_stack.clear();
    Ok(())
}

/// `drop (n --)`
/// Drop the value from the top of the stack.
fn drop(forth: &mut Forth) -> Result<(), Error> {
    forth.pop()?;
    Ok(())
}

/// `rot (n1 n2 n3 -- n2 n3 n1)`
fn rot(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.data_stack.len();
    if n < 3 {
        return Err(StackUnderflow);
    }
    forth.data_stack.swap(n - 2, n - 3);
    forth.data_stack.swap(n - 1, n - 2);
    Ok(())
}

/// `over (n1 n2 -- n1 n2 n1)`
fn over(forth: &mut Forth) -> Result<(), Error> {
    let n = forth.data_stack.len();
    if n < 2 {
        return Err(StackUnderflow);
    }
    let val = forth.data_stack.get(n - 2).unwrap();
    forth.push(*val);
    Ok(())
}

/// `cr (--)`
/// Print newline.
fn cr(_: &mut Forth) -> Result<(), Error> {
    println!();
    Ok(())
}

/// `. (n --)`
/// Take the value from the top of the stack and print it followed by space.
fn dot(forth: &mut Forth) -> Result<(), Error> {
    print!("{} ", forth.pop()?);
    Ok(())
}

/// `emit (n --)`
/// Take the value from the top of the stack and print it as a character.
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
/// Print the stack and it's size.
fn print_stack(forth: &mut Forth) -> Result<(), Error> {
    let show_max = 10;
    let stack = forth
        .data_stack
        .iter()
        .take(show_max)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let n = forth.data_stack.len();
    let dots = if n > show_max { "..." } else { "" };
    print!(" <{}> {}{}", forth.data_stack.len(), stack, dots);
    Ok(())
}

/// `words (--)`
/// Print all the available words.
fn words(forth: &mut Forth) -> Result<(), Error> {
    print!("{}", forth.words().join(" "));
    Ok(())
}

/// `! (n addr --)`
/// Set the the variable at addr to n.
fn set(forth: &mut Forth) -> Result<(), Error> {
    let (val, addr) = forth.pop2()?;
    let addr = addr as usize;
    if addr >= forth.memory.len() {
        return Err(InvalidAddress);
    }
    forth.memory.insert(addr, val);
    Ok(())
}

/// `@ (addr -- n)`
/// Get the value of the variable at addr.
fn fetch(forth: &mut Forth) -> Result<(), Error> {
    let addr = forth.pop()? as usize;
    let val = forth.memory.get(addr).ok_or(InvalidAddress)?;
    forth.push(*val);
    Ok(())
}

/// `leave (--)`
/// Break the loop.
fn leave(_: &mut Forth) -> Result<(), Error> {
    Err(LeaveLoop)
}

/// `while (n --)`
/// If flag is false, break the loop.
fn while_cond(forth: &mut Forth) -> Result<(), Error> {
    let flag = forth.pop()?;
    if flag == 0 {
        return Err(LeaveLoop);
    }
    Ok(())
}

/// `until (n --)`
/// If flag is true, break the loop.
fn until(forth: &mut Forth) -> Result<(), Error> {
    let flag = forth.pop()?;
    if flag != 0 {
        return Err(LeaveLoop);
    }
    Ok(())
}

/// `depth (-- n)`
/// The depth of the stack.
fn depth(forth: &mut Forth) -> Result<(), Error> {
    forth.push(forth.data_stack.len() as i32);
    Ok(())
}

/// `>r (n --)`
/// Take a value off the data stack and push it onto the return stack.
pub fn to_return(forth: &mut Forth) -> Result<(), Error> {
    let value = forth.pop()?;
    forth.return_stack.push(value);
    Ok(())
}

/// `r> (-- n)`
/// Take a value off the return stack and push it onto the data stack.
pub fn from_return(forth: &mut Forth) -> Result<(), Error> {
    let value = forth.return_stack.pop().ok_or(StackUnderflow)?;
    forth.push(value);
    Ok(())
}

/// `r@ (-- n)`
/// Copy the last value from return stack and push it onto the data stack.
pub fn copy_from_return(forth: &mut Forth) -> Result<(), Error> {
    let value = forth.return_stack.last().ok_or(StackUnderflow)?;
    forth.push(*value);
    Ok(())
}

/// `j (-- n)`
fn loop_j(forth: &mut Forth) -> Result<(), Error> {
    if forth.return_stack.len() < 2 {
        return Err(StackUnderflow);
    }
    let index = forth.return_stack.len() - 2;
    let value = forth.return_stack.get(index).unwrap();
    forth.push(*value);
    Ok(())
}

// FIXME
// /// Treat signed integer bytes as representation of a unsigned integer.
// fn signed_to_unsigned(value: i32) -> u32 {
//     let bytes = value.to_ne_bytes();
//     u32::from_be_bytes(bytes)
// }

// /// Treat unsigned integer bytes as representation of a signed integer.
// fn unsigned_to_signed(value: u32) -> i32 {
//     let bytes = value.to_ne_bytes();
//     i32::from_be_bytes(bytes)
// }
