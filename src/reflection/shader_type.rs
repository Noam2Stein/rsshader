use crate::ir::{Primitive, Type};

pub trait ShaderType: Copy + 'static + Send + Sync {
    const IR: Type;
}

impl ShaderType for f32 {
    const IR: Type = Type::Primitive(Primitive::F32);
}

impl ShaderType for i32 {
    const IR: Type = Type::Primitive(Primitive::I32);
}

impl ShaderType for u32 {
    const IR: Type = Type::Primitive(Primitive::U32);
}

impl ShaderType for bool {
    const IR: Type = Type::Primitive(Primitive::Bool);
}
