use rsshader_macros::ConstEq;

use crate::ir::LinkedShaderIr;

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum TypeIr {
    Primitive(Primitive),
    Vector { n: Length, t: Primitive },
    Struct { fields: &'static [TypeIr] },
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

////////////////////////////////////////////////////////////////////////////////
// Primitives
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Primitive {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Numeric {
    F32,
    I32,
    U32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Int {
    I32,
    U32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Sint {
    I32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Uint {
    U32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Float {
    F32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum SignedNumeric {
    F32,
    I32,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BitwisePrimitive {
    I32,
    U32,
    Bool,
}

impl Numeric {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::F32 => Primitive::F32,
            Self::I32 => Primitive::I32,
            Self::U32 => Primitive::U32,
        }
    }
}

impl Int {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::I32 => Primitive::I32,
            Self::U32 => Primitive::U32,
        }
    }
}

impl Sint {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::I32 => Primitive::I32,
        }
    }
}

impl Uint {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::U32 => Primitive::U32,
        }
    }
}

impl Float {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::F32 => Primitive::F32,
        }
    }
}

impl SignedNumeric {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::F32 => Primitive::F32,
            Self::I32 => Primitive::I32,
        }
    }
}

impl BitwisePrimitive {
    pub const fn as_primitive(self) -> Primitive {
        match self {
            Self::Bool => Primitive::Bool,
            Self::I32 => Primitive::I32,
            Self::U32 => Primitive::U32,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Helper Types
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Length {
    Two,
    Three,
    Four,
}
