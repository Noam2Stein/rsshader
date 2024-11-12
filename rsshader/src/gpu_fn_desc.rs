use super::*;

pub struct GPUFnDesc<'a> {
    pub ident: &'a str,
    pub inputs: &'a [GPUFnInputDesc<'a>],
    pub output: Option<&'a GPUTypeDesc<'a>>,
}
pub struct GPUFnInputDesc<'a> {
    pub ident: &'a str,
    pub ty: &'a GPUTypeDesc<'a>,
}
