mod function;
mod ty;
pub use function::*;
pub use ty::*;

#[derive(Debug, Clone, Copy)]
pub struct LinkedShader {
    pub types: &'static [&'static Type],
    pub functions: &'static [&'static Function],
}
