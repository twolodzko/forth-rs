use crate::expressions::{self, Expr, IfThenElse};
use std::{iter::Peekable, str::Chars};

pub struct Reader<'a>(Peekable<Chars<'a>>);

impl<'a> Reader<'a> {
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

impl<'a> From<&'a str> for Reader<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.chars().peekable())
    }
}

pub struct Parser<'a>(Reader<'a>);

impl<'a> Parser<'a> {
    #[inline]
    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.0.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.0.next();
        }
    }

    #[inline]
    fn skip_until(&mut self, delimiter: char) {
        let reader = &mut self.0;
        reader.take_while(|c| c != &delimiter).for_each(drop)
    }

    #[inline]
    pub fn read_until(&mut self, delimiter: char) -> String {
        let reader = &mut self.0;
        reader.take_while(|c| c != &delimiter).collect()
    }

    #[inline]
    fn read_word(&mut self) -> String {
        let reader = &mut self.0;
        reader
            .take_while(|c| !c.is_whitespace())
            .flat_map(|c| c.to_lowercase())
            .collect()
    }

    fn read_function(&mut self) -> Option<Expr> {
        self.skip_whitespaces();

        let name = self.read_word();
        if name.is_empty() {
            return None;
        }

        let body = self
            .take_while(|expr| {
                if let Expr::Word(word) = expr {
                    word != ";"
                } else {
                    true
                }
            })
            .collect();

        let func = expressions::Function { body };

        Some(Expr::NewFunction(name, func))
    }

    fn read_ite(&mut self) -> Option<Expr> {
        let mut acc = Vec::new();
        let mut then = Vec::new();

        for ref expr in self.by_ref() {
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
}

impl<'a> Iterator for Parser<'a> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        use Expr::{NewConstant, NewVariable, Print, Word};

        // skip leading spaces
        self.skip_whitespaces();

        let word = self.read_word();
        match word.as_str() {
            // end of input
            "" => None,
            // skip comments
            "(" => {
                self.skip_until(')');
                self.next()
            }
            "\\" => {
                self.skip_until('\n');
                self.next()
            }
            // strings
            ".\"" => {
                let string = self.read_until('"');
                Some(Print(string))
            }
            ".(" => {
                let string = self.read_until(')');
                print!("{}", string);
                self.next()
            }
            // bindings:
            ":" => self.read_function(),
            "variable" => {
                let name = self.read_word();
                Some(NewVariable(name))
            }
            "constant" => {
                let name = self.read_word();
                Some(NewConstant(name))
            }
            "if" => self.read_ite(),
            // other words:
            word => Some(Word(word.to_string())),
        }
    }
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        let reader = Reader::from(value);
        Self(reader)
    }
}
