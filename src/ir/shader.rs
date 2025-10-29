use rsshader_macros::ConstEq;

use crate::ir::{EntryPointIr, FnIr, FragInputIr, FragOutputIr, TypeIr, VertexInputIr};

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct ShaderIr {
    pub entry_points: &'static [&'static EntryPointIr],
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedShaderIr {
    pub vertex_inputs: &'static [&'static VertexInputIr],
    pub frag_inputs: &'static [&'static FragInputIr],
    pub frag_outputs: &'static [&'static FragOutputIr],
    pub types: &'static [&'static TypeIr],
    pub entry_points: &'static [&'static EntryPointIr],
    pub fns: &'static [&'static FnIr],
}

#[doc(hidden)]
pub mod linker {
    use core::mem::MaybeUninit;

    use crate::ir::{
        AttrIr, AttrIrIter, BodyIr, BuiltInFnIr, EntryPointIr, ExprIr, FnIr, FragInputIr,
        FragOutputIr, LinkedShaderIr, LiteralIr, PlaceIr, PrimitiveIr, ShaderIr, StmtIr, TypeIr,
        VariableIr, VertexInputIr,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Linker<
        const VERTEX_INPUT_CAP: usize,
        const FRAG_INPUT_CAP: usize,
        const FRAG_OUTPUT_CAP: usize,
        const TY_CAP: usize,
        const ENTRY_POINT_CAP: usize,
        const FN_CAP: usize,
    > {
        vertex_inputs: LinkerVec<&'static VertexInputIr, VERTEX_INPUT_CAP>,
        frag_inputs: LinkerVec<&'static FragInputIr, FRAG_INPUT_CAP>,
        frag_outputs: LinkerVec<&'static FragOutputIr, FRAG_OUTPUT_CAP>,
        types: LinkerVec<&'static TypeIr, TY_CAP>,
        entry_points: LinkerVec<&'static EntryPointIr, ENTRY_POINT_CAP>,
        fns: LinkerVec<&'static FnIr, FN_CAP>,
    }

    #[derive(Debug, Clone, Copy)]
    struct LinkerVec<T: Copy, const CAP: usize> {
        buf: [MaybeUninit<T>; CAP],
        len: usize,
    }

    impl<
        const VERTEX_INPUT_CAP: usize,
        const FRAG_INPUT_CAP: usize,
        const FRAG_OUTPUT_CAP: usize,
        const TY_CAP: usize,
        const ENTRY_POINT_CAP: usize,
        const FN_CAP: usize,
    > Linker<VERTEX_INPUT_CAP, FRAG_INPUT_CAP, FRAG_OUTPUT_CAP, TY_CAP, ENTRY_POINT_CAP, FN_CAP>
    {
        pub const fn new(shader: &'static ShaderIr) -> Self {
            let mut output = Self {
                vertex_inputs: LinkerVec::new(),
                frag_inputs: LinkerVec::new(),
                frag_outputs: LinkerVec::new(),
                types: LinkerVec::new(),
                entry_points: LinkerVec::new(),
                fns: LinkerVec::new(),
            };

            let mut i = 0;
            while i < shader.entry_points.len() {
                output.link_entry_point(shader.entry_points[i]);

                i += 1;
            }

            output
        }

        pub const fn view(&'static self) -> LinkedShaderIr {
            LinkedShaderIr {
                vertex_inputs: self.vertex_inputs.view(),
                frag_inputs: self.frag_inputs.view(),
                frag_outputs: self.frag_outputs.view(),
                types: self.types.view(),
                entry_points: self.entry_points.view(),
                fns: self.fns.view(),
            }
        }

        const fn link_entry_point(&mut self, entry_point: &'static EntryPointIr) {
            self.entry_points.link(entry_point);

            match entry_point {
                EntryPointIr::Vertex {
                    input,
                    output,
                    body,
                } => {
                    self.link_attr_iter(input.iter());
                    self.link_attr_iter(output.iter());
                    self.link_body(body);
                }
                EntryPointIr::Frag {
                    input,
                    output,
                    body,
                } => {
                    self.link_attr_iter(input.iter());
                    self.link_attr_iter(output.iter());
                    self.link_body(body);
                }
            }
        }

        const fn link_attr_iter(&mut self, mut iter: AttrIrIter) {
            while let Some(attr) = iter.next() {
                let AttrIr { ty, idx: _ } = attr;

                self.link_ty(ty);
            }
        }

        const fn link_ty(&mut self, ty: &'static TypeIr) {
            self.types.link(ty);

            match ty {
                TypeIr::Primitive(_) => {}

                TypeIr::Vector(ty) => match ty.primitive {
                    PrimitiveIr::F32 => self.link_ty(&TypeIr::Primitive(PrimitiveIr::F32)),
                    PrimitiveIr::I32 => self.link_ty(&TypeIr::Primitive(PrimitiveIr::I32)),
                    PrimitiveIr::U32 => self.link_ty(&TypeIr::Primitive(PrimitiveIr::U32)),
                    PrimitiveIr::Bool => self.link_ty(&TypeIr::Primitive(PrimitiveIr::Bool)),
                },

                TypeIr::Struct(ty) => {
                    let mut i = 0;
                    while i < ty.fields.len() {
                        self.link_ty(ty.fields[i]);
                        i += 1;
                    }
                }
            }
        }

        const fn link_body(&mut self, body: &BodyIr) {
            let BodyIr {
                stmts,
                expr_bank: _,
                stmt_bank: _,
            } = body;

            let mut i = 0;
            while i < stmts.len() {
                self.link_stmt(stmts[i].access(&body.stmt_bank), body);

                i += 1;
            }
        }

        const fn link_stmt(&mut self, stmt: &StmtIr, body: &BodyIr) {
            match stmt {
                StmtIr::VariableDecl { var } => {
                    let VariableIr { id: _, ty } = var;

                    self.link_ty(ty);
                }

                StmtIr::Assignment { left, right } => {
                    self.link_place(left, body);
                    self.link_expr(right, body);
                }

                StmtIr::Return { value: Some(value) } => {
                    self.link_expr(value, body);
                }

                StmtIr::Return { value: None } => {}
            }
        }

        const fn link_expr(&mut self, expr: &ExprIr, body: &BodyIr) {
            match expr {
                ExprIr::Literal(LiteralIr::F32(_)) => {
                    self.link_ty(&TypeIr::Primitive(PrimitiveIr::F32))
                }
                ExprIr::Literal(LiteralIr::I32(_)) => {
                    self.link_ty(&TypeIr::Primitive(PrimitiveIr::I32))
                }
                ExprIr::Literal(LiteralIr::U32(_)) => {
                    self.link_ty(&TypeIr::Primitive(PrimitiveIr::U32))
                }
                ExprIr::Literal(LiteralIr::Bool(_)) => {
                    self.link_ty(&TypeIr::Primitive(PrimitiveIr::Bool))
                }

                ExprIr::Variable(var) => {
                    let VariableIr { id: _, ty } = var;

                    self.link_ty(ty);
                }

                ExprIr::FunctionCall { func, args } => {
                    self.link_fn(func);

                    let mut i = 0;
                    while i < args.len() {
                        self.link_expr(args[i].access(&body.expr_bank), body);

                        i += 1;
                    }
                }
            }
        }

        const fn link_place(&mut self, place: &PlaceIr, _body: &BodyIr) {
            match place {
                PlaceIr::Variable(var) => {
                    let VariableIr { id: _, ty } = var;

                    self.link_ty(ty);
                }
            }
        }

        const fn link_fn(&mut self, func: &FnIr) {
            match func {
                FnIr::UserDefined {
                    params,
                    ret_ty,
                    body,
                } => {
                    let mut i = 0;
                    while i < params.len() {
                        let VariableIr { id: _, ty } = &params[i];

                        self.link_ty(ty);

                        i += 1;
                    }
                }

                FnIr::BuiltIn(BuiltInFnIr::Neg(ty)) => self.link_ty(ty),
                FnIr::BuiltIn(BuiltInFnIr::Not(ty)) => self.link_ty(ty),
                FnIr::BuiltIn(BuiltInFnIr::Add(ty)) => self.link_ty(ty),
            }
        }
    }

    impl<T: Copy, const CAP: usize> LinkerVec<T, CAP> {
        pub const fn new() -> Self {
            Self {
                buf: [MaybeUninit::uninit(); CAP],
                len: 0,
            }
        }
    }

    for_linked_types! {
        impl<const CAP: usize> LinkerVec<T, CAP> {
            pub const fn link(&mut self, value: T) -> bool {
                let mut i = 0;
                while i < self.len {
                    // SAFETY: all self.buf indices 0..self.len must be init.
                    let other_value = unsafe { &self.buf[i].assume_init() };
                    if other_value.eq(&value) {
                        return false;
                    }

                    i += 1;
                }

                self.buf[self.len] = MaybeUninit::new(value);
                self.len += 1;

                true
            }

            pub const fn view(&'static self) -> &'static [T] {
                // SAFETY: all self.buf indices 0..self.len must be init.
                unsafe { core::slice::from_raw_parts(self.buf.as_ptr() as *const T, self.len) }
            }
        }
    }

    macro_rules! for_linked_types {
        ($impl:item) => {
            mod _mod0 {
                use super::*;
                type T = &'static VertexInputIr;
                $impl
            }

            mod _mod1 {
                use super::*;
                type T = &'static FragInputIr;
                $impl
            }

            mod _mod2 {
                use super::*;
                type T = &'static FragOutputIr;
                $impl
            }

            mod _mod3 {
                use super::*;
                type T = &'static TypeIr;
                $impl
            }

            mod _mod4 {
                use super::*;
                type T = &'static EntryPointIr;
                $impl
            }

            mod _mod5 {
                use super::*;
                type T = &'static FnIr;
                $impl
            }
        };
    }

    use for_linked_types;
}
