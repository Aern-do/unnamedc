pub mod entity_arena;
pub mod interner;
pub mod source;
pub mod source_cache;
pub mod span;

use cranelift_entity::{EntityList, EntityRef, packed_option::ReservedValue};
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
    type Ctx;
    fn span(&self, ctx: &Self::Ctx) -> Span;
}

pub fn entity_list_span<E: EntityRef + ReservedValue, T: Spanned>(
    list: EntityList<E>,
    arena: &EntityArena<E, T>,
    ctx: &T::Ctx,
) -> Span {
    list.as_slice(&arena.pool)
        .iter()
        .map(|entity| {
            let entity = &arena.map[*entity];
            entity.span(ctx)
        })
        .reduce(|acc, span| acc + span)
        .unwrap_or_default()
}
