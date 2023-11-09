use crate::{
    errors::Error::{self, UnknownWord},
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
    NewFunction(String, imp::Function),
    /// A function that can be executed.
    Function(imp::Function),
    /// If-then-else block.
    IfThenElse(imp::IfThenElse),
    /// Create a new constant.
    NewConstant(String),
    /// Push the constant to the stack.
    Constant(Int),
    /// Create a new constant holding the memory address of the variable.
    NewVariable(String),
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
            IfThenElse(body) => body.execute(forth),
            Constant(val) => {
                forth.push(*val);
                Ok(())
            }
            NewConstant(name) => {
                let value = forth.pop()?;
                forth.define_word(name, Constant(value))
            }
            NewVariable(name) => forth.insert_variable(name, 0),
            Dummy => Ok(()),
        }
    }
}

pub mod imp {
    use super::Expr;
    use crate::{errors::Error, forth::Forth};

    #[inline]
    fn execute_many(forth: &mut Forth, body: &[Expr]) -> Result<(), Error> {
        for obj in body {
            obj.execute(forth)?;
        }
        Ok(())
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Function {
        pub body: Vec<Expr>,
    }

    impl Function {
        #[inline]
        pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
            execute_many(forth, &self.body)
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct IfThenElse {
        pub then: Vec<Expr>,
        pub other: Vec<Expr>,
    }

    impl IfThenElse {
        #[inline]
        pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
            if forth.pop()? != 0 {
                execute_many(forth, &self.then)
            } else {
                execute_many(forth, &self.other)
            }
        }
    }
}
