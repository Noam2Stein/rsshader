use crate::{
    ir::{
        BodyIr, BuiltinFn, EntryPointIr, ExprIr, FnIr, FragInputIr, FragOutputIr, Length, LinkedShaderIr, Literal, PlaceIr, Primitive,
        StmtIr, StructIr, TypeIr, VariableIr, VectorIr, VertexInputIr,
    },
    lang::Formatter,
};

#[macro_export]
macro_rules! wgsl {
    ($($entry_point:path),* $(,)?) => {
        $crate::shader!($($entry_point),* => $crate::lang::wgsl::fmt)
    };
}

pub const fn fmt(f: &mut Formatter, shader: &LinkedShaderIr) {
    macro_rules! fmt_all {
        ($f:path => $values:expr) => {
            let mut i = 0;
            while i < $values.len() {
                $f(f, &$values[i], shader);

                i += 1;
            }
        };
    }

    fmt_all!(fmt_vertex_input => shader.vertex_inputs);
    fmt_all!(fmt_frag_input => shader.frag_inputs);
    fmt_all!(fmt_frag_output => shader.frag_outputs);
    fmt_all!(fmt_ty => shader.types);
    fmt_all!(fmt_entry_point => shader.entry_points);
    fmt_all!(fmt_fn => shader.fns);
}

const fn fmt_vertex_input(f: &mut Formatter, vertex_input: &VertexInputIr, shader: &LinkedShaderIr) {
    f.write_str("struct vertex_input");
    f.write_i128(vertex_input.id(shader) as i128);
    f.write_str(" {\n");

    let mut attr_idx = 0;
    let mut attrs = vertex_input.attrs();
    while let Some(attr_ty) = attrs.next() {
        f.write_str("\t@location(");
        f.write_i128(attr_idx as i128);
        f.write_str(")");
        f.write_str("\tattr");
        f.write_i128(attr_idx as i128);
        f.write_str(": ");
        fmt_ty(f, attr_ty, shader);
        f.write_str(";\n");

        attr_idx += 1;
    }

    f.write_str("}\n\n");
}

const fn fmt_frag_input(f: &mut Formatter, frag_input: &FragInputIr, shader: &LinkedShaderIr) {
    f.write_str("struct frag_input");
    f.write_i128(frag_input.id(shader) as i128);
    f.write_str(" {\n");

    let mut attr_idx = 0;
    let mut attrs = frag_input.attrs();
    while let Some(attr_ty) = attrs.next() {
        if attr_idx == 0 {
            f.write_str("\t@builtin(position)");
        } else {
            f.write_str("\t@location(");
            f.write_i128(attr_idx as i128);
            f.write_str(")");
        }

        f.write_str("\tattr");
        f.write_i128(attr_idx as i128);
        f.write_str(":");
        fmt_ty(f, attr_ty, shader);
        f.write_str(";\n");

        attr_idx += 1;
    }

    f.write_str("}\n\n");
}

const fn fmt_frag_output(f: &mut Formatter, frag_output: &FragOutputIr, shader: &LinkedShaderIr) {
    f.write_str("struct frag_output");
    f.write_i128(frag_output.id(shader) as i128);
    f.write_str(" {\n");

    let mut attr_idx = 0;
    let mut attrs = frag_output.attrs();
    while let Some(attr_ty) = attrs.next() {
        f.write_str("\t@location(");
        f.write_i128(attr_idx as i128);
        f.write_str(")");
        f.write_str("\tattr");
        f.write_i128(attr_idx as i128);
        f.write_str(":");
        fmt_ty(f, attr_ty, shader);
        f.write_str(";\n");

        attr_idx += 1;
    }

    f.write_str("}\n\n");
}

const fn fmt_ty(f: &mut Formatter, ty: &'static TypeIr, shader: &LinkedShaderIr) {
    match ty {
        TypeIr::Primitive(Primitive::F32 | Primitive::I32 | Primitive::U32 | Primitive::Bool) => {}

        TypeIr::Vector(VectorIr {
            primitive: Primitive::F32 | Primitive::I32 | Primitive::U32 | Primitive::Bool,
            length: Length::Two | Length::Three | Length::Four,
        }) => {}

        TypeIr::Struct(StructIr { fields }) => {
            f.write_str("struct type");
            f.write_i128(ty.id(shader) as i128);
            f.write_str(" {\n");

            let mut field_idx = 0;
            while field_idx < fields.len() {
                f.write_str("\tfield");
                f.write_i128(field_idx as i128);
                f.write_str(": ");

                fmt_type_name(f, &fields[field_idx], shader);
                f.write_str(",\n");

                field_idx += 1;
            }

            f.write_str("}\n\n");
        }
    }
}

