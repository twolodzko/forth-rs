use crate::expressions::{self, Expr};
use std::{iter::Peekable, str::Chars};

pub type Reader<'a> = Peekable<Chars<'a>>;

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
pub fn next(chars: &mut Reader) -> Option<Expr> {
    use Expr::{NewConstant, NewFunction, NewVariable, Print, Word};

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
            Some(Print(string))
        }
        // bindings:
        ":" => {
            // FIXME
            skip_whitespaces(chars);
            // let words = read_function(chars);
            // let (name, _func) = compile_function(&mut words.iter())?;
            let name = "<FIXME>".to_string();
            let body = Vec::new();
            Some(NewFunction(name, expressions::Function { body }))
        }
        "variable" => {
            let name = read_word(chars);
            Some(NewVariable(name))
        }
        "constant" => {
            let name = read_word(chars);
            Some(NewConstant(name))
        }
        // other words:
        word => Some(Word(word.to_string())),
    }
}
