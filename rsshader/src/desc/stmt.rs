use super::GPUTypeDesc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUStmtDesc<'a> {
    Let(GPULetDesc<'a>),
    Expr(GPUExprDesc<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPULetDesc<'a> {
    pub ident: &'a str,
    pub value: GPUExprDesc<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUExprDesc<'a> {
    BoolLiteral(bool),
    IntLiteral(u128),
    FloatLiteral(&'a str),
    Struct(&'a GPUTypeDesc<'a>, &'a [(&'a str, GPUExprDesc<'a>)]),
    Array(&'a [GPUExprDesc<'a>]),
    Local(&'a str),
    Field(&'a GPUExprDesc<'a>, &'a str),
    Index(&'a GPUExprDesc<'a>, &'a GPUExprDesc<'a>),
    Empty,
}
