use super::*;

pub trait GPUType {
    const DESC: GPUTypeDesc<'static>;
}

pub use rsshader_proc_macros::GPUType;

impl<T: GPUType, const N: usize> GPUType for [T; N] {
    const DESC: GPUTypeDesc<'static> = GPUTypeDesc::Array(GPUArrayDesc {
        item_type: &T::DESC,
        length: N,
    });
}

impl GPUType for bool {
    const DESC: GPUTypeDesc<'static> = GPUTypeDesc::Bool;
}
impl GPUType for f32 {
    const DESC: GPUTypeDesc<'static> = GPUTypeDesc::F32;
}
impl GPUType for i32 {
    const DESC: GPUTypeDesc<'static> = GPUTypeDesc::I32;
}
impl GPUType for u32 {
    const DESC: GPUTypeDesc<'static> = GPUTypeDesc::U32;
}
