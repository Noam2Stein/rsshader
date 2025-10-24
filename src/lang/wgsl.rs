use crate::{
    ir::{
        Array, EntryKind, Expr, FieldKind, Function, Length, LinkedShader, Literal, Matrix, Place,
        Primitive, Stmt, Type, Variable, Vector,
    },
    lang::Formatter,
};

#[macro_export]
macro_rules! wgsl {
    ($ir:expr) => {
        $crate::shader!($ir => $crate::lang::fmt_wgsl)
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
    match function.entry_kind {
        Some(EntryKind::Vertex) => f.write_str("@vertex\n"),
        Some(EntryKind::Fragment) => f.write_str("@fragment\n"),
        None => {}
    }

    f.write_str("fn fn");
    f.write_i128(fn_idx as i128);
    f.write_str("(");

    let mut param_idx = 0;
    while param_idx < function.parameters.len() {
        if param_idx > 0 {
            f.write_str(", ");
        }

        let param = function.parameters[param_idx];

        f.write_str("var");
        f.write_i128(param.id as i128);
        f.write_str(": ");
        fmt_type_name(f, param.ty, shader);

        param_idx += 1;
    }

    f.write_str(")");

    if let Some(return_type) = function.return_type {
        f.write_str(" -> ");
        fmt_type_name(f, return_type, shader);
    }

    f.write_str(" {\n");

    let mut stmt_idx = 0;
    while stmt_idx < function.stmts.len() {
        let stmt = &function.stmt_bank[function.stmts[stmt_idx]];

        fmt_stmt(f, &stmt, function, 1, shader);

        stmt_idx += 1;
    }

    f.write_str("}\n\n");
}

const fn fmt_stmt(
    f: &mut Formatter,
    stmt: &'static Stmt,
    outer_fn: &'static Function,
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
            fmt_expr(f, right, outer_fn, shader);
            f.write_str(";\n");
        }

        Stmt::Return(expr) => {
            f.write_str("return");

            if let Some(expr) = expr {
                f.write_str(" ");
                fmt_expr(f, expr, outer_fn, shader);
            }

            f.write_str(";\n");
        }
    }
}

const fn fmt_expr(
    f: &mut Formatter,
    expr: &'static Expr,
    outer_fn: &'static Function,
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

        Expr::FunctionCall { function, args } => {
            f.write_str("fn");
            f.write_i128(shader.fn_id(function) as i128);
            f.write_str("(");

            let mut arg_idx = 0;
            while arg_idx < args.len() {
                if arg_idx > 0 {
                    f.write_str(", ");
                }

                fmt_expr(f, &outer_fn.expr_bank[args[arg_idx].0], outer_fn, shader);

                arg_idx += 1;
            }

            f.write_str(")");
        }

        Expr::Neg(expr) => {
            let expr = &outer_fn.expr_bank[expr.0];

            f.write_str("-(");
            fmt_expr(f, expr, outer_fn, shader);
            f.write_str(")");
        }

        Expr::Add(left, right) => {
            f.write_str("(");
            fmt_expr(f, &outer_fn.expr_bank[left.0], outer_fn, shader);
            f.write_str(") + (");
            fmt_expr(f, &outer_fn.expr_bank[right.0], outer_fn, shader);
            f.write_str(")");
        }

        Expr::Sub(left, right) => {
            f.write_str("(");
            fmt_expr(f, &outer_fn.expr_bank[left.0], outer_fn, shader);
            f.write_str(") - (");
            fmt_expr(f, &outer_fn.expr_bank[right.0], outer_fn, shader);
            f.write_str(")");
        }

        Expr::Mul(left, right) => {
            f.write_str("(");
            fmt_expr(f, &outer_fn.expr_bank[left.0], outer_fn, shader);
            f.write_str(") * (");
            fmt_expr(f, &outer_fn.expr_bank[right.0], outer_fn, shader);
            f.write_str(")");
        }

        Expr::Div(left, right) => {
            f.write_str("(");
            fmt_expr(f, &outer_fn.expr_bank[left.0], outer_fn, shader);
            f.write_str(") / (");
            fmt_expr(f, &outer_fn.expr_bank[right.0], outer_fn, shader);
            f.write_str(")");
        }
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
