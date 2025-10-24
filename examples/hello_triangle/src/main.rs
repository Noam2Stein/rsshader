fn main() {
    println!("{}", SHADER.replace("\t", "  "));
}

use rsshader::{
    ir::{Field, FieldKind, Length, Primitive, Shader, Struct, Type, Vector},
    wgsl,
};

const SHADER: &str = wgsl!(Shader {
    types: &[&Type::Struct(Struct {
        fields: &[
            Field {
                id: 0,
                kind: FieldKind::VertexAttribute(0),
                ty: &Type::Primitive(Primitive::F32),
            },
            Field {
                id: 1,
                kind: FieldKind::Position,
                ty: &Type::Vector(Vector {
                    length: Length::Four,
                    primitive: Primitive::F32,
                }),
            },
        ],
    })],
    functions: &[],
});
