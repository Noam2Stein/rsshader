use crate::desc::GPUTypeDesc;

pub trait GPUType {
    const TYPE_DESC: GPUTypeDesc;

    fn val_type_desc(&self) -> GPUTypeDesc {
        Self::TYPE_DESC
    }
}

impl<T: GPUType, const N: usize> GPUType for [T; N] {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::Array(&T::TYPE_DESC, N);
}

impl GPUType for bool {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::Bool;
}
impl GPUType for f32 {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::F32;
}
impl GPUType for i32 {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::I32;
}
impl GPUType for u32 {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::U32;
}
