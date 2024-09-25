use std::fmt::{self, Formatter};

pub trait GPUType: 'static {
    fn wgsl_ident(f: &mut Formatter) -> fmt::Result;
    fn wgsl_declaration(f: &mut Formatter) -> fmt::Result;
}

pub trait GPUStruct {
    fn validate_impl() {}
}

pub trait Vertex: GPUStruct {
    fn validate_impl() {}
}
pub trait Fragment: GPUStruct {
    fn validate_impl() {}
}