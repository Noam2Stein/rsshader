use super::{GPUFieldDesc, GPUIdentDesc, GPUTypeDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUStmtDesc {
    Let(GPULetDesc),
    Expr(GPUExprDesc),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPULetDesc {
    pub ident: GPUIdentDesc,
    pub value: GPUExprDesc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUExprDesc {
    BoolLiteral(bool),
    IntLiteral(u128),
    FloatLiteral(&'static str),
    Struct(
        &'static GPUTypeDesc,
        &'static [(&'static GPUFieldDesc, GPUExprDesc)],
    ),
    Array(&'static [GPUExprDesc]),
    Local(&'static GPUIdentDesc),
    Field(&'static GPUExprDesc, GPUIdentDesc),
    Index(&'static GPUExprDesc, &'static GPUExprDesc),
    Empty,
}
