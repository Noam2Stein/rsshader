use super::{GPUIdentDesc, GPUStmtDesc, GPUTypeDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFnDesc {
    pub ident: GPUIdentDesc,
    pub inputs: &'static [GPUFnInputDesc],
    pub output: Option<&'static GPUTypeDesc>,
    pub stmts: &'static [GPUStmtDesc],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFnInputDesc {
    pub ident: GPUIdentDesc,
    pub ty: &'static GPUTypeDesc,
}
