pub trait GPUType: 'static {
    const INFO: GPUTypeInfo;
}
impl GPUType for () {
    const INFO: GPUTypeInfo = GPUTypeInfo::Nothing;
}

pub enum GPUTypeInfo {
    Struct(GPUStructInfo),
    Bool,
    Int32,
    Float32,
    BoolX2,
    Int32X2,
    Float32X2,
    BoolX3,
    Int32X3,
    Float32X3,
    BoolX4,
    Int32X4,
    Float32X4,
    Nothing,
}
pub struct GPUStructInfo {
    pub ident: &'static str,
    pub fields: &'static [GPUFieldInfo],
}
pub struct GPUFieldInfo {
    pub ident: &'static str,
    pub ty: &'static GPUTypeInfo,
}
