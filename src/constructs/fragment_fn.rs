use super::*;

pub unsafe trait GPUFragmentFn: GPUFn<Output = Vec4> {
    type Input: GPUFragment;

    fn validate() {}
}
