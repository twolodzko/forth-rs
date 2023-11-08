use crate::{errors::Error, forth::Forth};

pub type Int = i32;

#[allow(dead_code)] // FIXME
#[derive(Clone)]
pub enum Compiled {
    Word(String),
    Variable(Int),
    Constant(Int),
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    Function(Function),
    IfThenElse(IfThenElse),
}

impl Compiled {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        match self {
            Compiled::Callable(exec) => exec(forth),
            Compiled::Word(word) => forth.evaluate(word),
            Compiled::Function(func) => func.execute(forth),
            Compiled::IfThenElse(body) => body.execute(forth),
            Compiled::Constant(val) => {
                forth.push(*val);
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}

fn execute_many(forth: &mut Forth, body: &[Compiled]) -> Result<(), Error> {
    for obj in body {
        obj.execute(forth)?;
    }
    Ok(())
}

#[derive(Clone)]
pub struct Function {
    body: Vec<Compiled>,
}

impl Function {
    fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        execute_many(forth, &self.body)
    }
}

#[derive(Clone)]
pub struct IfThenElse {
    then: Vec<Compiled>,
    otherwise: Vec<Compiled>,
}

impl IfThenElse {
    fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        if forth.pop()? != 0 {
            execute_many(forth, &self.then)
        } else {
            execute_many(forth, &self.otherwise)
        }
    }
}
