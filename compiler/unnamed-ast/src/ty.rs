use cranelift_entity::EntityList;
use unnamed_common::{Span, Spanned, StrId};

use crate::{AstCtx, TypeEntity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Type {
    pub kind: TypeKind,
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

impl Spanned for Type {
    type Ctx = AstCtx;

    fn span(&self, _ctx: &Self::Ctx) -> Span {
        self.span
    }
}
