use std::fmt::{self, Formatter};

use crate::shader_core::*;

pub unsafe trait GPUType: 'static {
    fn wgsl_ident(f: &mut Formatter) -> fmt::Result;
    fn wgsl_declaration(f: &mut Formatter) -> fmt::Result;

    fn validate() {}
}
pub unsafe trait GPUFn: 'static {
    fn wgsl_ident(f: &mut Formatter) -> fmt::Result;
    fn wgsl_declaration(f: &mut Formatter) -> fmt::Result;

    fn validate() {}
}

pub unsafe trait Vertex: GPUType {
    fn validate() {}
}
pub unsafe trait Fragment: GPUType {
    fn pos(&self) -> Vec4;

    fn validate() {}
}

pub unsafe trait VertexFn: GPUFn {
    type I: Vertex;
    type O: Fragment;
    
    fn invoke(input: Self::I) -> Self::O;

    fn validate() {}
}
pub unsafe trait FragmentFn: GPUFn {
    type I: Fragment;

    fn invoke(input: Self::I) -> Vec4;

    fn validate() {}
}
