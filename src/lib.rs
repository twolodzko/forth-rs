mod buildins;
mod errors;
mod forth;

pub use crate::errors::Error;
pub use crate::forth::Forth;

#[cfg(test)]
mod tests;
