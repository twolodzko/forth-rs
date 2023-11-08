use crate::{compiled::Compiled, compiler::compile_function};
use std::{iter::Peekable, str::Chars};

type Reader<'a> = Peekable<Chars<'a>>;

#[derive(Clone)]
pub enum Parsed {
    Word(String),
    ToPrint(String),
    // a function, variable, or a constant
    Binding((String, Compiled)),
}

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

pub fn read(chars: &mut Reader) -> Option<Parsed> {
    use self::Parsed::{Binding, ToPrint, Word};

    // skip leading spaces
    skip_whitespaces(chars);

    let word = read_word(chars);
    match word.as_str() {
        "" | "\n" => None,
        // comments
        "(" => {
            skip_until(chars, ')');
            read(chars)
        }
        "\\" => {
            skip_until(chars, '\n');
            read(chars)
        }
        // strings
        ".\"" => Some(ToPrint(read_until(chars, '"'))),
        ".(" => {
            print!("{} ", read_until(chars, ')'));
            read(chars)
        }
        // bindings:
        ":" => {
            skip_whitespaces(chars);
            let words = read_function(chars);
            let (name, func) = compile_function(&mut words.iter())?;
            Some(Binding((name, func)))
        }
        "variable" => {
            let name = read_word(chars);
            match name.as_str() {
                // FIXME: this should be error
                "" | "\n" => None,
                _ => Some(Binding((name, Compiled::Variable(0)))),
            }
        }
        "constant" => unimplemented!(),
        // other words:
        _ => Some(Word(word)),
    }
}
