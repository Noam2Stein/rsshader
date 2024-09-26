use std::fmt::{self, Formatter};

pub unsafe trait GPUType: 'static {
    fn wgsl_ident(f: &mut Formatter) -> fmt::Result;
    fn wgsl_declaration(f: &mut Formatter) -> fmt::Result;

    fn validate() {}
}

pub unsafe trait Vertex: GPUType {
    fn validate() {}
}
pub unsafe trait Fragment: GPUType {
    fn validate() {}
}