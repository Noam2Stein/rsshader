use crate::{
    ir::{Primitive, Type},
    reflection::ShaderType,
};

pub trait VertexType: ShaderType {
    const ATTRIBUTES: &'static [&'static Type];
}

impl VertexType for f32 {
    const ATTRIBUTES: &'static [&'static Type] = &[&Type::Primitive(Primitive::F32)];
}

impl VertexType for i32 {
    const ATTRIBUTES: &'static [&'static Type] = &[&Type::Primitive(Primitive::I32)];
}

impl VertexType for u32 {
    const ATTRIBUTES: &'static [&'static Type] = &[&Type::Primitive(Primitive::U32)];
}
