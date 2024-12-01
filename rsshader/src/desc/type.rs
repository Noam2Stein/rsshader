#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUTypeDesc<'a> {
    Struct(GPUStructDesc<'a>),
    Array(GPUArrayDesc<'a>),
    Bool,
    F32,
    F32x2,
    F32x3,
    F32x4,
    I32,
    I32x2,
    I32x3,
    I32x4,
    U32,
    U32x2,
    U32x3,
    U32x4,
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
