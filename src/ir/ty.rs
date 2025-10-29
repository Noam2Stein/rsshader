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
