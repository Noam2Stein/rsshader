use rsshader_macros::ConstEq;

use crate::ir::{
    BuiltInFunction, EntryPointInfo, Expr, Function, Literal, Place, Primitive, Stmt, Struct, Type,
    Variable, Vector,
};

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct Shader {
    pub entry_points: &'static [&'static Function],
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedShader {
    pub types: &'static [&'static Type],
    pub functions: &'static [&'static Function],
}

impl LinkedShader {
    pub const fn type_id(&self, ty: &'static Type) -> usize {
        let mut i = 0;
        while i < self.types.len() {
            if self.types[i].eq(ty) {
                return i;
            }

            i += 1;
        }

        panic!("Type not found");
    }

    pub const fn fn_id(&self, function: &'static Function) -> usize {
        let mut i = 0;
        while i < self.functions.len() {
            if self.functions[i].eq(function) {
                return i;
            }
            i += 1;
        }

        panic!("Function not found");
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct LinkedShaderBuffer<const TYPE_CAP: usize, const FN_CAP: usize> {
    pub types: [&'static Type; TYPE_CAP],
    pub type_count: usize,
    pub functions: [&'static Function; FN_CAP],
    pub fn_count: usize,
}

#[doc(hidden)]
impl<const TYPE_CAP: usize, const FN_CAP: usize> LinkedShaderBuffer<TYPE_CAP, FN_CAP> {
    pub const fn as_ref(&'static self) -> LinkedShader {
        assert!(self.type_count <= TYPE_CAP);
        assert!(self.fn_count <= FN_CAP);

        // SAFETY: the slices are valid because the type_count and fn_count are less than or equal to the constants
        LinkedShader {
            types: &unsafe { core::slice::from_raw_parts(self.types.as_ptr(), self.type_count) },
            functions: &unsafe {
                core::slice::from_raw_parts(self.functions.as_ptr(), self.fn_count)
            },
        }
    }

    pub const fn link(shader: &Shader) -> Self {
        let mut output = Self {
            types: [&Type::Primitive(Primitive::F32); TYPE_CAP],
            type_count: 0,
            functions: [&Function::BuiltIn(BuiltInFunction::Neg(&Type::Primitive(Primitive::F32)));
                FN_CAP],
            fn_count: 0,
        };

        let mut i = 0;
        while i < shader.entry_points.len() {
            output.link_fn(shader.entry_points[i]);
            i += 1;
        }

        output
    }

    const fn link_fn(&mut self, function: &'static Function) {
        let mut i = 0;
        while i < self.fn_count {
            if self.functions[i].eq(function) {
                return;
            }
            i += 1;
        }

        self.functions[self.fn_count] = function;
        self.fn_count += 1;

        match function {
            Function::UserDefined {
                entry_point_info,
                parameters,
                return_type,
                stmts: _,
                expr_bank,
                stmt_bank,
            } => {
                match entry_point_info {
                    Some(EntryPointInfo::Vertex(info)) => {
                        let mut i = 0;
                        while i < info.input_attrs.len() {
                            self.link_type(info.input_attrs[i]);
                            i += 1;
                        }

                        i = 0;
                        while i < info.output_attrs.len() {
                            self.link_type(info.output_attrs[i]);
                            i += 1;
                        }
                    }
                    Some(EntryPointInfo::Fragment(info)) => {
                        let mut i = 0;
                        while i < info.input_attrs.len() {
                            self.link_type(info.input_attrs[i]);
                            i += 1;
                        }
                    }
                    None => {}
                };

                let mut i = 0;
                while i < parameters.len() {
                    self.link_type(parameters[i].ty);
                    i += 1;
                }

                if let Some(return_type) = return_type {
                    self.link_type(return_type);
                }

                let mut i = 0;
                while i < expr_bank.len() {
                    self.link_expr(&expr_bank[i]);
                    i += 1;
                }

                let mut i = 0;
                while i < stmt_bank.len() {
                    self.link_stmt(&stmt_bank[i]);
                    i += 1;
                }
            }

            Function::BuiltIn(function) => self.link_builtin_fn(function),
        }
    }

    const fn link_builtin_fn(&mut self, function: &'static BuiltInFunction) {
        match function {
            BuiltInFunction::Neg(a) => self.link_type(a),
            BuiltInFunction::Not(a) => self.link_type(a),
            BuiltInFunction::Add(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Sub(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Mul(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Div(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Rem(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Shl(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Shr(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::BitAnd(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::BitOr(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::BitXor(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFunction::Eq(a) => self.link_type(a),
            BuiltInFunction::Ne(a) => self.link_type(a),
            BuiltInFunction::Lt(a) => self.link_type(a),
            BuiltInFunction::Gt(a) => self.link_type(a),
            BuiltInFunction::Le(a) => self.link_type(a),
            BuiltInFunction::Ge(a) => self.link_type(a),
            BuiltInFunction::And => self.link_type(&Type::Primitive(Primitive::Bool)),
            BuiltInFunction::Or => self.link_type(&Type::Primitive(Primitive::Bool)),
        }
    }

    const fn link_type(&mut self, ty: &'static Type) {
        let mut i = 0;
        while i < self.type_count {
            if self.types[i].eq(ty) {
                return;
            }
            i += 1;
        }

        self.types[self.type_count] = ty;
        self.type_count += 1;

        match ty {
            Type::Primitive(_) => {}

            Type::Vector(Vector {
                length: _,
                primitive: Primitive::F32,
            }) => self.link_type(&Type::Primitive(Primitive::F32)),

            Type::Vector(Vector {
                length: _,
                primitive: Primitive::I32,
            }) => self.link_type(&Type::Primitive(Primitive::I32)),

            Type::Vector(Vector {
                length: _,
                primitive: Primitive::U32,
            }) => self.link_type(&Type::Primitive(Primitive::U32)),

            Type::Vector(Vector {
                length: _,
                primitive: Primitive::Bool,
            }) => self.link_type(&Type::Primitive(Primitive::Bool)),

            Type::Struct(Struct { fields }) => {
                let mut i = 0;
                while i < fields.len() {
                    self.link_type(fields[i].ty);
                    i += 1;
                }
            }
        }
    }

    const fn link_expr(&mut self, expr: &'static Expr) {
        match expr {
            Expr::Literal(Literal::F32(_)) => self.link_type(&Type::Primitive(Primitive::F32)),
            Expr::Literal(Literal::I32(_)) => self.link_type(&Type::Primitive(Primitive::I32)),
            Expr::Literal(Literal::U32(_)) => self.link_type(&Type::Primitive(Primitive::U32)),
            Expr::Literal(Literal::Bool(_)) => self.link_type(&Type::Primitive(Primitive::Bool)),

            Expr::Variable(Variable { ty, .. }) => self.link_type(ty),
            Expr::FunctionCall { function, .. } => self.link_fn(function),
        }
    }

    const fn link_place(&mut self, place: &'static Place) {
        match place {
            Place::Variable(Variable { ty, .. }) => self.link_type(ty),
        }
    }

    const fn link_stmt(&mut self, stmt: &'static Stmt) {
        match stmt {
            Stmt::VariableDecl(Variable { ty, .. }) => self.link_type(ty),

            Stmt::Assignment(place, expr) => {
                self.link_place(place);
                self.link_expr(expr);
            }

            Stmt::Return(Some(expr)) => {
                self.link_expr(expr);
            }
            Stmt::Return(None) => {}
        }
    }
}
