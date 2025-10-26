use rsshader_macros::ConstEq;

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Type {
    Primitive(Primitive),
    Vector(Vector),
    Struct(Struct),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Primitive {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct Vector {
    pub length: Length,
    pub primitive: Primitive,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Length {
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct Struct {
    pub fields: &'static [Field],
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct Field {
    pub ty: &'static Type,
    pub rust_offset: usize,
    pub marker: Option<FieldMarker>,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum FieldMarker {
    Position,
}
