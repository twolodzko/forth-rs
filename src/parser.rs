use crate::{
    memory::ForthResult,
    special::{Function, IfThenElse},
    Memory,
};

pub type Int = i32;

#[allow(dead_code)] // FIXME
#[derive(Clone)]
pub enum Parsed {
    Value(Int),
    Word(String),
    String(String),
    Function(Function),
    IfThenElse(IfThenElse),
    // Loop(Loop),
    // PlusLoop(PlusLoop),
    // Begin(Begin),
    Variable(String),
    Constant(String, String),
}

pub struct Forth<'a> {
    memory: Memory,
    reader: &'a mut dyn Iterator<Item = &'a str>,
}

impl<'a> Forth<'a> {
    pub fn new(capacity: usize, reader: &'a mut impl Iterator<Item = &'a str>) -> Forth<'a> {
        Self {
            memory: Memory::new(capacity),
            reader: reader,
        }
    }

    pub fn run(&mut self) -> ForthResult {
        use Parsed::*;
        while let Some(word) = self.next() {
            match word {
                Value(val) => self.memory.push(val),
                Word(ref word) => self.memory.execute(word)?,
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

impl<'a> Iterator for Forth<'a> {
    type Item = Parsed;

    fn next(&mut self) -> Option<Self::Item> {
        let word = self.reader.next()?;
        match word.to_lowercase().as_ref() {
            "(" => unimplemented!(),
            ".\"" => unimplemented!(),
            ":" => unimplemented!(),
            "if" => unimplemented!(),
            "do" => unimplemented!(),
            "begin" => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
