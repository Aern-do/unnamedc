use std::{iter::Peekable, str::Chars};

use unnamed_common::{Source, Span};

use crate::Error;

#[derive(Debug, Clone)]
pub struct Cursor<'s> {
    source: Source<'s>,
    chars: Peekable<Chars<'s>>,

    current: usize,
    previous: usize,
}

impl<'s> Cursor<'s> {
    pub fn new(source: Source<'s>) -> Self {
        let chars = source.content.chars();

        Self {
            source,
            chars: chars.peekable(),
            current: 0,
            previous: 0,
        }
    }

    pub fn next_char(&mut self) -> Result<char, Error> {
        let char = self.chars.next().ok_or(Error::unexpected_eof())?;
        self.current += char.len_utf8();

        Ok(char)
    }

    pub fn peek(&mut self) -> Result<char, Error> {
        self.chars.peek().ok_or(Error::unexpected_eof()).copied()
    }

    pub fn is_eof(&self) -> bool {
        self.current == self.source.content.len()
    }

    pub fn span(&self) -> Span {
        Span::new(self.previous, self.current)
    }

    pub fn current_span(&mut self) -> Result<Span, Error> {
        if self.current == 0 {
            return Err(Error::unexpected_eof());
        }

        let current_char = self.peek()?;
        Ok(Span::new(
            self.current - current_char.len_utf8(),
            self.current,
        ))
    }

    pub fn slice(&self) -> &'s str {
        &self.source.content[self.previous..self.current]
    }

    pub fn slice_at(&self, span: Span) -> &'s str {
        &self.source.content[span.start..span.end]
    }

    pub fn consume(&mut self) -> (&'s str, Span) {
        let slice = self.slice();
        let span = self.span();

        self.previous = self.current;

        (slice, span)
    }

    pub fn lookahead(&self, n: usize) -> Option<char> {
        let mut chars = self.source.content[self.current..].chars().peekable();
        for _ in 0..n {
            chars.next()?;
        }

        chars.peek().copied()
    }

    pub fn skip(&mut self, n: usize) -> Result<(), Error> {
        for _ in 0..n {
            self.next_char()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_next_char() {
        let source = Source::new("abc", "test_file");
        let mut cursor = Cursor::new(source);

        assert_eq!(cursor.next_char().unwrap(), 'a');
        assert_eq!(cursor.next_char().unwrap(), 'b');
        assert_eq!(cursor.next_char().unwrap(), 'c');
        assert!(cursor.next_char().is_err());
    }

    #[test]
    fn test_cursor_span() {
        let source = Source::new("abc", "test_file");
        let mut cursor = Cursor::new(source);

        cursor.next_char().unwrap();
        assert_eq!(cursor.span(), Span::new(0, 1));

        cursor.next_char().unwrap();
        assert_eq!(cursor.span(), Span::new(0, 2));

        cursor.consume();
        cursor.next_char().unwrap();
        assert_eq!(cursor.span(), Span::new(2, 3));
    }

    #[test]
    fn test_cursor_slice() {
        let source = Source::new("hello world", "test_file");
        let mut cursor = Cursor::new(source);

        for _ in 0..5 {
            cursor.next_char().unwrap();
        }

        assert_eq!(cursor.slice(), "hello");
        cursor.consume();

        for _ in 0..6 {
            cursor.next_char().unwrap();
        }

        assert_eq!(cursor.slice(), " world");
    }

    #[test]
    fn test_cursor_consume() {
        let source = Source::new("hello world", "test_file");
        let mut cursor = Cursor::new(source);

        for _ in 0..5 {
            cursor.next_char().unwrap();
        }

        let (slice, span) = cursor.consume();
        assert_eq!(slice, "hello");
        assert_eq!(span, Span::new(0, 5));

        cursor.next_char().unwrap();

        let (slice, span) = cursor.consume();
        assert_eq!(slice, " ");
        assert_eq!(span, Span::new(5, 6));
    }

    #[test]
    fn test_utf8_characters() {
        let source = Source::new("héllö wörld", "test_file");
        let mut cursor = Cursor::new(source);

        assert_eq!(cursor.next_char().unwrap(), 'h');
        assert_eq!(cursor.next_char().unwrap(), 'é');
        assert_eq!(cursor.current, 3);

        cursor.next_char().unwrap();
        cursor.next_char().unwrap();
        cursor.next_char().unwrap();

        let (slice, ..) = cursor.consume();
        assert_eq!(slice, "héllö");
    }

    #[test]
    fn test_empty_source() {
        let source = Source::new("", "test_file");
        let mut cursor = Cursor::new(source);

        assert!(cursor.next_char().is_err());
        assert_eq!(cursor.slice(), "");
        assert_eq!(cursor.span(), Span::new(0, 0));

        let (slice, span) = cursor.consume();
        assert_eq!(slice, "");
        assert_eq!(span, Span::new(0, 0));
    }

    #[test]
    fn test_consecutive_consumes() {
        let source = Source::new("one two three", "test_file");
        let mut cursor = Cursor::new(source);

        for _ in 0..3 {
            cursor.next_char().unwrap();
        }

        let (slice, ..) = cursor.consume();
        assert_eq!(slice, "one");

        cursor.next_char().unwrap();
        let (slice, ..) = cursor.consume();
        assert_eq!(slice, " ");

        for _ in 0..3 {
            cursor.next_char().unwrap();
        }

        let (slice, ..) = cursor.consume();
        assert_eq!(slice, "two");
    }

    #[test]
    fn test_consume_spans_are_correct() {
        let source = Source::new("abc def", "test_file");
        let mut cursor = Cursor::new(source);

        cursor.next_char().unwrap();
        cursor.next_char().unwrap();

        let (.., span) = cursor.consume();
        assert_eq!(span, Span::new(0, 2));

        cursor.next_char().unwrap();
        cursor.next_char().unwrap();

        let (.., span) = cursor.consume();
        assert_eq!(span, Span::new(2, 4));
    }
}
