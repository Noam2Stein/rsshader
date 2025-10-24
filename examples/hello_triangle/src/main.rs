fn main() {
    println!("{}", SHADER.replace("\t", "  "));
}

use rsshader::{
    ir::{Field, FieldKind, Length, Primitive, Shader, Struct, Type, Vector},
    lang::{Formatter, fmt_wgsl},
};

const SHADER: &str = {
    const IR: Shader = Shader {
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
    };

    const LEN: usize = {
        let mut f = Formatter::without_output();
        fmt_wgsl(&mut f, &IR);
        f.output_len()
    };

    const BUF: [u8; LEN] = {
        let mut buf = [0; LEN];
        let mut f = Formatter::with_output(&mut buf);
        fmt_wgsl(&mut f, &IR);
        buf
    };

    unsafe { std::str::from_utf8_unchecked(&BUF) }
};
