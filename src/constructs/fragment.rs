use super::*;
use crate as rsshader;

pub unsafe trait GPUFragment: GPULerp {
    fn pos(&self) -> Vec4;

    fn validate() {}
}

#[gpu(fragment)]
pub struct FallbackFragment {
    #[fragment_pos]
    pos: Vec4,
}
