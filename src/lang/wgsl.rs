use crate::{
    ir::{
        BuiltInFunction, EntryPointInfo, Expr, FragmentFunctionInfo, Function, Length,
        LinkedShader, Literal, Place, Primitive, Stmt, Type, Variable, Vector, VertexFunctionInfo,
    },
    lang::Formatter,
};

#[macro_export]
macro_rules! wgsl {
    ($($entry_point:path),* $(,)?) => {
        $crate::shader!($($entry_point),* => $crate::lang::fmt_wgsl)
    };
}

#[doc(hidden)]
pub const fn fmt_wgsl(f: &mut Formatter, shader: &'static LinkedShader) {
    let mut ty_idx = 0;
    while ty_idx < shader.types.len() {
        fmt_type_decl(f, &shader.types[ty_idx], ty_idx, shader);

        ty_idx += 1;
    }

    let mut fn_idx = 0;
    while fn_idx < shader.functions.len() {
        fmt_fn_decl(f, &shader.functions[fn_idx], fn_idx, shader);

        fn_idx += 1;
    }
}

const fn fmt_type_decl(
    f: &mut Formatter,
    ty: &'static Type,
    ty_idx: usize,
    shader: &'static LinkedShader,
) {
    match ty {
        Type::Primitive(Primitive::F32 | Primitive::I32 | Primitive::U32 | Primitive::Bool) => {}

        Type::Vector(Vector {
            primitive: Primitive::F32 | Primitive::I32 | Primitive::U32 | Primitive::Bool,
            length: Length::Two | Length::Three | Length::Four,
        }) => {}

        Type::Struct(ty) => {
            f.write_str("struct type");
            f.write_i128(ty_idx as i128);
            f.write_str(" {\n");

            let mut field_idx = 0;
            while field_idx < ty.fields.len() {
                let field = &ty.fields[field_idx];

                f.write_str("\tfield");
                f.write_i128(field.rust_offset as i128);
                f.write_str(": ");

                fmt_type_name(f, field.ty, shader);
                f.write_str(",\n");

                field_idx += 1;
            }

            f.write_str("}\n\n");
        }
    }
}

const fn fmt_type_name(f: &mut Formatter, ty: &'static Type, shader: &'static LinkedShader) {
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

        Type::Struct(_) => {
            f.write_str("type");
            f.write_i128(shader.type_id(ty) as i128);
        }
    }
}

