use std::fmt::Display;

use crate::{
    compiled,
    errors::Error::{self, CompileTimeWord, UnknownWord},
    forth::Forth,
};

pub type Int = i32;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    /// Execute the function related to this word.
    Word(String),
    /// The string that is printed,
    Print(String),
    /// A builtin function.
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    /// Initialize a function and name it.
    NewFunction(String, compiled::Function),
    /// A function that can be executed.
    Function(compiled::Function),
    /// If-then-else block.
    IfElseThen(compiled::IfElseThen),
    /// Begin loop
    Begin(compiled::Begin),
    /// Do-loop.
    Loop(compiled::Loop),
    /// Create a new constant.
    NewConstant(String),
    /// Push the constant to the stack.
    Constant(Int),
    /// Create a new constant holding the memory address of the variable.
    NewVariable(String),
    /// Read Forth script from the path.
    Include(String),
    /// Display the content of the word.
    See(String),
    /// Placeholder for a reserved word.
    Dummy,
}

impl Expr {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        use Expr::*;
        match self {
            Word(word) => match forth.get_word(word) {
                Some(compiled) => compiled.execute(forth),
                None => {
                    if let Ok(num) = word.parse::<Int>() {
                        forth.push(num);
                        Ok(())
                    } else {
                        Err(UnknownWord(word.to_string()))
                    }
                }
            },
            Print(string) => {
                print!("{}", string);
                Ok(())
            }
            Callable(exec) => exec(forth),
            NewFunction(name, func) => {
                let func = Function(func.clone());
                forth.define_word(name, func)
            }
            Function(func) => func.execute(forth),
            IfElseThen(body) => body.execute(forth),
            Begin(body) => body.execute(forth),
            Loop(body) => body.execute(forth),
            NewConstant(name) => {
                let value = forth.pop()?;
                forth.define_word(name, Constant(value))
            }
            Constant(val) => {
                forth.push(*val);
                Ok(())
            }
            NewVariable(name) => {
                forth.memory.push(0);
                let addr = (forth.memory.len() - 1) as i32;
                forth.define_word(name, Constant(addr))?;
                Ok(())
            }
            Include(path) => forth.eval_file(path),
            See(word) => {
                match forth.dictionary.get(word) {
                    Some(Dummy) => println!("<compiled-word: {}>", word),
                    Some(func @ Function(_)) => println!(": {} {} ;", word, func),
                    Some(other) => println!("{}", other),
                    None => return Err(UnknownWord(word.to_string())),
                }
                Ok(())
            }
            Dummy => Err(CompileTimeWord),
        }
    }
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
            Word(string) => string.to_string(),
            Print(string) => format!(".\" {}\"", string),
            Callable(obj) => format!("<func: {:?}>", &obj),
            NewFunction(name, obj) => format!(": {} {} ;", name, vec_to_string(&obj.body)),
            Function(obj) => vec_to_string(&obj.body),
            IfElseThen(body) => {
                if body.other.is_empty() {
                    format!("if {} then", vec_to_string(&body.then))
                } else {
                    format!(
                        "if {} else {} then",
                        vec_to_string(&body.then),
                        vec_to_string(&body.other)
                    )
                }
            }
            Begin(obj) => format!("begin {}", vec_to_string(&obj.body)),
            Loop(obj) => format!("do {} loop", vec_to_string(&obj.body)),
            NewConstant(name) => format!("constant {}", name),
            Constant(val) => format!("{}", val),
            NewVariable(name) => format!("variable {}", name),
            Include(path) => format!("include {}", path),
            See(word) => format!("see {}", word),
            Dummy => unreachable!(),
        };
        write!(f, "{}", string)
    }
}
