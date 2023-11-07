use crate::forth::{Forth, ForthResult};

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

    /// Execute the function
    pub fn execute(&mut self, forth: &mut Forth) -> ForthResult {
        for word in self.body.iter() {
            forth.execute(word)?;
        }
        Ok(())
    }
}