const fn fmt_entry_point(f: &mut Formatter, entry_point: &'static EntryPointIr, shader: &LinkedShaderIr) {
    match entry_point {
        EntryPointIr::Vertex { input, output, body } => {
            f.write_str("@vertex\n");
            f.write_str("fn entry_point");
            f.write_i128(entry_point.id(shader) as i128);
            f.write_str("(vertex: vertex_input");
            f.write_i128(input.id(shader) as i128);
            f.write_str(") -> frag_input");
            f.write_i128(output.id(shader) as i128);
            f.write_str(" {\n");

            fmt_body(f, body, shader);

            f.write_str("}\n\n");
        }

        EntryPointIr::Frag { input, output, body } => {
            f.write_str("@fragment\n");
            f.write_str("fn entry_point");
            f.write_i128(entry_point.id(shader) as i128);
            f.write_str("(frag: frag_input");
            f.write_i128(input.id(shader) as i128);
            f.write_str(") -> frag_output");
            f.write_i128(output.id(shader) as i128);
            f.write_str(" {\n");

            fmt_body(f, body, shader);

            f.write_str("}\n\n");
        }
    }
}

const fn fmt_fn(f: &mut Formatter, func: &'static FnIr, shader: &LinkedShaderIr) {
    match func {
        FnIr::UserDefined {
            param_types: params,
            ret_type: ret_ty,
            body,
        } => {
            f.write_str("fn fn");
            f.write_i128(func.id(shader) as i128);
            f.write_str("(");

            let mut param_idx = 0;
            while param_idx < params.len() {
                if param_idx > 0 {
                    f.write_str(", ");
                }

                f.write_str("var");
                f.write_i128(params[param_idx].id as i128);
                f.write_str(": ");
                fmt_type_name(f, &params[param_idx].ty, shader);

                param_idx += 1;
            }

            f.write_str(")");

            if let Some(ret_ty) = ret_ty {
                f.write_str(" -> ");
                fmt_type_name(f, ret_ty, shader);
            }

            f.write_str(" {\n");

            fmt_body(f, body, shader);

            f.write_str("}\n\n");
        }

        FnIr::Builtin(BuiltinFn::Neg { ty }) => match ty {
            TypeIr::Primitive(Primitive::F32 | Primitive::I32 | Primitive::U32) => {}
            TypeIr::Vector(VectorIr {
                length: Length::Two | Length::Three | Length::Four,
                primitive: Primitive::F32 | Primitive::I32 | Primitive::U32,
            }) => {}

            TypeIr::Struct(_) => panic!("struct neg not supported"),
            TypeIr::Primitive(Primitive::Bool)
            | TypeIr::Vector(VectorIr {
                length: _,
                primitive: Primitive::Bool,
            }) => panic!("bool neg not supported"),
        },

        FnIr::Builtin(BuiltinFn::Not { ty }) => match ty {
            TypeIr::Primitive(Primitive::Bool | Primitive::I32 | Primitive::U32) => {}
            TypeIr::Vector(VectorIr {
                length: Length::Two | Length::Three | Length::Four,
                primitive: Primitive::Bool | Primitive::I32 | Primitive::U32,
            }) => {}

            TypeIr::Struct(_) => panic!("struct neg not supported"),
            TypeIr::Primitive(Primitive::Bool)
            | TypeIr::Vector(VectorIr {
                length: _,
                primitive: Primitive::Bool,
            }) => panic!("bool neg not supported"),
        },
    }
}

