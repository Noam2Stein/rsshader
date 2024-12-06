use super::{GPUIdentDesc, GPUTypeDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUStructDesc {
    pub ident: GPUIdentDesc,
    pub fields: &'static [GPUFieldDesc],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUFieldDesc {
    pub ident: GPUIdentDesc,
    pub ty: &'static GPUTypeDesc,
}

impl GPUStructDesc {
    pub const fn field(&self, ident: &str) -> &'static GPUFieldDesc {
        let mut index = 0;
        while !const_eq_str(self.fields[index].ident.1, ident) {
            index += 1;
        }

        &self.fields[index]
    }
}

const fn const_eq_str(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    if a.len() != b.len() {
        false
    } else {
        let mut index = 0;
        while index < a.len() {
            if a[index] != b[index] {
                return false;
            }

            index += 1;
        }

        true
    }
}
