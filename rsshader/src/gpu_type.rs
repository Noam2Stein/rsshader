use crate::desc::GPUTypeDesc;

pub unsafe trait GPUType {
    const TYPE_DESC: GPUTypeDesc;

    fn val_type_desc(&self) -> GPUTypeDesc {
        Self::TYPE_DESC
    }
}

unsafe impl<T: GPUType, const N: usize> GPUType for [T; N] {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::Array(&T::TYPE_DESC, N);
}

unsafe impl GPUType for bool {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::Bool;
}
unsafe impl GPUType for f32 {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::F32;
}
unsafe impl GPUType for i32 {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::I32;
}
unsafe impl GPUType for u32 {
    const TYPE_DESC: GPUTypeDesc = GPUTypeDesc::U32;
}
