mod buildins;
mod compiled;
mod errors;
mod expressions;
mod forth;
mod parser;

pub use crate::errors::Error;
pub use crate::forth::Forth;

#[cfg(test)]
mod tests;
