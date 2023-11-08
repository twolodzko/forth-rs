use crate::{
    executables::{self, Executable},
    Error, Forth,
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
pub fn read_until(chars: &mut Reader, delimiter: char) -> String {
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

// fn read_function(chars: &mut Reader) -> Option<(String, executables::Function)> {
//     let name = read_word(chars);
//     if name.is_empty() {
//         return None;
//     }

//     while let Some(expr) = next(chars) {
//         if let Executable::Word(word) = expr {
//             match word.as_str() {
//                 ";" => break,
//                 _ => (), // FIXME
//             }
//         }
//     }

//     Some((name, _))
// }

// TODO
#[allow(dead_code)]
fn next(chars: &mut Reader) -> Option<Executable> {
    // skip leading spaces
    skip_whitespaces(chars);

    let word = read_word(chars);
    match word.as_str() {
        // end of input
        "" => None,
        // skip comments
        "(" => {
            skip_until(chars, ')');
            next(chars)
        }
        "\\" => {
            skip_until(chars, '\n');
            next(chars)
        }
        // strings
        ".\"" => {
            let string = read_until(chars, '"');
            Some(Executable::String(string))
        }
        // bindings:
        ":" => {
            // FIXME
            skip_whitespaces(chars);
            // let words = read_function(chars);
            // let (name, _func) = compile_function(&mut words.iter())?;
            let name = "<FIXME>".to_string();
            let body = Vec::new();
            Some(Executable::NewFunction(
                name,
                executables::Function { body },
            ))
        }
        "variable" => {
            let name = read_word(chars);
            Some(Executable::NewVariable(name))
        }
        "constant" => {
            let name = read_word(chars);
            Some(Executable::NewConstant(name))
        }
        // other words:
        word => Some(Executable::Word(word.to_string())),
    }
}

impl Forth {
    /// Go to next word and evaluate it
    pub(crate) fn eval_next_word(&mut self, chars: &mut Reader) -> Option<Result<(), Error>> {
        Some(next(chars)?.execute(self))
    }
}
