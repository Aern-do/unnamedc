use cranelift_entity::EntityList;
use unnamed_common::{Span, Spanned, StrId, entity_list_span};

use crate::{AstCtx, Block, ExprEntity, TypeEntity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Neq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,

    BitOr,
    BitAnd,
    BitXor,
    BitShr,
    BitShl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinExpr {
    pub lhs: ExprEntity,
    pub op: BinOp,
    pub op_span: Span,
    pub rhs: ExprEntity,
}

impl Spanned for BinExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let lhs_span = ctx.exprs.map[self.lhs].span(ctx);
        let rhs_span = ctx.exprs.map[self.rhs].span(ctx);

        lhs_span + self.op_span + rhs_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub op_span: Span,

    pub value: ExprEntity,
}

impl Spanned for UnaryExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let value_span = ctx.exprs.map[self.value].span(ctx);

        value_span + self.op_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallExpr {
    pub receiver: ExprEntity,
    pub ty_args: EntityList<TypeEntity>,
    pub args: EntityList<ExprEntity>,
}

impl Spanned for CallExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let receiver_span = ctx.exprs.map[self.receiver].span(ctx);
        let ty_args_span = entity_list_span(self.ty_args, &ctx.types, ctx);
        let args_span = entity_list_span(self.args, &ctx.exprs, ctx);

        receiver_span + ty_args_span + args_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhileExpr {
    pub cond: ExprEntity,
    pub body: Block,
}

impl Spanned for WhileExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let cond_span = ctx.exprs.map[self.cond].span(ctx);
        let body_span = self.body.span(ctx);

        cond_span + body_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodCallExpr {
    pub receiver: ExprEntity,
    pub ty_args: EntityList<TypeEntity>,
    pub args: EntityList<ExprEntity>,
}

impl Spanned for MethodCallExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let receiver_span = ctx.exprs.map[self.receiver].span(ctx);
        let ty_args_span = entity_list_span(self.ty_args, &ctx.types, ctx);
        let args_span = entity_list_span(self.args, &ctx.exprs, ctx);

        receiver_span + ty_args_span + args_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expr {
    Ident(StrId, Span),
    Str(StrId, Span),
    Int(u64, Span),

    Bin(BinExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    MethodCall(MethodCallExpr),
}

impl Spanned for Expr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        match self {
            Expr::Ident(_, span) => *span,
            Expr::Str(_, span) => *span,
            Expr::Int(_, span) => *span,
            Expr::Bin(bin_expr) => bin_expr.span(ctx),
            Expr::Unary(unary_expr) => unary_expr.span(ctx),
            Expr::Call(call_expr) => call_expr.span(ctx),
            Expr::MethodCall(method_call_expr) => method_call_expr.span(ctx),
        }
    }
}
