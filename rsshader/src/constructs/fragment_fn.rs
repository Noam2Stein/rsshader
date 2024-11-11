use super::*;

pub unsafe trait GPUFragmentFn: GPUFn<Output = FVec4> {
    type Input: GPUFragment;

    fn validate() {}
}
