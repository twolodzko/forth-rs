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
