use super::*;

pub unsafe trait FragmentFn: GPUFn {
    type I: Fragment;

    fn invoke(input: Self::I) -> Vec4;

    fn validate() {}
}
