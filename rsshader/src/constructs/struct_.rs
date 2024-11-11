use typeid::ConstTypeId;

use super::*;

pub unsafe trait GPUStruct: Sized + 'static {
    fn validate() {}

    const FIELDS: &'static [GPUField];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUField {
    pub ident: &'static str,
    pub ty: ConstTypeId,
}
impl GPUField {
    pub const fn new<T: GPUType>(ident: &'static str) -> Self {
        Self {
            ident,
            ty: ConstTypeId::of::<T>(),
        }
    }
}
