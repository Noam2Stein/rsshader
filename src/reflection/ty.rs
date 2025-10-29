use crate::ir::{
    FragInputIr, FragOutputIr, LengthIr, PrimitiveIr, TypeIr, VectorIr, VertexInputIr,
};

pub trait Ty: Copy + 'static + Send + Sync {
    const IR: TypeIr;
}

pub trait PrimitiveTy: Ty {}

pub trait VectorTy<const N: usize, T: PrimitiveTy>: Ty {}

pub trait VertexInputTy: Ty {
    const IR: VertexInputIr;
}

pub trait FragInputTy: Ty {
    const IR: FragInputIr;
}

pub trait FragOutputTy: Ty {
    const IR: FragOutputIr;
}

impl PrimitiveTy for f32 {}
impl Ty for f32 {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::F32);
}

impl PrimitiveTy for i32 {}
impl Ty for i32 {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::I32);
}

impl PrimitiveTy for u32 {}
impl Ty for u32 {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::U32);
}

impl PrimitiveTy for bool {}
impl Ty for bool {
    const IR: TypeIr = TypeIr::Primitive(PrimitiveIr::Bool);
}

impl<T: VectorTy<4, f32>> FragOutputTy for T {
    const IR: FragOutputIr = FragOutputIr(&TypeIr::Vector(VectorIr {
        length: LengthIr::Four,
        primitive: PrimitiveIr::F32,
    }));
}
