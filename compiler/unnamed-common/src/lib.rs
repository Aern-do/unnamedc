pub mod entity_arena;
pub mod interner;
pub mod source;
pub mod source_cache;
pub mod span;

pub use entity_arena::EntityArena;
pub use interner::{Interner, StrId};
pub use source::{Source, SourceSpan};
pub use source_cache::SourceCache;
pub use span::{Position, Span};

pub type Report<'s> = ariadne::Report<'s, SourceSpan<'s>>;
pub trait IntoReport {
    fn into_report<'s>(self, source: Source<'s>) -> Report<'s>;
}

pub trait Spanned {
    fn span(&self) -> Span;
}
