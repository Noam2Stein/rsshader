use crate::desc::GPUFnDesc;

pub unsafe trait GPUFn {
    const FN_DESC: GPUFnDesc;
}
