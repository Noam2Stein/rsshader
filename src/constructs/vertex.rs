use super::*;
use crate as rsshader;

pub unsafe trait Vertex: GPUType {
    fn validate() {}
}

#[gpu(vertex)]
pub struct FallbackVertex {}
