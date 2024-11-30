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
    Local(&'a str),
    Static(&'a GPUExprDesc<'a>),
    Empty,
}
