use crate::ir::Function;

pub trait ShaderFn {
    const IR: Function;
}
