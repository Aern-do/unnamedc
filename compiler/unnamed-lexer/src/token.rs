use unnamed_common::Span;

#[derive(Debug, Clone)]
pub struct Token<'s> {
    pub span: Span,
    pub slice: &'s str,
    pub kind: TokenKind,
    pub data: Option<TokenData>,
}

impl<'s> Token<'s> {
    pub fn new(span: Span, slice: &'s str, kind: TokenKind) -> Self {
        Self {
            span,
            slice,
            kind,
            data: None,
        }
    }

    pub fn with_data(mut self, data: TokenData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn number(&self) -> u64 {
        match &self.data {
            Some(TokenData::Number(value)) => *value,
            _ => panic!("invalid token data"),
        }
    }

    pub fn string(&self) -> &str {
        match &self.data {
            Some(TokenData::String(value)) => value,
            _ => panic!("invalid token data"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenData {
    Number(u64),
    String(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Base {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}

impl Base {
    pub fn is_valid_digit(self, char: char) -> bool {
        char.is_digit(self.radix())
    }

    pub fn radix(self) -> u32 {
        match self {
            Base::Hexadecimal => 16,
            Base::Decimal => 10,
            Base::Octal => 8,
            Base::Binary => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Int,
    Str,
    Ident,

    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Asgmt,
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

    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Semicolon,
    Colon,
    DoubleColon,

    FuncKw,
    TraitKw,
    ImplKw,
    StructKw,
    WhileKw,
    LetKw,
    ForKw,
    IfKw,
    ElseKw,
    ReturnKw,
}
