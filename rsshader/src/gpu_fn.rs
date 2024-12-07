use crate::GPUItemInfo;

pub unsafe trait GPUFn {
    const GPU_FN_INFO: GPUFnInfo;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFnInfo {
    pub item_info: GPUItemInfo,
    pub wgsl_call_reference: &'static str,
}
