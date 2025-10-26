use crate::ir::{Primitive, Type};

pub trait ShaderType: Copy + 'static + Send + Sync {
    const IR: Type;
}

pub trait FragmentType: ShaderType {
    const __: ();
}

pub trait PrimitiveType: ShaderType {}

pub trait VectorType<const N: usize, T: PrimitiveType>: ShaderType {}

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

impl PrimitiveType for f32 {}
impl PrimitiveType for i32 {}
impl PrimitiveType for u32 {}
impl PrimitiveType for bool {}
