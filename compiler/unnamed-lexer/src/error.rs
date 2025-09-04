use ariadne::{Label, ReportKind};
use unnamed_common::{IntoReport, Report, Source, Span};

#[derive(Debug, Clone)]
pub struct Error {
    span: Span,
    kind: ErrorKind,
}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Self {
        Self { span, kind }
    }

    pub fn invalid_token(span: Span) -> Self {
        Self::new(span, ErrorKind::InvalidToken)
    }

    pub fn unclosed_string_literal(span: Span) -> Self {
        Self::new(span, ErrorKind::UnclosedStringLiteral)
    }

    pub fn invalid_escape_sequence(span: Span) -> Self {
        Self::new(span, ErrorKind::InvalidEscapeSequence)
    }

    pub fn unexpected_eof() -> Self {
        Self::new(Span::ZERO, ErrorKind::UnexpectedEof)
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidToken,
    UnexpectedEof,
    UnclosedStringLiteral,
    InvalidEscapeSequence,
}

impl IntoReport for Error {
    fn into_report<'s>(self, source: Source<'s>) -> Report<'s> {
        let message = match self.kind {
            ErrorKind::InvalidToken => "invalid token",
            ErrorKind::UnexpectedEof => "unexpected eof",
            ErrorKind::UnclosedStringLiteral => "unclosed string literal",
            ErrorKind::InvalidEscapeSequence => "invalid escape sequence",
        };

        let label_message = match self.kind {
            ErrorKind::UnclosedStringLiteral => Some("this string literal is not closed"),
            _ => None,
        };

        Report::build(ReportKind::Error, source.span(self.span))
            .with_message(message)
            .with_label(
                Label::new(source.span(self.span)).with_message(label_message.unwrap_or(message)),
            )
            .finish()
    }
}
