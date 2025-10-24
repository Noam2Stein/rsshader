use crate::{
    ir::{Array, FieldKind, Length, Matrix, Primitive, Shader, Type, Vector},
    lang::Formatter,
};

#[doc(hidden)]
pub const fn fmt_wgsl(f: &mut Formatter, shader: &'static Shader) {
    let mut ty_idx = 0;
    while ty_idx < shader.types.len() {
        let ty = shader.types[ty_idx];

        fmt_type_decl(f, &ty, ty_idx, shader);

        ty_idx += 1;
    }
}

const fn fmt_type_decl(
    f: &mut Formatter,
    ty: &'static Type,
    ty_idx: usize,
    shader: &'static Shader,
) {
    match ty {
        Type::Primitive(Primitive::F32 | Primitive::I32 | Primitive::U32 | Primitive::Bool) => {}

        Type::Vector(Vector {
            primitive: Primitive::F32 | Primitive::I32 | Primitive::U32 | Primitive::Bool,
            length: Length::Two | Length::Three | Length::Four,
        }) => {}

        Type::Matrix(Matrix {
            rows: Length::Two | Length::Three | Length::Four,
            columns: Length::Two | Length::Three | Length::Four,
        }) => {}

        Type::Array(_) => {}

        Type::Struct(ty) => {
            f.write_str("struct type");
            f.write_i128(ty_idx as i128);
            f.write_str(" {\n");

            let mut field_idx = 0;
            while field_idx < ty.fields.len() {
                let field = &ty.fields[field_idx];

                match field.kind {
                    FieldKind::Normal => {}
                    FieldKind::VertexAttribute(attr_idx) => {
                        f.write_str("\t@location(");
                        f.write_i128(attr_idx as i128);
                        f.write_str(")\n");
                    }
                    FieldKind::Position => {
                        f.write_str("\t@builtin(position)\n");
                    }
                }

                f.write_str("\tfield");
                f.write_i128(field.id as i128);
                f.write_str(": ");

                fmt_type_name(f, field.ty, shader);
                f.write_str(",\n");

                field_idx += 1;
            }

            f.write_str("}\n\n");
        }
    }
}

const fn fmt_type_name(f: &mut Formatter, ty: &'static Type, shader: &'static Shader) {
    match ty {
        Type::Primitive(Primitive::F32) => f.write_str("f32"),
        Type::Primitive(Primitive::I32) => f.write_str("i32"),
        Type::Primitive(Primitive::U32) => f.write_str("u32"),
        Type::Primitive(Primitive::Bool) => f.write_str("bool"),

        Type::Vector(Vector { length, primitive }) => {
            f.write_str("vec");

            match length {
                Length::Two => f.write_str("2"),
                Length::Three => f.write_str("3"),
                Length::Four => f.write_str("4"),
            }

            match primitive {
                Primitive::F32 => f.write_str("f"),
                Primitive::I32 => f.write_str("i"),
                Primitive::U32 => f.write_str("u"),
                Primitive::Bool => f.write_str("b"),
            }
        }

        Type::Matrix(Matrix { rows, columns }) => {
            f.write_str("mat");

            match rows {
                Length::Two => f.write_str("2"),
                Length::Three => f.write_str("3"),
                Length::Four => f.write_str("4"),
            }

            f.write_str("x");

            match columns {
                Length::Two => f.write_str("2"),
                Length::Three => f.write_str("3"),
                Length::Four => f.write_str("4"),
            }

            f.write_str("f");
        }

        Type::Array(Array {
            length,
            element_type,
        }) => {
            f.write_str("array<");
            fmt_type_name(f, element_type, shader);

            if let Some(length) = length {
                f.write_str(", ");
                f.write_i128(*length as i128);
            }

            f.write_str(">");
        }

        Type::Struct(_) => {
            let ty_idx = {
                let mut ty_idx = 0;
                while !shader.types[ty_idx].eq(ty) {
                    ty_idx += 1;
                }

                ty_idx
            };

            f.write_str("type");
            f.write_i128(ty_idx as i128);
        }
    }
}
