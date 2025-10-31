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
        BodyIr, BuiltinFn, EntryPointIr, ExprIr, FnIr, FragInputIr, FragOutputIr, LinkedShaderIr,
        Literal, PlaceIr, Primitive, ShaderIr, StmtIr, TypeIr, VertexInputIr,
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
        types: LinkerVec<TypeIr, TY_CAP>,
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
                    self.vertex_inputs.link(input);
                    self.frag_inputs.link(output);

                    self.link_attr_iter(input.attrs());
                    self.link_attr_iter(output.attrs());
                    self.link_body(body);
                }
                EntryPointIr::Frag {
                    input,
                    output,
                    body,
                } => {
                    self.frag_inputs.link(input);
                    self.frag_outputs.link(output);

                    self.link_attr_iter(input.attrs());
                    self.link_attr_iter(output.attrs());
                    self.link_body(body);
                }
            }
        }

        const fn link_attr_iter(&mut self, mut iter: AttrIter) {
            while let Some(ty) = iter.next() {
                self.link_ty(ty);
            }
        }

        const fn link_ty(&mut self, ty: &'static TypeIr) {
            self.types.link(ty);

            match ty {
                TypeIr::Primitive(_) => {}

                TypeIr::Vector(ty) => match ty.primitive {
                    Primitive::F32 => self.link_ty(&TypeIr::Primitive(Primitive::F32)),
                    Primitive::I32 => self.link_ty(&TypeIr::Primitive(Primitive::I32)),
                    Primitive::U32 => self.link_ty(&TypeIr::Primitive(Primitive::U32)),
                    Primitive::Bool => self.link_ty(&TypeIr::Primitive(Primitive::Bool)),
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
                ExprIr::Literal(Literal::F32(_)) => {
                    self.link_ty(&TypeIr::Primitive(Primitive::F32))
                }
                ExprIr::Literal(Literal::I32(_)) => {
                    self.link_ty(&TypeIr::Primitive(Primitive::I32))
                }
                ExprIr::Literal(Literal::U32(_)) => {
                    self.link_ty(&TypeIr::Primitive(Primitive::U32))
                }
                ExprIr::Literal(Literal::Bool(_)) => {
                    self.link_ty(&TypeIr::Primitive(Primitive::Bool))
                }

                ExprIr::Variable(var) => {
                    let VariableIr { id: _, ty } = var;

                    self.link_ty(ty);
                }

                ExprIr::Call { func, args } => {
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

        const fn link_fn(&mut self, func: &'static FnIr) {
            self.fns.link(func);

            match func {
                FnIr::UserDefined {
                    param_types: params,
                    ret_type: ret_ty,
                    body,
                } => {
                    let mut i = 0;
                    while i < params.len() {
                        let VariableIr { id: _, ty } = &params[i];

                        self.link_ty(ty);

                        i += 1;
                    }

                    if let Some(ret_ty) = ret_ty {
                        self.link_ty(ret_ty);
                    }

                    self.link_body(body);
                }

                FnIr::Builtin(BuiltinFn::ScalarNeg { ty }) => {
                    self.link_ty(ty.as_primitive().as_type())
                }
                FnIr::Builtin(BuiltinFn::VectorNeg { n, t }) => self.link_ty(&Type),

                FnIr::Builtin(BuiltinFn::Not { ty }) => self.link_ty(ty),

                FnIr::Builtin(BuiltinFn::Add { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::Sub { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::Mul { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::Div { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::Rem { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::Shl { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::Shr { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::BitAnd { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::BitOr { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }
                FnIr::Builtin(BuiltinFn::BitXor { left, right }) => {
                    self.link_ty(left);
                    self.link_ty(right);
                }

                FnIr::Builtin(BuiltinFn::Eq { ty }) => self.link_ty(ty),
                FnIr::Builtin(BuiltinFn::Ne { ty }) => self.link_ty(ty),
                FnIr::Builtin(BuiltinFn::Lt { ty }) => self.link_ty(ty),
                FnIr::Builtin(BuiltinFn::Le { ty }) => self.link_ty(ty),
                FnIr::Builtin(BuiltinFn::Ge { ty }) => self.link_ty(ty),

                FnIr::Builtin(BuiltinFn::And) => self.link_ty(&TypeIr::Primitive(Primitive::Bool)),
                FnIr::Builtin(BuiltinFn::Or) => self.link_ty(&TypeIr::Primitive(Primitive::Bool)),

                FnIr::Builtin(BuiltinFn::StructConstructor { ty }) => {
                    self.link_ty(ty);
                }
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
                type T = TypeIr;
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

pub mod linker2 {

    pub struct Linker<const CAP: usize> {}
}

mod buffer {
    use core::{marker::PhantomData, mem::MaybeUninit};

    #[derive(Clone, Copy)]
    pub struct Buffer<const CAP: usize> {
        bytes: MaybeUninit<[u8; CAP]>,
        next_idx: usize,
    }

    #[derive(Clone, Copy)]
    pub struct BufferPtr<T: Copy> {
        idx: usize,
        _danny: PhantomData<T>,
    }

    impl<const CAP: usize> Buffer<CAP> {
        pub const fn new() -> Self {
            Self {
                bytes: MaybeUninit::uninit(),
                next_idx: 0,
            }
        }

        pub const fn alloc<T: Copy>(&mut self, value: T) -> BufferPtr<T> {
            if self.next_idx + core::mem::size_of::<T>() > CAP {
                panic!("allocation exceeded capacity");
            }

            let idx = self.next_idx;
            self.next_idx += core::mem::size_of::<T>();

            // SAFETY: the pointer is guaranteed to be in bounds
            let ptr =
                unsafe { &mut *(self.bytes.as_mut_ptr().byte_add(idx) as *mut MaybeUninit<T>) };

            ptr.write(value);

            BufferPtr {
                idx,
                _danny: PhantomData,
            }
        }
    }

    impl<T: Copy> BufferPtr<T> {
        pub const unsafe fn access<'buf, const CAP: usize>(
            &self,
            buffer: &'buf Buffer<CAP>,
        ) -> &'buf T {
            // SAFETY:
            // the pointer is guaranteed to be in bounds and initialized as long as
            // buffer is the original buffer.
            unsafe {
                buffer
                    .bytes
                    .as_ptr()
                    .byte_add(self.idx)
                    .cast::<MaybeUninit<T>>()
                    .as_ref()
                    .unwrap_unchecked()
                    .assume_init_ref()
            }
        }
    }
}
