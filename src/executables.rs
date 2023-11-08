use crate::{errors::Error, forth::Forth};

pub type Int = i32;

#[derive(Clone)]
pub enum Executable {
    Word(String),
    #[allow(dead_code)]
    Integer(Int),
    #[allow(dead_code)]
    String(String),
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    NewFunction(String, Function),
    Function(Function),
    #[allow(dead_code)]
    IfThenElse(IfThenElse),
    NewConstant(String),
    Constant(Int),
    NewVariable(String),
    Variable(Int),
    CompileOnly,
}

impl Executable {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        use Executable::*;
        match self {
            Word(word) => forth.eval_word(word),
            Integer(val) => {
                forth.push(*val);
                Ok(())
            }
            String(string) => {
                print!("{}", string);
                Ok(())
            }
            Callable(exec) => exec(forth),
            NewFunction(name, func) => {
                let func = Executable::Function(func.clone());
                forth.define_word(name, func)
            }
            Function(func) => func.execute(forth),
            IfThenElse(body) => body.execute(forth),
            Constant(val) => {
                forth.push(*val);
                Ok(())
            }
            NewConstant(name) => {
                let value = Executable::Constant(forth.pop()?);
                forth.define_word(name, value)
            }
            NewVariable(name) => {
                let default = Executable::Variable(0);
                forth.define_word(name, default)
            }
            Variable(_) => Ok(()),
            CompileOnly => Ok(()),
        }
    }
}

fn execute_many(forth: &mut Forth, body: &[Executable]) -> Result<(), Error> {
    for obj in body {
        obj.execute(forth)?;
    }
    Ok(())
}

#[derive(Clone)]
pub struct Function {
    pub body: Vec<Executable>,
}

impl Function {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        execute_many(forth, &self.body)
    }
}

#[derive(Clone)]
pub struct IfThenElse {
    pub then: Vec<Executable>,
    pub other: Vec<Executable>,
}

impl IfThenElse {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        if forth.pop()? != 0 {
            execute_many(forth, &self.then)
        } else {
            execute_many(forth, &self.other)
        }
    }
}
