pub mod source;
pub mod source_cache;
pub mod span;

pub use source::{Source, SourceSpan};
pub use source_cache::SourceCache;
pub use span::{Position, Span};

pub type Report<'s> = ariadne::Report<'s, SourceSpan<'s>>;
pub trait IntoReport {
    fn into_report<'s>(self, source: Source<'s>) -> Report<'s>;
}
