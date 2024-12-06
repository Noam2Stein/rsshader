use super::GPUStructDesc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUPrimitive {
    Bool,
    F32,
    I32,
    U32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUTypeDesc {
    Primitive(GPUPrimitive),
    PrimitiveX2(GPUPrimitive),
    PrimitiveX3(GPUPrimitive),
    PrimitiveX4(GPUPrimitive),
    Array(&'static GPUTypeDesc, usize),
    Tuple(&'static [GPUTypeDesc]),
    Struct(GPUStructDesc),
}

pub struct GPUUnsupportedType;
