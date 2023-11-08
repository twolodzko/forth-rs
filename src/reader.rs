use crate::{
    compiled::Compiled,
    compiler::compile_function,
    Error::{self, ParsingError},
    Forth,
};
use std::{iter::Peekable, str::Chars};

type Reader<'a> = Peekable<Chars<'a>>;

#[inline]
fn skip_whitespaces(chars: &mut Reader) {
    while let Some(c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }
}

#[inline]
fn skip_until(chars: &mut Reader, delimiter: char) {
    for c in chars {
        if c == delimiter {
            break;
        }
    }
}

#[inline]
fn read_until(chars: &mut Reader, delimiter: char) -> String {
    let mut string = String::new();
    for c in chars {
        if c == delimiter {
            break;
        }
        string.push(c);
    }
    string
}

fn read_word(chars: &mut Reader) -> String {
    let mut word = String::new();
    for c in chars {
        if c.is_whitespace() {
            break;
        }
        word.push(c);
    }
    word
}

fn read_function(chars: &mut Reader) -> Vec<String> {
    let mut body = Vec::new();
    loop {
        let word = read_word(chars);
        match word.as_str() {
            ";" => break,
            _ => body.push(word),
        }
    }
    body
}

impl Forth {
    /// Go to next word and evaluate it
    pub(crate) fn eval_next_word(&mut self, chars: &mut Reader) -> Option<Result<(), Error>> {
        // skip leading spaces
        skip_whitespaces(chars);

        let word = read_word(chars);
        let result = match word.as_str() {
            // end of input
            "" => return None,
            // comments
            "(" => {
                skip_until(chars, ')');
                return self.eval_next_word(chars);
            }
            "\\" => {
                skip_until(chars, '\n');
                return self.eval_next_word(chars);
            }
            // strings
            ".\"" | ".(" => {
                // FIXME
                print!("{}", read_until(chars, '"'));
                Ok(())
            }
            // bindings:
            ":" => {
                skip_whitespaces(chars);
                let words = read_function(chars);
                let (name, func) = compile_function(&mut words.iter())?;
                self.define_word(&name, func)
            }
            "variable" => {
                let name = read_word(chars);
                if name.is_empty() {
                    Err(ParsingError)
                } else {
                    let value = Compiled::Variable(0);
                    self.define_word(&name, value)
                }
            }
            "constant" => {
                let name = read_word(chars);
                if name.is_empty() {
                    Err(ParsingError)
                } else {
                    self.pop().and_then(|value| {
                        let value = Compiled::Constant(value);
                        self.define_word(&name, value)
                    })
                }
            }
            // other words:
            word => self.eval_word(word),
        };
        Some(result)
    }
}
