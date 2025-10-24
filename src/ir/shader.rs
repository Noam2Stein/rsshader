use rsshader_macros::ConstEq;

use crate::ir::{Array, Function, Matrix, Primitive, Struct, Type, Vector};

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct Shader {
    pub entries: &'static [&'static Function],
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedShader {
    pub types: &'static [&'static Type],
    pub functions: &'static [&'static Function],
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct LinkedShaderBuffer<const TYPE_CAP: usize, const FN_CAP: usize> {
    pub types: [&'static Type; TYPE_CAP],
    pub functions: [&'static Function; FN_CAP],
    pub type_count: usize,
    pub fn_count: usize,
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
            functions: [&Function {
                entry_kind: None,
                parameters: &[],
                return_type: None,
                stmts: &[],
                expr_bank: &[],
                stmt_bank: &[],
            }; FN_CAP],
            type_count: 0,
            fn_count: 0,
        };

        let mut i = 0;
        while i < shader.entries.len() {
            let entry = shader.entries[i];
            output.link_fn(entry);
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

        let mut i = 0;
        while i < function.parameters.len() {
            self.link_type(function.parameters[i].ty);
            i += 1;
        }

        if let Some(return_type) = function.return_type {
            self.link_type(return_type);
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

            Type::Matrix(Matrix {
                rows: _,
                columns: _,
            }) => self.link_type(&Type::Primitive(Primitive::F32)),

            Type::Array(Array {
                length: _,
                element_type,
            }) => self.link_type(element_type),

            Type::Struct(Struct { fields }) => {
                let mut i = 0;
                while i < fields.len() {
                    self.link_type(fields[i].ty);
                    i += 1;
                }
            }
        }
    }
}
