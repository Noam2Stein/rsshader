use super::*;
use crate as rsshader;

pub unsafe trait GPUVertex: GPUType {
    fn validate() {}
}

#[gpu(vertex)]
pub struct FallbackVertex {}
