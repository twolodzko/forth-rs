use crate::{errors::Error, forth::Forth};

pub type Int = i32;

#[derive(Clone)]
pub enum Object {
    Word(String),
    #[allow(dead_code)]
    Int(Int),
    #[allow(dead_code)]
    String(String),
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    Function(Function),
    IfThenElse(IfThenElse),
    Constant(Int),
    Variable(Int),
    CompileOnly,
}

impl Object {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        use Object::*;
        match self {
            Word(word) => forth.eval_word(word),
            Int(val) => {
                forth.push(*val);
                Ok(())
            }
            String(string) => {
                print!("{}", string);
                Ok(())
            }
            Callable(exec) => exec(forth),
            Function(func) => func.execute(forth),
            IfThenElse(body) => body.execute(forth),
            Constant(val) => {
                forth.push(*val);
                Ok(())
            }
            Variable(_) => Ok(()),
            CompileOnly => Ok(()),
        }
    }
}

fn execute_many(forth: &mut Forth, body: &[Object]) -> Result<(), Error> {
    for obj in body {
        obj.execute(forth)?;
    }
    Ok(())
}

#[derive(Clone)]
pub struct Function {
    pub body: Vec<Object>,
}

impl Function {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        execute_many(forth, &self.body)
    }
}

#[derive(Clone)]
pub struct IfThenElse {
    pub then: Vec<Object>,
    pub otherwise: Vec<Object>,
}

impl IfThenElse {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        if forth.pop()? != 0 {
            execute_many(forth, &self.then)
        } else {
            execute_many(forth, &self.otherwise)
        }
    }
}
