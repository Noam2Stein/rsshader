use super::{GPUExprDesc, GPUIdentDesc};

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
