use crate::{
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
    NewFunction(String, imp::Function),
    /// A function that can be executed.
    Function(imp::Function),
    /// If-then-else block.
    IfElseThen(imp::IfElseThen),
    /// Begin loop
    Begin(imp::Begin),
    /// Do-loop.
    Loop(imp::Loop),
    /// Create a new constant.
    NewConstant(String),
    /// Push the constant to the stack.
    Constant(Int),
    /// Create a new constant holding the memory address of the variable.
    NewVariable(String),
    /// Read Forth script from the path.
    Include(String),
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
            Dummy => Err(CompileTimeWord),
        }
    }
}

pub mod imp {
    use super::Expr;
    use crate::{
        errors::Error::{self, LeaveLoop},
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
}
