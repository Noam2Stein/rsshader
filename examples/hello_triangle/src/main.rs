fn main() {
    println!("{}", SHADER.replace("\t", "  "));
}

use rsshader::{
    ir::{
        EntryKind, Expr, Field, FieldKind, Function, Length, Literal, Primitive, Shader, Stmt,
        Struct, Type, Variable, Vector,
    },
    wgsl,
};

const SHADER: &str = wgsl!(Shader {
    entries: &[&Function {
        entry_kind: Some(EntryKind::Vertex),
        parameters: &[Variable {
            id: 0,
            ty: &Type::Struct(Struct {
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
                    Field {
                        id: 2,
                        kind: FieldKind::Normal,
                        ty: &Type::Struct(Struct {
                            fields: &[Field {
                                id: 0,
                                kind: FieldKind::VertexAttribute(0),
                                ty: &Type::Primitive(Primitive::Bool),
                            }]
                        })
                    }
                ],
            }),
        }],
        return_type: None,
        stmts: &[0],
        expr_bank: &[],
        stmt_bank: &[Stmt::Return(Some(Expr::Literal(Literal::F32(1.0))))],
    }],
});
