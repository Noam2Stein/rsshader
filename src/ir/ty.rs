#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Primitive(Primitive),
    Vector(Vector),
    Matrix(Matrix),
    Struct(Struct),
    Array(Array),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub length: Length,
    pub primitive: Primitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix {
    pub rows: Length,
    pub columns: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Length {
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Struct {
    pub fields: &'static [Field],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    pub ty: &'static Type,
    pub kind: FieldKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Normal,
    VertexAttribute(usize),
    Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array {
    pub length: Option<usize>,
    pub element_type: &'static Type,
}
