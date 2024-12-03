use super::{GPUFnDesc, GPUStructDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUItemDesc<'a> {
    Struct(GPUStructDesc<'a>),
    Fn(GPUFnDesc<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUItemID(pub u128);

impl<'a> GPUItemDesc<'a> {
    pub fn id(&self) -> GPUItemID {
        match self {
            Self::Struct(item) => item.id,
            Self::Fn(item) => item.id,
        }
    }
}
