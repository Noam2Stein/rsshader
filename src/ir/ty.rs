use rsshader_macros::ConstEq;

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum TypeIr {
    Primitive(PrimitiveIr),
    Vector(VectorIr),
    Struct(StructIr),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum PrimitiveIr {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct VectorIr {
    pub length: LengthIr,
    pub primitive: PrimitiveIr,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum LengthIr {
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct StructIr {
    pub fields: &'static [FieldIr],
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct FieldIr {
    pub ty: &'static TypeIr,
    pub rust_offset: usize,
    pub marker: Option<FieldMarkerIr>,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum FieldMarkerIr {
    Position,
}
