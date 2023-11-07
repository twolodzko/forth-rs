mod buildins;
mod errors;
mod forth;
mod memory;
mod parser;
mod special;

pub use crate::errors::Error;
pub use crate::memory::Memory;

#[cfg(test)]
mod tests;
