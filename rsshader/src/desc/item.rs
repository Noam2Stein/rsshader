use super::{GPUFnDesc, GPUIdentDesc, GPUStructDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUItemDesc {
    Struct(GPUStructDesc),
    Fn(GPUFnDesc),
}

impl GPUItemDesc {
    pub fn ident(&self) -> GPUIdentDesc {
        match self {
            Self::Struct(item) => item.ident,
            Self::Fn(item) => item.ident,
        }
    }
}
