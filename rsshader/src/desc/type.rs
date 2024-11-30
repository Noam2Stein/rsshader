#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUTypeDesc<'a> {
    Struct(GPUStructDesc<'a>),
    Array(GPUArrayDesc<'a>),
    Bool,
    F32,
    F32X2,
    F32X3,
    F32X4,
    I32,
    I32X2,
    I32X3,
    I32X4,
    U32,
    U32X2,
    U32X3,
    U32X4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUStructDesc<'a> {
    pub ident: &'a str,
    pub fields: &'a [GPUFieldDesc<'a>],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFieldDesc<'a> {
    pub ident: &'a str,
    pub ty: &'a GPUTypeDesc<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUArrayDesc<'a> {
    pub item_type: &'a GPUTypeDesc<'a>,
    pub length: usize,
}

pub struct GPUUnsupportedType;
