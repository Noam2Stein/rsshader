use super::{GPUItemID, GPUStmtDesc, GPUTypeDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFnDesc<'a> {
    pub id: GPUItemID,
    pub name: &'a str,
    pub inputs: &'a [GPUFnInputDesc<'a>],
    pub output: Option<&'a GPUTypeDesc<'a>>,
    pub stmts: &'a [GPUStmtDesc<'a>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFnInputDesc<'a> {
    pub ident: &'a str,
    pub ty: &'a GPUTypeDesc<'a>,
}
