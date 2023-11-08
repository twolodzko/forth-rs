mod buildins;
mod compiled;
mod errors;
mod forth;
mod reader;

pub use crate::errors::Error;
pub use crate::forth::Forth;

#[cfg(test)]
mod tests;
