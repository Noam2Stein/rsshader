use rsshader_macros::ConstEq;

use crate::ir::{
    BuiltInFnIr, ExprIr, FnIr, LiteralIr, PlaceIr, PrimitiveIr, StmtIr, StructIr, TypeIr,
    VariableIr, VectorIr,
};

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct ShaderIr {
    pub entry_points: &'static [&'static FnIr],
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedShaderIr {
    pub types: &'static [&'static TypeIr],
    pub functions: &'static [&'static FnIr],
}

impl LinkedShaderIr {
    pub const fn type_id(&self, ty: &'static TypeIr) -> usize {
        let mut i = 0;
        while i < self.types.len() {
            if self.types[i].eq(ty) {
                return i;
            }

            i += 1;
        }

        panic!("Type not found");
    }

    pub const fn fn_id(&self, function: &'static FnIr) -> usize {
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
pub struct LinkedShaderIrBuffer<const TYPE_CAP: usize, const FN_CAP: usize> {
    pub types: [&'static TypeIr; TYPE_CAP],
    pub type_count: usize,
    pub functions: [&'static FnIr; FN_CAP],
    pub fn_count: usize,
}

#[doc(hidden)]
impl<const TYPE_CAP: usize, const FN_CAP: usize> LinkedShaderIrBuffer<TYPE_CAP, FN_CAP> {
    pub const fn as_ref(&'static self) -> LinkedShaderIr {
        assert!(self.type_count <= TYPE_CAP);
        assert!(self.fn_count <= FN_CAP);

        // SAFETY: the slices are valid because the type_count and fn_count are less than or equal to the constants
        LinkedShaderIr {
            types: &unsafe { core::slice::from_raw_parts(self.types.as_ptr(), self.type_count) },
            functions: &unsafe {
                core::slice::from_raw_parts(self.functions.as_ptr(), self.fn_count)
            },
        }
    }

    pub const fn link(shader: &ShaderIr) -> Self {
        let mut output = Self {
            types: [&TypeIr::Primitive(PrimitiveIr::F32); TYPE_CAP],
            type_count: 0,
            functions: [&FnIr::BuiltIn(BuiltInFnIr::Neg(&TypeIr::Primitive(PrimitiveIr::F32)));
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

    const fn link_fn(&mut self, function: &'static FnIr) {
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
            FnIr::UserDefined {
                entry_point_kind: _,
                parameters,
                return_type,
                stmts: _,
                expr_bank,
                stmt_bank,
            } => {
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

            FnIr::BuiltIn(function) => self.link_builtin_fn(function),
        }
    }

    const fn link_builtin_fn(&mut self, function: &'static BuiltInFnIr) {
        match function {
            BuiltInFnIr::Neg(a) => self.link_type(a),
            BuiltInFnIr::Not(a) => self.link_type(a),
            BuiltInFnIr::Add(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Sub(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Mul(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Div(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Rem(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Shl(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Shr(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::BitAnd(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::BitOr(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::BitXor(a, b) => {
                self.link_type(a);
                self.link_type(b);
            }
            BuiltInFnIr::Eq(a) => self.link_type(a),
            BuiltInFnIr::Ne(a) => self.link_type(a),
            BuiltInFnIr::Lt(a) => self.link_type(a),
            BuiltInFnIr::Gt(a) => self.link_type(a),
            BuiltInFnIr::Le(a) => self.link_type(a),
            BuiltInFnIr::Ge(a) => self.link_type(a),
            BuiltInFnIr::And => self.link_type(&TypeIr::Primitive(PrimitiveIr::Bool)),
            BuiltInFnIr::Or => self.link_type(&TypeIr::Primitive(PrimitiveIr::Bool)),

            BuiltInFnIr::StructConstructor { ty } => self.link_type(ty),
        }
    }

    const fn link_type(&mut self, ty: &'static TypeIr) {
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
            TypeIr::Primitive(_) => {}

            TypeIr::Vector(VectorIr {
                length: _,
                primitive: PrimitiveIr::F32,
            }) => self.link_type(&TypeIr::Primitive(PrimitiveIr::F32)),

            TypeIr::Vector(VectorIr {
                length: _,
                primitive: PrimitiveIr::I32,
            }) => self.link_type(&TypeIr::Primitive(PrimitiveIr::I32)),

            TypeIr::Vector(VectorIr {
                length: _,
                primitive: PrimitiveIr::U32,
            }) => self.link_type(&TypeIr::Primitive(PrimitiveIr::U32)),

            TypeIr::Vector(VectorIr {
                length: _,
                primitive: PrimitiveIr::Bool,
            }) => self.link_type(&TypeIr::Primitive(PrimitiveIr::Bool)),

            TypeIr::Struct(StructIr { fields }) => {
                let mut i = 0;
                while i < fields.len() {
                    self.link_type(fields[i].ty);
                    i += 1;
                }
            }

            TypeIr::VertexAttributes(ty)
            | TypeIr::FragmentAttributes(ty)
            | TypeIr::RenderOutputAttributes(ty) => self.link_type(ty),
        }
    }

    const fn link_expr(&mut self, expr: &'static ExprIr) {
        match expr {
            ExprIr::Literal(LiteralIr::F32(_)) => {
                self.link_type(&TypeIr::Primitive(PrimitiveIr::F32))
            }
            ExprIr::Literal(LiteralIr::I32(_)) => {
                self.link_type(&TypeIr::Primitive(PrimitiveIr::I32))
            }
            ExprIr::Literal(LiteralIr::U32(_)) => {
                self.link_type(&TypeIr::Primitive(PrimitiveIr::U32))
            }
            ExprIr::Literal(LiteralIr::Bool(_)) => {
                self.link_type(&TypeIr::Primitive(PrimitiveIr::Bool))
            }

            ExprIr::Variable(VariableIr { ty, .. }) => self.link_type(ty),
            ExprIr::FunctionCall { function, .. } => self.link_fn(function),
        }
    }

    const fn link_place(&mut self, place: &'static PlaceIr) {
        match place {
            PlaceIr::Variable(VariableIr { ty, .. }) => self.link_type(ty),
        }
    }

    const fn link_stmt(&mut self, stmt: &'static StmtIr) {
        match stmt {
            StmtIr::VariableDecl(VariableIr { ty, .. }) => self.link_type(ty),

            StmtIr::Assignment(place, expr) => {
                self.link_place(place);
                self.link_expr(expr);
            }

            StmtIr::Return(Some(expr)) => {
                self.link_expr(expr);
            }
            StmtIr::Return(None) => {}
        }
    }
}
