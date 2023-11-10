mod buildins;
mod errors;
mod expressions;
mod forth;
mod parser;
mod tail;

pub use crate::errors::Error;
pub use crate::forth::Forth;

#[cfg(test)]
mod tests;

pub use crate::tail::Tail;
