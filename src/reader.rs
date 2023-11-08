use crate::compiled::{Compiled, Function};
use std::str::Chars;

#[derive(Clone)]
pub enum Parsed {
    Word(String),
    // a function, variable, or a constant
    Binding((String, Compiled)),
}

fn read_word(chars: &mut Chars<'_>) -> Option<String> {
    let mut word = String::new();
    for c in chars {
        if c.is_whitespace() {
            break;
        }
        word.push(c);
    }
    if word.is_empty() {
        return None;
    }
    Some(word)
}

#[allow(unused_variables)] // FIXME
fn read_function(chars: &mut Chars<'_>) -> Option<(String, Function)> {
    // let mut name = read_word(chars)?;
    // let mut body = Vec::new();
    // loop {
    //     match read_word(chars)?.as_ref() {
    //         ";" => return Some((name, Function::new(&body))),
    //         word => body.push(word),
    //     }
    // }
    unimplemented!()
}

#[allow(dead_code)] // FIXME
fn compile(chars: &mut Chars<'_>) -> Option<Compiled> {
    match read_word(chars).expect("failed to read").as_ref() {
        "if" => unimplemented!(),
        "begin" => unimplemented!(),
        "do" => unimplemented!(),
        word => Some(Compiled::Word(word.to_string())),
    }
}

#[inline]
fn skip_whitespaces(chars: &mut Chars<'_>) {
    for c in chars {
        if c.is_whitespace() {
            break;
        }
    }
}

pub fn read(chars: &mut Chars<'_>) -> Option<Parsed> {
    use self::Parsed::{Binding, Word};

    // skip leading spaces
    skip_whitespaces(chars);

    match read_word(chars).expect("nothing was read").as_ref() {
        ":" => {
            let (name, func) = read_function(chars)?;
            Some(Binding((name, Compiled::Function(func))))
        }
        ".\"" => unimplemented!(),
        ".(" => unimplemented!(),
        // bindings:
        "variable" => unimplemented!(),
        "constant" => unimplemented!(),
        // those should fail:
        "if" => unimplemented!(),
        "begin" => unimplemented!(),
        "do" => unimplemented!(),
        // other words:
        word => Some(Word(word.to_string())),
    }
}
