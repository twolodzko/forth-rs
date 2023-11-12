use crate::expressions::Expr::{
    self, Begin, IfElseThen, Include, Loop, NewConstant, NewFunction, NewVariable, Print, See, Word,
};
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
        // Read the block:
        // : <name> <body...> ;

        self.skip_whitespaces();

        let name = self.read_word();
        if name.is_empty() {
            return None;
        }

        let body = self
            .take_while(|expr| {
                if let Word(word) = expr {
                    word != ";"
                } else {
                    true
                }
            })
            .collect();

        Some(NewFunction(name, body))
    }

    fn read_iet(&mut self) -> Option<Expr> {
        // if <then...> then
        // if <then...> else <otherwise...> then

        let mut acc = Vec::new();
        let mut then = Vec::new();
        let mut other = Vec::new();
        let mut had_else = false;

        for ref expr in self.by_ref() {
            if let Word(word) = expr {
                match word.as_str() {
                    // the else block starts
                    "else" => {
                        then = acc.clone();
                        acc.clear();
                        had_else = true;
                        continue;
                    }
                    // the definition ends
                    "then" => break,
                    _ => (),
                }
            }
            acc.push(expr.clone())
        }

        if had_else {
            other = acc.clone();
        } else {
            then = acc.clone();
        }

        Some(IfElseThen(then, other))
    }

    fn read_begin(&mut self) -> Option<Expr> {
        // begin <body...> again
        // begin <body...> <flag> until
        // begin <body...> <flag> while <body...> repeat

        let mut body = Vec::new();

        for ref expr in self.by_ref() {
            if let Word(word) = expr {
                match word.as_str() {
                    // end of block
                    "repeat" | "again" => break,
                    // end of block, but take the "until" word
                    "until" => {
                        body.push(expr.clone());
                        break;
                    }
                    _ => (),
                }
            }
            body.push(expr.clone())
        }
        Some(Begin(body))
    }

    fn read_loop(&mut self) -> Option<Expr> {
        // do ... loop

        let body = self
            .take_while(|expr| {
                if let Word(word) = expr {
                    word != "loop"
                } else {
                    true
                }
            })
            .collect();
        Some(Loop(body))
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
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
            ".(" => {
                let string = self.read_until(')');
                print!("{}", string);
                self.next()
            }
            ".\"" => {
                let string = self.read_until('"');
                Some(Print(string))
            }
            // special forms
            ":" => self.read_function(),
            "variable" => {
                let name = self.read_word();
                Some(NewVariable(name))
            }
            "constant" => {
                let name = self.read_word();
                Some(NewConstant(name))
            }
            "if" => self.read_iet(),
            "include" => {
                let path = self.read_word();
                Some(Include(path))
            }
            "see" => {
                let word = self.read_word();
                Some(See(word))
            }
            // loops
            "begin" => self.read_begin(),
            "do" => self.read_loop(),
            // other words
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
