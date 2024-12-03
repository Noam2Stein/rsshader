use super::GPUItemID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUTypeDesc<'a> {
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
    Array(&'a GPUTypeDesc<'a>, usize),
    Struct(GPUStructDesc<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUStructDesc<'a> {
    pub id: GPUItemID,
    pub name: &'a str,
    pub fields: &'a [GPUFieldDesc<'a>],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFieldDesc<'a> {
    pub id: GPUItemID,
    pub name: &'a str,
    pub ty: &'a GPUTypeDesc<'a>,
}

pub struct GPUUnsupportedType;
