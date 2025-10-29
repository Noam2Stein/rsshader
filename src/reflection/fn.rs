use crate::ir::{EntryPointIr, FnIr};

pub trait Fn {
    const IR: FnIr;
}

pub trait EntryPoint: Fn {
    const IR: EntryPointIr;
}
