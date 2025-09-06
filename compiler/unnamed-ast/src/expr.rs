use cranelift_entity::EntityList;
use unnamed_common::{Span, Spanned, StrId, entity_list_span};

use crate::{AstCtx, Block, ExprEntity, FieldEntity, TypeEntity};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayExpr {
    List {
        values: EntityList<ExprEntity>,
    },
    Repeat {
        value: ExprEntity,
        repeat: ExprEntity,
    },
}

impl Spanned for ArrayExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        match self {
            ArrayExpr::List { values } => entity_list_span(*values, &ctx.exprs, ctx),
            ArrayExpr::Repeat { value, repeat } => {
                let value_span = ctx.exprs[*value].span(ctx);
                let repeat_span = ctx.exprs[*repeat].span(ctx);

                value_span + repeat_span
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructExpr {
    pub name: StrId,
    pub name_span: Span,

    pub fields: EntityList<FieldEntity>,
}

impl Spanned for StructExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let fields_span = entity_list_span(self.fields, &ctx.field_exprs, ctx);

        fields_span + self.name_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructFieldExpr {
    pub name: StrId,
    pub name_span: Span,

    pub value: ExprEntity,
}

impl Spanned for StructFieldExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let value_span = ctx.exprs.map[self.value].span(ctx);

        value_span + self.name_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IfExpr {
    pub cond: ExprEntity,
    pub then_branch: Block,
    pub else_branch: Option<ElseExpr>,
}

impl Spanned for IfExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let cond_span = ctx.exprs[self.cond].span(ctx);
        let then_span = self.then_branch.span(ctx);
        let else_span = self.else_branch.map(|else_branch| else_branch.span(ctx));

        let mut span = cond_span + then_span;
        if let Some(else_span) = else_span {
            span += else_span
        }

        span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElseExpr {
    If(ExprEntity),
    Else(Block),
}

impl Spanned for ElseExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        match self {
            ElseExpr::If(expr) => ctx.exprs[*expr].span(ctx),
            ElseExpr::Else(block) => block.span(ctx),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldExpr {
    pub base: ExprEntity,
    pub member: StrId,
    pub member_span: Span,
}

impl Spanned for FieldExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let base_span = ctx.exprs[self.base].span(ctx);
        base_span + self.member_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexExpr {
    pub base: ExprEntity,
    pub index: ExprEntity,
}

impl Spanned for IndexExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let base_span = ctx.exprs[self.base].span(ctx);
        let index_span = ctx.exprs[self.index].span(ctx);

        base_span + index_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssignExpr {
    pub lhs: StrId,
    pub lhs_span: Span,
    pub rhs: ExprEntity,
}

impl Spanned for AssignExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let rhs_span = ctx.exprs[self.rhs].span(ctx);

        self.lhs_span + rhs_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReturnExpr {
    pub kw_span: Span,
    pub value: Option<ExprEntity>,
}

impl Spanned for ReturnExpr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        let value_span = self
            .value
            .map(|value| ctx.exprs[value].span(ctx))
            .unwrap_or_default();

        self.kw_span + value_span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expr {
    Ident(StrId, Span),
    Str(StrId, Span),
    Int(u64, Span),
    Bool(bool, Span),
    Unit(Span),

    Block(Block),
    Assign(AssignExpr),
    Bin(BinExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    MethodCall(MethodCallExpr),
    If(IfExpr),
    While(WhileExpr),
    Array(ArrayExpr),
    Struct(StructExpr),
    Field(FieldExpr),
    Index(IndexExpr),
    Return(ReturnExpr),
}

impl Spanned for Expr {
    type Ctx = AstCtx;

    fn span(&self, ctx: &Self::Ctx) -> Span {
        match self {
            Expr::Ident(_, span) => *span,
            Expr::Str(_, span) => *span,
            Expr::Int(_, span) => *span,
            Expr::Bool(_, span) => *span,
            Expr::Unit(span) => *span,
            Expr::Block(block) => block.span(ctx),
            Expr::Assign(assign) => assign.span(ctx),
            Expr::Bin(bin_expr) => bin_expr.span(ctx),
            Expr::Unary(unary_expr) => unary_expr.span(ctx),
            Expr::Call(call_expr) => call_expr.span(ctx),
            Expr::MethodCall(method_call_expr) => method_call_expr.span(ctx),
            Expr::If(if_expr) => if_expr.span(ctx),
            Expr::While(while_expr) => while_expr.span(ctx),
            Expr::Array(array_expr) => array_expr.span(ctx),
            Expr::Struct(struct_expr) => struct_expr.span(ctx),
            Expr::Field(field_expr) => field_expr.span(ctx),
            Expr::Index(index_expr) => index_expr.span(ctx),
            Expr::Return(return_expr) => return_expr.span(ctx),
        }
    }
}
