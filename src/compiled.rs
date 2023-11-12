use crate::{
    errors::Error::{self, LeaveLoop},
    expressions::Expr,
    forth::Forth,
};

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
pub struct IfElseThen {
    pub then: Vec<Expr>,
    pub other: Vec<Expr>,
}

impl IfElseThen {
    #[inline]
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        if forth.pop()? != 0 {
            execute_many(forth, &self.then)
        } else {
            execute_many(forth, &self.other)
        }
    }
}

macro_rules! maybe_break_loop {
    ( $expr:expr ) => {
        match $expr {
            Err(LeaveLoop) => break,
            result => result?,
        }
    };
}

#[derive(Clone, PartialEq, Debug)]
pub struct Begin {
    pub body: Vec<Expr>,
}

impl Begin {
    #[inline]
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        loop {
            maybe_break_loop!(execute_many(forth, &self.body))
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Loop {
    pub body: Vec<Expr>,
}

impl Loop {
    #[inline]
    pub fn execute(&self, forth: &mut Forth) -> Result<(), Error> {
        let (limit, index) = forth.pop2()?;
        for i in index..limit {
            forth.return_stack.push(i);
            maybe_break_loop!(execute_many(forth, &self.body));
            forth.return_stack.pop();
        }
        Ok(())
    }
}
