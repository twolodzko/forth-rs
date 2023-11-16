use crate::{
    errors::Error::{self, MissingArgument, ParsingError},
    expressions::Expr::{self, *},
    numbers::Int,
    reader::Reader,
};

/// The parser that can read the code.
pub struct Parser<'a>(Reader<'a>);

impl<'a> Parser<'a> {
    /// Skip whitespaces until any non-whitespace character. Do not pop the character.
    #[inline]
    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.0.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.0.next();
        }
    }

    /// Read all the characters until the `delimiter` (exclusive).
    #[inline]
    pub fn read_until(&mut self, delimiter: char) -> Result<String, Error> {
        let reader = &mut self.0;
        let mut string = String::new();
        for c in reader {
            if c == delimiter {
                return Ok(string);
            }
            string.push(c);
        }
        Err(ParsingError(format!("missing '{}'", delimiter)))
    }

    /// Read all the characters until a whitespace (exclusive).
    #[inline]
    fn read_word(&mut self) -> String {
        let reader = &mut self.0;
        reader
            .take_while(|c| !c.is_whitespace())
            .flat_map(|c| c.to_lowercase())
            .collect()
    }

    /// Read the function delimited by `: ... ;`.
    #[inline]
    fn read_function(&mut self) -> Result<Expr, Error> {
        // : <name> <body...> ;

        self.skip_whitespaces();

        let name = self.read_word();
        if name.is_empty() {
            return Err(ParsingError("function needs to be named".into()));
        }

        let mut body = Vec::new();
        for expr in self {
            let expr = expr?;
            if let Word(word) = &expr {
                if word == ";" {
                    return Ok(NewFunction(name, body));
                }
            }
            body.push(expr);
        }

        Err(ParsingError("missing ';'".into()))
    }

    /// Read the `if ... [else ...] then` block
    #[inline]
    fn read_iet(&mut self) -> Result<Expr, Error> {
        // if <then...> then
        // if <then...> else <otherwise...> then

        let mut then = Vec::new();
        let mut other = Vec::new();
        let mut acc = &mut then;

        for expr in self {
            let expr = expr?;
            if let Word(word) = &expr {
                match word.as_str() {
                    // the else block starts
                    "else" => {
                        acc = &mut other;
                        continue;
                    }
                    // the definition ends
                    "then" => return Ok(IfElseThen(then, other)),
                    _ => (),
                }
            }
            acc.push(expr)
        }

        Err(ParsingError("missing 'then'".into()))
    }

    /// Read the `begin ... again | until | repeat` block.
    #[inline]
    fn read_begin(&mut self) -> Result<Expr, Error> {
        // begin <body...> again
        // begin <body...> <flag> until
        // begin <body...> <flag> while <body...> repeat

        let mut body = Vec::new();
        for expr in self.by_ref() {
            let expr = expr?;
            if let Word(word) = &expr {
                match word.as_str() {
                    // end of block
                    "repeat" | "again" => return Ok(Begin(body)),
                    // end of block, but take the "until" word
                    "until" => {
                        body.push(expr);
                        return Ok(Begin(body));
                    }
                    _ => (),
                }
            }
            body.push(expr)
        }
        Err(ParsingError(
            "begin blocks needs to end with 'repeat', 'again', or 'until'".into(),
        ))
    }

    /// Read the `do ... loop` block.
    #[inline]
    fn read_loop(&mut self) -> Result<Expr, Error> {
        // do ... loop

        let mut body = Vec::new();
        for expr in self.by_ref() {
            let expr = expr?;
            if let Word(word) = &expr {
                if word == "loop" {
                    return Ok(Loop(body));
                }
            }
            body.push(expr)
        }

        Err(ParsingError("do blocks needs to end with 'loop'".into()))
    }
}

/// Parse the keyword with one argument form.
macro_rules! single_arg {
    ( $self:ident, $type:expr ) => {{
        let word = $self.read_word();
        if word.is_empty() {
            Some(Err(MissingArgument))
        } else {
            Some(Ok($type(word)))
        }
    }};
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Expr, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // skip leading spaces
        self.skip_whitespaces();

        let word = self.read_word();
        match word.as_str() {
            // end of input
            "" => None,
            // skip comments
            "(" => {
                for c in &mut self.0 {
                    if c == ')' {
                        return self.next();
                    }
                }
                Some(Err(ParsingError("missing ')'".into())))
            }
            "\\" => {
                let reader = &mut self.0;
                reader.take_while(|c| c != &'\n').for_each(drop);
                self.next()
            }
            // strings
            "char" => {
                let result = match self.0.next() {
                    None => Err(ParsingError("failed to read character".into())),
                    Some(c) => {
                        // ignore the rest of the word if there is any, this is how Forth behaves
                        for c in &mut self.0 {
                            if c.is_whitespace() {
                                break;
                            }
                        }
                        Ok(Char(c as Int))
                    }
                };
                Some(result)
            }
            ".(" => match self.read_until(')') {
                Ok(string) => {
                    print!("{}", string);
                    self.next()
                }
                Err(msg) => Some(Err(msg)),
            },
            ".\"" => match self.read_until('"') {
                Ok(string) => Some(Ok(Print(string))),
                Err(msg) => Some(Err(msg)),
            },
            // special forms
            ":" => Some(self.read_function()),
            "if" => Some(self.read_iet()),
            "begin" => Some(self.read_begin()),
            "do" => Some(self.read_loop()),
            // words followed by a single argument
            "variable" => {
                single_arg!(self, NewVariable)
            }
            "create" => {
                single_arg!(self, NewCreate)
            }
            "constant" => {
                single_arg!(self, NewConstant)
            }
            "value" => {
                single_arg!(self, NewValue)
            }
            "to" => {
                single_arg!(self, ToValue)
            }
            "include" => {
                single_arg!(self, Include)
            }
            "see" => {
                single_arg!(self, See)
            }
            // regular words
            word => Some(Ok(Word(word.into()))),
        }
    }
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        let reader = Reader::from(value);
        Self(reader)
    }
}
