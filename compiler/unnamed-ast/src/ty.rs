use cranelift_entity::EntityList;
use unnamed_common::{Span, StrId};
use unnamed_derive::Spanned;

use crate::TypeEntity;

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Type {
    pub kind: TypeKind,
    #[span]
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    Named {
        name: StrId,
        args: EntityList<TypeEntity>,
    },
    Unit,
}
