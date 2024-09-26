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

pub unsafe trait VertexFn<I: Vertex, O: Fragment>: GPUFn {
    fn invoke(input: I) -> O;

    fn validate() {}
}
pub unsafe trait FragmentFn<I: Fragment>: GPUFn {
    fn invoke(input: I) -> Vec4;

    fn validate() {}
}
