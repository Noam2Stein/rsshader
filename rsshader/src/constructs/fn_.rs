use typeid::ConstTypeId;

use super::*;

pub unsafe trait GPUFn: 'static {
    fn validate() {}

    const INPUTS: &'static [GPUArgument];
    type Output: GPUType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUArgument {
    ident: &'static str,
    ty: ConstTypeId,
}
impl GPUArgument {
    pub const fn new<T: GPUType>(ident: &'static str) -> Self {
        Self {
            ident,
            ty: ConstTypeId::of::<T>(),
        }
    }
}
