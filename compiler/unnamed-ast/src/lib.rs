pub mod expr;
pub mod ty;

use cranelift_entity::{EntityList, entity_impl};
use unnamed_common::{EntityArena, Span, Spanned, entity_list_span};

pub use expr::{BinExpr, BinOp, Expr, StructFieldExpr};
pub use ty::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprEntity(u32);
entity_impl!(ExprEntity);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeEntity(u32);
entity_impl!(TypeEntity);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldEntity(u32);
entity_impl!(FieldEntity);

#[derive(Debug, Default, Clone)]
pub struct AstCtx {
    pub exprs: EntityArena<ExprEntity, Expr>,
    pub field_exprs: EntityArena<FieldEntity, StructFieldExpr>,
    pub types: EntityArena<TypeEntity, Type>,
}

impl AstCtx {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub exprs: EntityList<ExprEntity>,
}

impl Spanned for Block {
    type Ctx = AstCtx;
    fn span(&self, ctx: &Self::Ctx) -> Span {
        entity_list_span(self.exprs, &ctx.exprs, ctx)
    }
}
