use crate::{
    errors::Error::{self, UnknownWord},
    forth::Forth,
};

pub type Int = i32;

#[derive(Clone)]
pub enum Expr {
    Word(String),
    Print(String),
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    NewFunction(String, Function),
    Function(Function),
    IfThenElse(IfThenElse),
    NewConstant(String),
    Constant(Int),
    NewVariable(String),
    Variable(Int),
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
                let value = Constant(forth.pop()?);
                forth.define_word(name, value)
            }
            NewVariable(name) => {
                let default = Variable(0);
                forth.define_word(name, default)
            }
            Variable(_) => Ok(()),
            Dummy => Ok(()),
        }
    }
}

#[inline]
fn execute_many(forth: &mut Forth, body: &[Expr]) -> Result<(), Error> {
    for obj in body {
        obj.execute(forth)?;
    }
    Ok(())
}

#[derive(Clone)]
pub struct Function {
    pub body: Vec<Expr>,
}

impl Function {
    #[inline]
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        execute_many(forth, &self.body)
    }
}

#[derive(Clone)]
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
