use cranelift_entity::EntityList;
use unnamed_common::{Span, Spanned, StrId};
use unnamed_derive::Spanned;

use crate::{Block, ExprEntity, FieldEntity, TypeEntity};

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

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinExpr {
    pub lhs: ExprEntity,
    pub op: BinOp,
    pub rhs: ExprEntity,
    #[span]
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Add,
    Sub,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub value: ExprEntity,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallExpr {
    pub receiver: ExprEntity,
    pub ty_args: EntityList<TypeEntity>,
    pub args: EntityList<ExprEntity>,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhileExpr {
    pub cond: ExprEntity,
    pub body: Block,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodCallExpr {
    pub receiver: ExprEntity,
    pub ty_args: EntityList<TypeEntity>,
    pub args: EntityList<ExprEntity>,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub kind: ArrayExprKind,
    #[span]
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayExprKind {
    List {
        values: EntityList<ExprEntity>,
    },
    Repeat {
        value: ExprEntity,
        repeat: ExprEntity,
    },
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructExpr {
    pub name: StrId,
    pub fields: EntityList<FieldEntity>,
    #[span]
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructFieldExpr {
    pub name: StrId,
    pub value: ExprEntity,
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IfExpr {
    pub cond: ExprEntity,
    pub then_branch: Block,
    pub else_branch: Option<ElseExpr>,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElseExpr {
    pub kind: ElseExprKind,
    #[span]
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElseExprKind {
    If(ExprEntity),
    Else(Block),
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldExpr {
    pub base: ExprEntity,
    pub member: StrId,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexExpr {
    pub base: ExprEntity,
    pub index: ExprEntity,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssignExpr {
    pub lhs: StrId,
    pub rhs: ExprEntity,
    #[span]
    pub span: Span,
}

#[derive(Spanned, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReturnExpr {
    pub value: Option<ExprEntity>,
    #[span]
    pub span: Span,
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
    fn span(&self) -> Span {
        match self {
            Expr::Ident(_, span) => *span,
            Expr::Str(_, span) => *span,
            Expr::Int(_, span) => *span,
            Expr::Bool(_, span) => *span,
            Expr::Unit(span) => *span,
            Expr::Block(block) => block.span(),
            Expr::Assign(assign) => assign.span(),
            Expr::Bin(bin_expr) => bin_expr.span(),
            Expr::Unary(unary_expr) => unary_expr.span(),
            Expr::Call(call_expr) => call_expr.span(),
            Expr::MethodCall(method_call_expr) => method_call_expr.span(),
            Expr::If(if_expr) => if_expr.span(),
            Expr::While(while_expr) => while_expr.span(),
            Expr::Array(array_expr) => array_expr.span(),
            Expr::Struct(struct_expr) => struct_expr.span(),
            Expr::Field(field_expr) => field_expr.span(),
            Expr::Index(index_expr) => index_expr.span(),
            Expr::Return(return_expr) => return_expr.span(),
        }
    }
}
