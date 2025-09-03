use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign},
};

use crate::Source;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const DUMMY: Span = Span::new(0, 0);

    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn chars_len(&self, source: Source) -> usize {
        source.content[self.start..self.end].chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn shrink(self, bytes: usize) -> Self {
        Self::new(self.start + bytes, self.end - bytes)
    }
}

impl Add<Span> for Span {
    type Output = Span;

    fn add(self, rhs: Span) -> Self::Output {
        Span::new(self.start.min(rhs.start), self.end.max(rhs.end))
    }
}

impl AddAssign<Span> for Span {
    fn add_assign(&mut self, rhs: Span) {
        *self = *self + rhs
    }
}

impl Add<usize> for Span {
    type Output = Span;

    fn add(self, rhs: usize) -> Self::Output {
        Span::new(self.start, self.end + rhs)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub line_start: usize,
    pub line_end: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, line_start: usize, line_end: usize) -> Self {
        Self {
            line,
            column,
            line_start,
            line_end,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn test_span_len() {
        let span1 = Span::new(0, 4);
        let span2 = Span::new(4, 6);
        let span3 = Span::new(0, 0);

        assert_eq!(span1.len(), 4);
        assert_eq!(span2.len(), 2);
        assert_eq!(span3.len(), 0);
    }
}
