use crate::{Position, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Source<'s> {
    pub content: &'s str,
    pub file_name: &'s str,
}

impl<'s> Source<'s> {
    pub const fn new(content: &'s str, file_name: &'s str) -> Self {
        Self { content, file_name }
    }

    pub fn span(&self, span: Span) -> SourceSpan<'s> {
        SourceSpan::new(self.file_name, span)
    }

    pub fn position(&self, span: Span) -> Position {
        let is_at_newline =
            span.start < self.content.len() && self.content.as_bytes()[span.start] == b'\n';

        let text = &self.content[..span.start];
        let newline_count = text.matches('\n').count();

        let line_start = match text.rfind('\n') {
            Some(last_newline) => last_newline + 1,
            None => 0,
        };

        if is_at_newline {
            let next_line_start = span.start + 1;

            let next_line_end = if next_line_start < self.content.len() {
                match self.content[next_line_start..].find('\n') {
                    Some(next_newline) => next_line_start + next_newline,
                    None => self.content.len(),
                }
            } else {
                self.content.len()
            };

            return Position::new(newline_count + 2, 1, next_line_start, next_line_end);
        }

        let line = newline_count + 1;
        let column = match text.rfind('\n') {
            Some(last_newline) => text[last_newline + 1..].chars().count() + 1,
            None => text.chars().count() + 1,
        };

        let line_end = if span.start < self.content.len() {
            match self.content[span.start..].find('\n') {
                Some(next_newline) => span.start + next_newline,
                None => self.content.len(),
            }
        } else {
            self.content.len()
        };

        Position::new(line, column, line_start, line_end)
    }
}

pub struct SourceSpan<'s> {
    file_name: &'s str,
    span: Span,
}

impl<'s> SourceSpan<'s> {
    pub fn new(file_name: &'s str, span: Span) -> Self {
        Self { file_name, span }
    }
}

impl<'s> ariadne::Span for SourceSpan<'s> {
    type SourceId = &'s str;

    fn source(&self) -> &Self::SourceId {
        &self.file_name
    }

    fn start(&self) -> usize {
        self.span.start
    }

    fn end(&self) -> usize {
        self.span.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_new() {
        let source = Source::new("content", "file.rs");
        assert_eq!(source.content, "content");
        assert_eq!(source.file_name, "file.rs");
    }

    #[test]
    fn test_source_unknown() {
        let source = Source::new("content", "unknown");
        assert_eq!(source.content, "content");
        assert_eq!(source.file_name, "unknown");
    }

    #[test]
    fn test_position_from_span_single_line() {
        let content = "hello world";
        let source = Source::new(content, "unknown");

        let span = Span::new(0, 1);
        let position = source.position(span);
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 1);
        assert_eq!(position.line_start, 0);
        assert_eq!(position.line_end, 11);

        let span = Span::new(6, 7);
        let position = source.position(span);
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 7);
        assert_eq!(position.line_start, 0);
        assert_eq!(position.line_end, 11);
    }

    #[test]
    fn test_position_from_span_multiple_lines() {
        let content = "first line\nsecond line\nthird line";
        let source = Source::new(content, "unknown");

        let span = Span::new(5, 6);
        let position = source.position(span);
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 6);
        assert_eq!(position.line_start, 0);
        assert_eq!(position.line_end, 10);

        let span = Span::new(11, 12);
        let position = source.position(span);
        assert_eq!(position.line, 2);
        assert_eq!(position.column, 1);
        assert_eq!(position.line_start, 11);
        assert_eq!(position.line_end, 22);

        let span = Span::new(15, 16);
        let position = source.position(span);
        assert_eq!(position.line, 2);
        assert_eq!(position.column, 5);
        assert_eq!(position.line_start, 11);
        assert_eq!(position.line_end, 22);

        let span = Span::new(22, 23);
        let position = source.position(span);
        assert_eq!(position.line, 3);
        assert_eq!(position.column, 1);
        assert_eq!(position.line_start, 23);
        assert_eq!(position.line_end, 33);
    }

    #[test]
    fn test_position_from_span_with_unicode() {
        let content = "Hello!\n🚀 Unicode test\nLine with café\nÑandú é açaí";
        let source = Source::new(content, "unknown");

        let span1 = Span::new(0, 1);
        let pos1 = source.position(span1);
        assert_eq!(pos1, Position::new(1, 1, 0, 6));

        let span2 = Span::new(7, 11);
        let pos2 = source.position(span2);
        assert_eq!(pos2, Position::new(2, 1, 7, 24));

        let span3 = Span::new(30, 32);
        let pos3 = source.position(span3);
        assert_eq!(pos3, Position::new(3, 6, 25, 40));

        let span4 = Span::new(41, 43);
        let pos4 = source.position(span4);
        assert_eq!(pos4, Position::new(4, 1, 41, 58));
    }

    #[test]
    fn test_position_from_span_empty_content() {
        let content = "";
        let source = Source::new(content, "unknown");

        let span = Span::new(0, 0);
        let position = source.position(span);
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 1);
        assert_eq!(position.line_start, 0);
        assert_eq!(position.line_end, 0);
    }
}
