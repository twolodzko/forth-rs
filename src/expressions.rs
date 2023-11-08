use crate::objects;

#[allow(dead_code)]
pub enum Expression {
    Word(String),
    Function(String, objects::Function),
    Constant(String),
    Variable(String),
}