const fn fmt_type_name(f: &mut Formatter, ty: &'static TypeIr, shader: &LinkedShaderIr) {
    match ty {
        TypeIr::Primitive(Primitive::F32) => f.write_str("f32"),
        TypeIr::Primitive(Primitive::I32) => f.write_str("i32"),
        TypeIr::Primitive(Primitive::U32) => f.write_str("u32"),
        TypeIr::Primitive(Primitive::Bool) => f.write_str("bool"),

        TypeIr::Vector(VectorIr { length, primitive }) => {
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

        TypeIr::Struct(_) | TypeIr::VertexAttributes(_) | TypeIr::FragmentAttributes(_) | TypeIr::RenderOutputAttributes(_) => {
            f.write_str("type");
            f.write_i128(shader.type_id(ty) as i128);
        }
    }
}

const fn fmt_body(f: &mut Formatter, body: &'static BodyIr, shader: &LinkedShaderIr) {}

const fn fmt_stmt(
    f: &mut Formatter,
    stmt: &'static StmtIr,
    expr_bank: &'static [ExprIr],
    stmt_bank: &'static [StmtIr],
    tab_lvl: usize,
    shader: &'static LinkedShaderIr,
) {
    let mut i = 0;
    while i < tab_lvl {
        f.write_str("\t");
        i += 1;
    }

    match stmt {
        StmtIr::VariableDecl(VariableIr { id, ty }) => {
            f.write_str("var var");
            f.write_i128(*id as i128);
            f.write_str(": ");
            fmt_type_name(f, ty, shader);
            f.write_str(";\n");
        }

        StmtIr::Assignment(left, right) => {
            fmt_place(f, left, shader);
            f.write_str(" = ");
            fmt_expr(f, right, expr_bank, stmt_bank, shader);
            f.write_str(";\n");
        }

        StmtIr::Return(expr) => {
            f.write_str("return");

            if let Some(expr) = expr {
                f.write_str(" ");
                fmt_expr(f, expr, expr_bank, stmt_bank, shader);
            }

            f.write_str(";\n");
        }
    }
}

const fn fmt_expr(
    f: &mut Formatter,
    expr: &'static ExprIr,
    expr_bank: &'static [ExprIr],
    stmt_bank: &'static [StmtIr],
    shader: &'static LinkedShaderIr,
) {
    match expr {
        ExprIr::Literal(Literal::F32(value)) => {
            f.write_str("bitcast<f32>(0x");
            f.write_u32_hex(value.to_bits());
            f.write_str(">");
        }
        ExprIr::Literal(Literal::I32(value)) => {
            f.write_str("bitcast<i32>(0x");
            f.write_u32_hex(value.cast_unsigned());
            f.write_str(">");
        }
        ExprIr::Literal(Literal::U32(value)) => {
            f.write_str("bitcast<u32>(0x");
            f.write_u32_hex(*value);
            f.write_str(">");
        }
        ExprIr::Literal(Literal::Bool(false)) => f.write_str("false"),
        ExprIr::Literal(Literal::Bool(true)) => f.write_str("true"),

        ExprIr::Variable(VariableIr { id, ty: _ }) => {
            f.write_str("var");
            f.write_i128(*id as i128);
        }

        ExprIr::Call { func: function, args } => match function {
            FnIr::UserDefined { .. } => {
                f.write_str("fn");
                f.write_i128(shader.fn_id(function) as i128);
                f.write_str("(");

                let mut arg_idx = 0;
                while arg_idx < args.len() {
                    if arg_idx > 0 {
                        f.write_str(", ");
                    }

                    fmt_expr(f, &expr_bank[args[arg_idx].0], expr_bank, stmt_bank, shader);

                    arg_idx += 1;
                }

                f.write_str(")");
            }

            FnIr::Builtin(BuiltinFn::Neg(_)) => {
                f.write_str("-(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Not(_)) => {
                f.write_str("!(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Add(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") + (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Sub(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") - (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Mul(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") * (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Div(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") / (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Rem(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") % (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Shl(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") << (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Shr(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") >> (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::BitAnd(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") & (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::BitOr(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") | (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::BitXor(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") ^ (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Eq(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") == (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Ne(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") != (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Lt(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") < (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Gt(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") > (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Le(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") <= (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Ge(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") >= (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::And) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") && (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::Builtin(BuiltinFn::Or) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") || (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }

            FnIr::Builtin(BuiltinFn::StructConstructor { ty }) => {
                f.write_str("type");
                f.write_i128(shader.type_id(ty) as i128);
                f.write_str("(");

                let mut arg_idx = 0;
                while arg_idx < args.len() {
                    if arg_idx > 0 {
                        f.write_str(", ");
                    }

                    fmt_expr(f, &expr_bank[args[arg_idx].0], expr_bank, stmt_bank, shader);

                    arg_idx += 1;
                }

                f.write_str(")");
            }
        },
    }
}

const fn fmt_place(f: &mut Formatter, place: &'static PlaceIr, _shader: &'static LinkedShaderIr) {
    match place {
        PlaceIr::Variable(VariableIr { id, ty: _ }) => {
            f.write_str("var");
            f.write_i128(*id as i128);
        }
    }
}
