use std::{iter::Peekable, str::Chars};

/// The reader that traverses the string returning the characters. It has `pop` and `peek` functionalities.
pub struct Reader<'a>(Peekable<Chars<'a>>);

impl<'a> Reader<'a> {
    /// Peek at the next character, but do not advance the iterator.
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
    /// Create the `Reader` from a string.
    fn from(value: &'a str) -> Self {
        Self(value.chars().peekable())
    }
}
