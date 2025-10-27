use crate::ir::{PrimitiveIr, TypeIr};

pub trait ShaderType: Copy + 'static + Send + Sync {
    const IR: TypeIr;
}

pub trait FragmentType: ShaderType {
    const __: ();
}

pub trait PrimitiveType: ShaderType {}

pub trait VectorType<const N: usize, T: PrimitiveType>: ShaderType {}

impl ShaderType for f32 {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::F32);
}

impl ShaderType for i32 {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::I32);
}

impl ShaderType for u32 {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::U32);
}

impl ShaderType for bool {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::Bool);
}

impl PrimitiveType for f32 {}
impl PrimitiveType for i32 {}
impl PrimitiveType for u32 {}
impl PrimitiveType for bool {}
