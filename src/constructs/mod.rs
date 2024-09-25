use std::fmt::{self, Formatter};

pub trait GPUType: 'static {
    fn wgsl_ident(f: &mut Formatter) -> fmt::Result;
    fn wgsl_declaration(f: &mut Formatter) -> fmt::Result;

    fn validate() {}
}

pub trait Vertex: GPUType {
    fn validate() {}
}
pub trait Fragment: GPUType {
    fn validate() {}
}