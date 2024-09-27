use super::*;

pub unsafe trait FragmentFn: GPUFn<Output = Vec4> {
    type Input: Fragment;

    fn validate() {}
}
