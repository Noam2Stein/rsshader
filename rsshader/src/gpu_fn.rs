use crate::GPUItemInfo;

pub unsafe trait GPUFn {
    const GPU_FN_INFO: GPUFnInfo;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFnInfo {
    item_info: GPUItemInfo,
    wgsl_call_reference: &'static str,
}
