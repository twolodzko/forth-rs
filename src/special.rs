use crate::{
    errors::Error,
    forth::Forth,
    memory::{ForthResult, Memory},
};

#[derive(Clone)]
pub struct Function {
    body: Vec<String>,
}

impl Function {
    #[allow(dead_code)] // FIXME
    /// Create new function
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|w| w.to_string()).collect(),
        }
    }

    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        for word in self.body {
            forth.execute(&word)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct IfThenElse {
    then: Vec<String>,
    otherwise: Vec<String>,
}

impl IfThenElse {
    pub fn execute(&mut self, forth: &mut Memory) -> ForthResult {
        if forth.pop()? != 0 {
            forth.execute_many(&self.then)
        } else {
            forth.execute_many(&self.otherwise)
        }
    }
}
