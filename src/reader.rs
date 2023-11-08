use crate::compiled::{imp, Compiled};
use std::str::Chars;

#[derive(Clone)]
pub enum Parsed {
    Word(String),
    ToPrint(String),
    // a function, variable, or a constant
    Binding((String, Compiled)),
}

fn read_word(chars: &mut Chars<'_>) -> String {
    let mut word = String::new();
    for c in chars {
        if c.is_whitespace() {
            break;
        }
        word.push(c);
    }
    word
}

#[inline]
fn read_until(chars: &mut Chars<'_>, delimiter: char) -> String {
    let mut string = String::new();
    for c in chars {
        if c == delimiter {
            break;
        }
        string.push(c);
    }
    string
}

#[allow(unused_variables)] // FIXME
fn read_function(chars: &mut Chars<'_>) -> Option<(String, Compiled)> {
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
fn compile(chars: &mut Chars<'_>) -> Compiled {
    let word = read_word(chars);
    match word.as_ref() {
        "if" => unimplemented!(),
        "begin" => unimplemented!(),
        "do" => unimplemented!(),
        _ => Compiled::Word(word),
    }
}

#[allow(dead_code)] // FIXME
fn compile_function(chars: &mut Chars<'_>) -> Compiled {
    let mut body = Vec::new();
    loop {
        let compiled = compile(chars);
        if let Compiled::Word(word) = &compiled {
            if matches!(word.as_ref(), ";") {
                break;
            }
        }
        body.push(compiled);
    }
    Compiled::Function(imp::Function { body })
}

#[inline]
fn skip_whitespaces(chars: &mut Chars<'_>) {
    for c in chars {
        if c.is_whitespace() {
            break;
        }
    }
}

#[inline]
fn skip_comment(chars: &mut Chars<'_>) {
    for c in chars {
        if c == ')' {
            break;
        }
    }
}

#[inline]
fn skip_line(chars: &mut Chars<'_>) {
    for c in chars {
        if c == '\n' {
            break;
        }
    }
}

pub fn read(chars: &mut Chars<'_>) -> Option<Parsed> {
    use self::Parsed::{Binding, ToPrint, Word};

    // skip leading spaces
    skip_whitespaces(chars);

    let word = read_word(chars);
    match word.as_ref() {
        "" => None,
        // comments
        "(" => {
            skip_comment(chars);
            read(chars)
        }
        "\\" => {
            skip_line(chars);
            read(chars)
        }
        // strings
        ".\"" => Some(ToPrint(read_until(chars, '"'))),
        ".(" => {
            print!("{}", read_until(chars, ')'));
            read(chars)
        }
        // bindings:
        ":" => {
            let (name, func) = read_function(chars)?;
            Some(Binding((name, func)))
        }
        "variable" => unimplemented!(),
        "constant" => unimplemented!(),
        // other words:
        _ => Some(Word(word)),
    }
}
