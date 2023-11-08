use crate::{errors::Error, forth::Forth};

pub type Int = i32;

#[allow(dead_code)] // FIXME
#[derive(Clone)]
pub enum Compiled {
    Word(String),
    Variable(Int),
    Constant(Int),
    Callable(fn(forth: &mut Forth) -> Result<(), Error>),
    Function(imp::Function),
    IfThenElse(imp::IfThenElse),
    // Loop(Loop),
    // PlusLoop(PlusLoop),
    // Begin(Begin),
}

impl Compiled {
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        use Compiled::{Callable, Constant, Function, IfThenElse, Variable, Word};
        match self {
            Callable(exec) => exec(forth),
            Word(word) => forth.eval(word),
            Function(func) => func.execute(forth),
            IfThenElse(body) => body.execute(forth),
            Constant(val) => {
                forth.push(*val);
                Ok(())
            }
            Variable(_) => Ok(()),
        }
    }
}

pub mod imp {
    use super::Compiled;
    use crate::{errors::Error, forth::Forth};

    fn execute_many(forth: &mut Forth, body: &[Compiled]) -> Result<(), Error> {
        for obj in body {
            obj.execute(forth)?;
        }
        Ok(())
    }

    #[derive(Clone)]
    pub struct Function {
        pub body: Vec<Compiled>,
    }

    impl Function {
        pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
            execute_many(forth, &self.body)
        }
    }

    #[derive(Clone)]
    pub struct IfThenElse {
        pub then: Vec<Compiled>,
        pub otherwise: Vec<Compiled>,
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
}
