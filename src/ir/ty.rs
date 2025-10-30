use rsshader_macros::ConstEq;

use crate::ir::LinkedShaderIr;

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum TypeIr {
    Primitive(PrimitiveIr),
    Vector(VectorIr),
    Struct(StructIr),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum PrimitiveIr {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct VectorIr {
    pub length: LengthIr,
    pub primitive: PrimitiveIr,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum LengthIr {
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct StructIr {
    pub fields: &'static [&'static TypeIr],
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum NumIr {
    F32,
    I32,
    U32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum IntIr {
    I32,
    U32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum SignedNumIr {
    F32,
    I32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BitwisePrimitiveIr {
    I32,
    U32,
    Bool,
}

impl TypeIr {
    pub const fn id(&self, shader: &LinkedShaderIr) -> usize {
        let mut i = 0;
        loop {
            if shader.types[i].eq(self) {
                break i;
            }

            i += 1;
        }
    }
}

impl PrimitiveIr {
    pub const fn as_type(self) -> &'static TypeIr {
        match self {
            Self::F32 => &TypeIr::Primitive(PrimitiveIr::F32),
            Self::I32 => &TypeIr::Primitive(PrimitiveIr::I32),
            Self::U32 => &TypeIr::Primitive(PrimitiveIr::U32),
            Self::Bool => &TypeIr::Primitive(PrimitiveIr::Bool),
        }
    }
}

impl NumIr {
    pub const fn as_primitive(self) -> &'static PrimitiveIr {
        match self {
            Self::F32 => &PrimitiveIr::F32,
            Self::I32 => &PrimitiveIr::I32,
            Self::U32 => &PrimitiveIr::U32,
        }
    }
}

impl IntIr {
    pub const fn as_primitive(self) -> &'static PrimitiveIr {
        match self {
            Self::I32 => &PrimitiveIr::I32,
            Self::U32 => &PrimitiveIr::U32,
        }
    }
}

impl SignedNumIr {
    pub const fn as_primitive(self) -> &'static PrimitiveIr {
        match self {
            Self::F32 => &PrimitiveIr::F32,
            Self::I32 => &PrimitiveIr::I32,
        }
    }
}

impl BitwisePrimitiveIr {
    pub const fn as_primitive(self) -> &'static PrimitiveIr {
        match self {
            Self::Bool => &PrimitiveIr::Bool,
            Self::I32 => &PrimitiveIr::I32,
            Self::U32 => &PrimitiveIr::U32,
        }
    }
}
