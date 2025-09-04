use cranelift_entity::EntityList;
use unnamed_common::{Span, Spanned, StrId, entity_list_span};

use crate::{AstCtx, TypeEntity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Named {
        name: StrId,
        name_span: Span,
        args: EntityList<TypeEntity>,
    },
    Unit {
        span: Span,
    },
}

impl Spanned for Type {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        match self {
            Type::Named {
                name_span, args, ..
            } => {
                let args_span = entity_list_span(*args, &ctx.types, ctx);
                args_span + *name_span
            }
            Type::Unit { span } => *span,
        }
    }
}