const fn fmt_fn_decl(
    f: &mut Formatter,
    function: &'static Function,
    fn_idx: usize,
    shader: &'static LinkedShader,
) {
    let Function::UserDefined {
        entry_point_info,
        parameters,
        return_type,
        stmts,
        expr_bank,
        stmt_bank,
    } = function
    else {
        return match function {
            Function::BuiltIn(function) => fmt_builtin_fn_decl(f, function),
            Function::UserDefined { .. } => panic!(),
        };
    };

    match entry_point_info {
        Some(EntryPointInfo::Vertex(VertexFunctionInfo {
            input_attrs: _,
            output_attrs,
        })) => {
            f.write_str("struct fn");
            f.write_i128(fn_idx as i128);
            f.write_str("_output {\n");
            f.write_str("\t@builtin(position) position: vec4f,\n");

            let mut attr_idx = 0;
            while attr_idx < output_attrs.len() {
                f.write_str("\tlocation(");
                f.write_i128(attr_idx as i128);
                f.write_str(")\n");
                f.write_str("\tattr");
                f.write_i128(attr_idx as i128);
                f.write_str(": ");
                fmt_type_name(f, output_attrs[attr_idx], shader);
                f.write_str(",\n");

                attr_idx += 1;
            }

            f.write_str("}\n\n");
            f.write_str("@vertex\n");
        }

        Some(EntryPointInfo::Fragment(_)) => f.write_str("@fragment\n"),

        None => {}
    }

    f.write_str("fn fn");
    f.write_i128(fn_idx as i128);
    f.write_str("(");

    match entry_point_info {
        Some(EntryPointInfo::Vertex(VertexFunctionInfo {
            input_attrs,
            output_attrs: _,
        })) => {
            let mut attr_idx = 0;
            while attr_idx < input_attrs.len() {
                if attr_idx > 0 {
                    f.write_str(", ");
                }

                f.write_str("@location(");
                f.write_i128(attr_idx as i128);
                f.write_str(") attr");
                f.write_i128(attr_idx as i128);
                f.write_str(": ");
                fmt_type_name(f, input_attrs[attr_idx], shader);

                attr_idx += 1;
            }

            f.write_str(") -> fn");
            f.write_i128(fn_idx as i128);
            f.write_str("_output");
        }

        Some(EntryPointInfo::Fragment(FragmentFunctionInfo { input_attrs })) => {
            f.write_str("@builtin(position) position: vec4f");

            let mut attr_idx = 0;
            while attr_idx < input_attrs.len() {
                f.write_str(", ");

                let attr = input_attrs[attr_idx];

                f.write_str("@location(");
                f.write_i128(attr_idx as i128);
                f.write_str(") attr");
                f.write_i128(attr_idx as i128);
                f.write_str(": ");
                fmt_type_name(f, attr, shader);

                attr_idx += 1;
            }

            f.write_str(") -> @location(0) vec4f");
        }

        None => {
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
        }
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

const fn fmt_builtin_fn_decl(_f: &mut Formatter, function: &'static BuiltInFunction) {
    match function {
        BuiltInFunction::Neg(_) => {}
        BuiltInFunction::Not(_) => {}

        BuiltInFunction::Add(_, _) => {}
        BuiltInFunction::Sub(_, _) => {}
        BuiltInFunction::Mul(_, _) => {}
        BuiltInFunction::Div(_, _) => {}
        BuiltInFunction::Rem(_, _) => {}
        BuiltInFunction::Shl(_, _) => {}
        BuiltInFunction::Shr(_, _) => {}
        BuiltInFunction::BitAnd(_, _) => {}
        BuiltInFunction::BitOr(_, _) => {}
        BuiltInFunction::BitXor(_, _) => {}

        BuiltInFunction::Eq(_) => {}
        BuiltInFunction::Ne(_) => {}
        BuiltInFunction::Lt(_) => {}
        BuiltInFunction::Gt(_) => {}
        BuiltInFunction::Le(_) => {}
        BuiltInFunction::Ge(_) => {}

        BuiltInFunction::And => {}
        BuiltInFunction::Or => {}
    }
}

const fn fmt_stmt(
    f: &mut Formatter,
    stmt: &'static Stmt,
    expr_bank: &'static [Expr],
    stmt_bank: &'static [Stmt],
    tab_lvl: usize,
    shader: &'static LinkedShader,
) {
    let mut i = 0;
    while i < tab_lvl {
        f.write_str("\t");
        i += 1;
    }

    match stmt {
        Stmt::VariableDecl(Variable { id, ty }) => {
            f.write_str("var var");
            f.write_i128(*id as i128);
            f.write_str(": ");
            fmt_type_name(f, ty, shader);
            f.write_str(";\n");
        }

        Stmt::Assignment(left, right) => {
            fmt_place(f, left, shader);
            f.write_str(" = ");
            fmt_expr(f, right, expr_bank, stmt_bank, shader);
            f.write_str(";\n");
        }

        Stmt::Return(expr) => {
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
    expr: &'static Expr,
    expr_bank: &'static [Expr],
    stmt_bank: &'static [Stmt],
    shader: &'static LinkedShader,
) {
    match expr {
        Expr::Literal(Literal::F32(value)) => {
            f.write_str("bitcast<f32>(0x");
            f.write_u32_hex(value.to_bits());
            f.write_str(">");
        }
        Expr::Literal(Literal::I32(value)) => {
            f.write_str("bitcast<i32>(0x");
            f.write_u32_hex(value.cast_unsigned());
            f.write_str(">");
        }
        Expr::Literal(Literal::U32(value)) => {
            f.write_str("bitcast<u32>(0x");
            f.write_u32_hex(*value);
            f.write_str(">");
        }
        Expr::Literal(Literal::Bool(false)) => f.write_str("false"),
        Expr::Literal(Literal::Bool(true)) => f.write_str("true"),

        Expr::Variable(Variable { id, ty: _ }) => {
            f.write_str("var");
            f.write_i128(*id as i128);
        }

        Expr::FunctionCall { function, args } => match function {
            Function::UserDefined { .. } => {
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

            Function::BuiltIn(BuiltInFunction::Neg(_)) => {
                f.write_str("-(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Not(_)) => {
                f.write_str("!(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Add(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") + (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Sub(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") - (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Mul(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") * (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Div(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") / (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Rem(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") % (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Shl(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") << (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Shr(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") >> (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::BitAnd(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") & (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::BitOr(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") | (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::BitXor(_, _)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") ^ (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Eq(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") == (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Ne(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") != (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Lt(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") < (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Gt(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") > (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Le(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") <= (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Ge(_)) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") >= (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::And) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") && (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
            Function::BuiltIn(BuiltInFunction::Or) => {
                f.write_str("(");
                fmt_expr(f, &expr_bank[args[0].0], expr_bank, stmt_bank, shader);
                f.write_str(") || (");
                fmt_expr(f, &expr_bank[args[1].0], expr_bank, stmt_bank, shader);
                f.write_str(")");
            }
        },
    }
}

const fn fmt_place(f: &mut Formatter, place: &'static Place, _shader: &'static LinkedShader) {
    match place {
        Place::Variable(Variable { id, ty: _ }) => {
            f.write_str("var");
            f.write_i128(*id as i128);
        }
    }
}
