use crate::ir::FnIr;

pub trait ShaderFn {
    const IR: FnIr;
}
