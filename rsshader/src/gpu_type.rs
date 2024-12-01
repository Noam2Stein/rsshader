use super::*;

pub trait GPUType {
    const TYPE_DESC: GPUTypeDesc<'static>;

    fn val_type_desc(&self) -> GPUTypeDesc<'static> {
        Self::TYPE_DESC
    }
}

impl<T: GPUType, const N: usize> GPUType for [T; N] {
    const TYPE_DESC: GPUTypeDesc<'static> = GPUTypeDesc::Array(GPUArrayDesc {
        item_type: &T::TYPE_DESC,
        length: N,
    });
}

impl GPUType for bool {
    const TYPE_DESC: GPUTypeDesc<'static> = GPUTypeDesc::Bool;
}
impl GPUType for f32 {
    const TYPE_DESC: GPUTypeDesc<'static> = GPUTypeDesc::F32;
}
impl GPUType for i32 {
    const TYPE_DESC: GPUTypeDesc<'static> = GPUTypeDesc::I32;
}
impl GPUType for u32 {
    const TYPE_DESC: GPUTypeDesc<'static> = GPUTypeDesc::U32;
}
