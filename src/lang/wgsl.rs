use crate::{
    ir::{
        BuiltInFnIr, ExprIr, FnIr, LengthIr, LinkedShaderIr, LiteralIr, PlaceIr, PrimitiveIr,
        StmtIr, TypeIr, VariableIr, VectorIr, VertexInputIr,
    },
    lang::Formatter,
};

#[macro_export]
macro_rules! wgsl {
    ($($entry_point:path),* $(,)?) => {
        $crate::shader!($($entry_point),* => $crate::lang::wgsl::fmt)
    };
}

#[doc(hidden)]
pub const fn fmt(f: &mut Formatter, shader: &'static LinkedShaderIr) {
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
}

const fn fmt_vertex_input(
    f: &mut Formatter,
    vertex_input: &VertexInputIr,
    shader: &LinkedShaderIr,
) {
}

const fn fmt_ty(
    f: &mut Formatter,
    ty: &'static TypeIr,
    ty_idx: usize,
    shader: &'static LinkedShaderIr,
) {
    match ty {
        TypeIr::Primitive(
            PrimitiveIr::F32 | PrimitiveIr::I32 | PrimitiveIr::U32 | PrimitiveIr::Bool,
        ) => {}

        TypeIr::Vector(VectorIr {
            primitive: PrimitiveIr::F32 | PrimitiveIr::I32 | PrimitiveIr::U32 | PrimitiveIr::Bool,
            length: LengthIr::Two | LengthIr::Three | LengthIr::Four,
        }) => {}

        TypeIr::Struct(ty) => {
            f.write_str("struct type");
            f.write_i128(ty_idx as i128);
            f.write_str(" {\n");

            let mut field_idx = 0;
            while field_idx < ty.fields.len() {
                f.write_str("\tfield");
                f.write_i128(field_idx as i128);
                f.write_str(": ");

                fmt_type_name(f, &ty.fields[field_idx], shader);
                f.write_str(",\n");

                field_idx += 1;
            }

            f.write_str("}\n\n");
        }
    }
}

const fn fmt_type_name(f: &mut Formatter, ty: &'static TypeIr, shader: &'static LinkedShaderIr) {
    match ty {
        TypeIr::Primitive(PrimitiveIr::F32) => f.write_str("f32"),
        TypeIr::Primitive(PrimitiveIr::I32) => f.write_str("i32"),
        TypeIr::Primitive(PrimitiveIr::U32) => f.write_str("u32"),
        TypeIr::Primitive(PrimitiveIr::Bool) => f.write_str("bool"),

        TypeIr::Vector(VectorIr { length, primitive }) => {
            f.write_str("vec");

            match length {
                LengthIr::Two => f.write_str("2"),
                LengthIr::Three => f.write_str("3"),
                LengthIr::Four => f.write_str("4"),
            }

            match primitive {
                PrimitiveIr::F32 => f.write_str("f"),
                PrimitiveIr::I32 => f.write_str("i"),
                PrimitiveIr::U32 => f.write_str("u"),
                PrimitiveIr::Bool => f.write_str("b"),
            }
        }

        TypeIr::Struct(_)
        | TypeIr::VertexAttributes(_)
        | TypeIr::FragmentAttributes(_)
        | TypeIr::RenderOutputAttributes(_) => {
            f.write_str("type");
            f.write_i128(shader.type_id(ty) as i128);
        }
    }
}

