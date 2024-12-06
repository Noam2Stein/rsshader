use super::GPUStructDesc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUTypeDesc {
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
    Array(&'static GPUTypeDesc, usize),
    Struct(GPUStructDesc),
}

pub struct GPUUnsupportedType;
