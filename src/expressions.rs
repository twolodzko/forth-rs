use std::{fmt::Display, str::FromStr};

use crate::{
    errors::Error::{self, CompileTimeWord, Exit, InvalidName, Leave, Recurse, UnknownWord},
    forth::Forth,
    numbers::Int,
};

macro_rules! maybe_break_loop {
    ( $expr:expr ) => {
        match $expr {
            Err(Leave) => break,
            result => result?,
        }
    };
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    /// Execute the function related to this word.
    Word(String),
    /// The string that is printed,
    Print(String),
    /// A builtin function.
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    /// Initialize a function and name it.
    NewFunction(String, Vec<Expr>),
    /// A function that can be executed.
    Function(Vec<Expr>),
    /// If-then-else block.
    IfElseThen(Vec<Expr>, Vec<Expr>),
    /// Begin loop
    Begin(Vec<Expr>),
    /// Do-loop.
    Loop(Vec<Expr>),
    /// Create a new constant.
    NewConstant(String),
    /// Push the value to the stack.
    Value(Int),
    /// Allocate memory and create a new constant holding the current memory address.
    NewVariable(String),
    /// Create a new constant holding the current memory address.
    NewCreate(String),
    /// Create a new value.
    NewValue(String),
    /// Update the value.
    ToValue(String),
    /// Read Forth script from the path.
    Include(String),
    /// Display the content of the word.
    See(String),
    /// Placeholder for a reserved word.
    Dummy,
}

impl Expr {
    /// Execute the expression.
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        use Expr::*;
        match self {
            Word(word) => match forth.get_word(word) {
                Some(compiled) => compiled.execute(forth),
                None => {
                    if let Ok(num) = Int::from_str(word) {
                        forth.push(num);
                        Ok(())
                    } else {
                        Err(UnknownWord(word.into()))
                    }
                }
            },
            Callable(exec) => exec(forth),
            NewFunction(name, func) => {
                let func = Function(func.clone());
                forth.define_word(name, func)
            }
            Function(body) => {
                let mut i = 0;
                while i < body.len() {
                    let obj = &body[i];
                    match obj.execute(forth) {
                        Err(Recurse) => {
                            i = 0;
                            continue;
                        }
                        Err(Exit) => return Ok(()),
                        other => other?,
                    }
                    i += 1;
                }
                Ok(())
            }
            IfElseThen(then, other) => {
                if forth.pop()?.is_true() {
                    execute_many(forth, then)
                } else {
                    execute_many(forth, other)
                }
            }
            Begin(body) => {
                loop {
                    maybe_break_loop!(execute_many(forth, body))
                }
                Ok(())
            }
            Loop(body) => {
                let (limit, index) = forth.pop2()?;
                for i in index.0..limit.0 {
                    forth.return_stack.push(Int(i));
                    maybe_break_loop!(execute_many(forth, body));
                    forth.return_stack.pop();
                }
                Ok(())
            }
            NewConstant(name) => {
                let value = forth.pop()?;
                forth.define_word(name, Value(value))
            }
            Value(val) => {
                forth.push(*val);
                Ok(())
            }
            NewVariable(name) => {
                forth.memory.push(Int(0));
                let addr = forth.memory.len() - 1;
                forth.define_word(name, Value(Int::from(addr)))?;
                Ok(())
            }
            NewCreate(name) => {
                let addr = forth.memory.len();
                forth.define_word(name, Value(Int::from(addr)))?;
                Ok(())
            }
            NewValue(name) => {
                forth.define_word(name, Value(Int(0)))?;
                Ok(())
            }
            ToValue(name) => {
                let value = forth.pop()?;
                match forth.dictionary.get_mut(name) {
                    Some(Value(val)) => {
                        *val = value;
                        Ok(())
                    }
                    Some(_) => Err(InvalidName(name.into())),
                    None => Err(UnknownWord(name.into())),
                }
            }
            Include(path) => forth.eval_file(path),
            Print(string) => {
                print!("{}", string);
                Ok(())
            }
            See(word) => {
                match forth.dictionary.get(word) {
                    Some(Dummy) => print!("<special word: {}>", word),
                    Some(func @ Function(_)) => print!(": {} {} ;", word, func),
                    Some(other) => print!("{}", other),
                    None => return Err(UnknownWord(word.into())),
                }
                Ok(())
            }
            Dummy => Err(CompileTimeWord),
        }
    }
}

/// Execute all the expressions in `body`.
#[inline]
fn execute_many(forth: &mut Forth, body: &[Expr]) -> Result<(), Error> {
    for obj in body {
        obj.execute(forth)?;
    }
    Ok(())
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[inline]
        fn vec_to_string(exprs: &[Expr]) -> String {
            exprs
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }

        use Expr::*;
        let string = match self {
            Word(string) => string.into(),
            Print(string) => format!(".\" {}\"", string),
            Callable(obj) => format!("<func: {:?}>", &obj),
            NewFunction(name, body) => format!(": {} {} ;", name, vec_to_string(body)),
            Function(body) => vec_to_string(body),
            IfElseThen(then, other) => {
                if other.is_empty() {
                    format!("if {} then", vec_to_string(then))
                } else {
                    format!(
                        "if {} else {} then",
                        vec_to_string(then),
                        vec_to_string(other)
                    )
                }
            }
            Begin(body) => format!("begin {}", vec_to_string(body)),
            Loop(body) => format!("do {} loop", vec_to_string(body)),
            NewConstant(name) => format!("constant {}", name),
            Value(val) => format!("{}", val),
            NewVariable(name) => format!("variable {}", name),
            NewCreate(name) => format!("create {}", name),
            NewValue(name) => format!("value {}", name),
            Include(path) => format!("include {}", path),
            See(word) => format!("see {}", word),
            Dummy | ToValue(_) => unreachable!(),
        };
        write!(f, "{}", string)
    }
}
