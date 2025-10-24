use rsshader_macros::ConstEq;

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Type {
    Primitive(Primitive),
    Vector(Vector),
    Matrix(Matrix),
    Struct(Struct),
    Array(Array),
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
pub struct Matrix {
    pub rows: Length,
    pub columns: Length,
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
    pub id: usize,
    pub kind: FieldKind,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum FieldKind {
    Normal,
    VertexAttribute(usize),
    Position,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct Array {
    pub length: Option<usize>,
    pub element_type: &'static Type,
}