const fn fmt_fn(
    f: &mut Formatter,
    function: &'static FnIr,
    fn_idx: usize,
    shader: &'static LinkedShaderIr,
) {
    let FnIr::UserDefined {
        entry_point_kind: entry_point_info,
        params: parameters,
        ret_ty: return_type,
        stmts,
        expr_bank,
        stmt_bank,
    } = function
    else {
        return match function {
            FnIr::BuiltIn(function) => fmt_builtin_fn_decl(f, function),
            FnIr::UserDefined { .. } => panic!(),
        };
    };

    match entry_point_info {
        None => {}
        Some(EntryPointKind::Vertex) => f.write_str("@vertex\n"),
        Some(EntryPointKind::Fragment) => f.write_str("@fragment\n"),
    }

    f.write_str("fn fn");
    f.write_i128(fn_idx as i128);
    f.write_str("(");

    let mut param_idx = 0;
    while param_idx < parameters.len() {
        if param_idx > 0 {
            f.write_str(", ");
        }

        let param = parameters[param_idx];

        f.write_str("var");
        f.write_i128(param.id as i128);
        f.write_str(": ");
        fmt_type_name(f, param.ty, shader);

        param_idx += 1;
    }

    f.write_str(")");

    if let Some(return_type) = return_type {
        f.write_str(" -> ");
        fmt_type_name(f, return_type, shader);
    }

    f.write_str(" {\n");

    let mut stmt_idx = 0;
    while stmt_idx < stmts.len() {
        let stmt = &stmt_bank[stmts[stmt_idx]];

        fmt_stmt(f, &stmt, expr_bank, stmt_bank, 1, shader);

        stmt_idx += 1;
    }

    f.write_str("}\n\n");
}

const fn fmt_builtin_fn_decl(_f: &mut Formatter, function: &'static BuiltInFnIr) {
    match function {
        BuiltInFnIr::Neg(_) => {}
        BuiltInFnIr::Not(_) => {}

        BuiltInFnIr::Add(_, _) => {}
        BuiltInFnIr::Sub(_, _) => {}
        BuiltInFnIr::Mul(_, _) => {}
        BuiltInFnIr::Div(_, _) => {}
        BuiltInFnIr::Rem(_, _) => {}
        BuiltInFnIr::Shl(_, _) => {}
        BuiltInFnIr::Shr(_, _) => {}
        BuiltInFnIr::BitAnd(_, _) => {}
        BuiltInFnIr::BitOr(_, _) => {}
        BuiltInFnIr::BitXor(_, _) => {}

        BuiltInFnIr::Eq(_) => {}
        BuiltInFnIr::Ne(_) => {}
        BuiltInFnIr::Lt(_) => {}
        BuiltInFnIr::Gt(_) => {}
        BuiltInFnIr::Le(_) => {}
        BuiltInFnIr::Ge(_) => {}

        BuiltInFnIr::And => {}
        BuiltInFnIr::Or => {}

        BuiltInFnIr::StructConstructor { .. } => {}
    }
}

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
        ExprIr::Literal(LiteralIr::F32(value)) => {
            f.write_str("bitcast<f32>(0x");
            f.write_u32_hex(value.to_bits());
            f.write_str(">");
        }
        ExprIr::Literal(LiteralIr::I32(value)) => {
            f.write_str("bitcast<i32>(0x");
            f.write_u32_hex(value.cast_unsigned());
            f.write_str(">");
        }
        ExprIr::Literal(LiteralIr::U32(value)) => {
            f.write_str("bitcast<u32>(0x");
            f.write_u32_hex(*value);
            f.write_str(">");
        }
        ExprIr::Literal(LiteralIr::Bool(false)) => f.write_str("false"),
        ExprIr::Literal(LiteralIr::Bool(true)) => f.write_str("true"),

        ExprIr::Variable(VariableIr { id, ty: _ }) => {
            f.write_str("var");
            f.write_i128(*id as i128);
        }

        ExprIr::FunctionCall {
            func: function,
            args,
        } => match function {
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

            FnIr::BuiltIn(BuiltInFnIr::Neg(_)) => {
                f.write_str("-(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Not(_)) => {
                f.write_str("!(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Add(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") + (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Sub(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") - (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Mul(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") * (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Div(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") / (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Rem(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") % (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Shl(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") << (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Shr(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") >> (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::BitAnd(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") & (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::BitOr(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") | (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::BitXor(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") ^ (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Eq(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") == (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Ne(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") != (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Lt(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") < (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Gt(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") > (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Le(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") <= (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Ge(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") >= (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::And) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") && (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            FnIr::BuiltIn(BuiltInFnIr::Or) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") || (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }

            FnIr::BuiltIn(BuiltInFnIr::StructConstructor { ty }) => {
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
