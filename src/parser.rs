use crate::expressions::{self, Expr, IfThenElse};
use std::{iter::Peekable, str::Chars};

pub struct Reader<'a>(Peekable<Chars<'a>>);

impl<'a> Reader<'a> {
    #[inline]
    pub fn new(string: &'a str) -> Self {
        Self(string.chars().peekable())
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&char> {
        self.0.peek()
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
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

fn read_function(chars: &mut Reader) -> Option<Expr> {
    let name = read_word(chars);
    if name.is_empty() {
        return None;
    }

    let mut body = Vec::new();

    while let Some(ref expr) = next(chars) {
        if let Expr::Word(word) = expr {
            if word == ";" {
                break;
            }
        }
        body.push(expr.clone());
    }

    let func = expressions::Function { body };

    Some(Expr::NewFunction(name, func))
}

fn read_ite(chars: &mut Reader) -> Option<Expr> {
    let mut acc = Vec::new();
    let mut then = Vec::new();
    while let Some(ref expr) = next(chars) {
        if let Expr::Word(word) = expr {
            match word.as_str() {
                "then" => {
                    then = acc.clone();
                    acc.clear();
                }
                "else" => break,
                _ => (),
            }
        }
        acc.push(expr.clone());
    }
    let other = acc.clone();
    Some(Expr::IfThenElse(IfThenElse { then, other }))
}

pub fn next(chars: &mut Reader) -> Option<Expr> {
    use Expr::{NewConstant, NewVariable, Print, Word};

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
            skip_whitespaces(chars);
            read_function(chars)
        }
        "variable" => {
            let name = read_word(chars);
            Some(NewVariable(name))
        }
        "constant" => {
            let name = read_word(chars);
            Some(NewConstant(name))
        }
        "if" => read_ite(chars),
        // other words:
        word => Some(Word(word.to_string())),
    }
}
