use core::marker::PhantomData;

use rsshader_macros::ConstEq;

use crate::ir::{BodyIr, Inner, Iter, LinkedShaderIr, TypeIr};

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum EntryPointIr {
    Vertex {
        input: &'static VertexInputIr,
        output: &'static FragInputIr,
        body: BodyIr,
    },
    Frag {
        input: &'static FragInputIr,
        output: &'static FragOutputIr,
        body: BodyIr,
    },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct VertexInputIr(pub &'static TypeIr);

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct FragInputIr(pub &'static TypeIr);

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct FragOutputIr(pub &'static TypeIr);

impl EntryPointIr {
    pub const fn id(&self, shader: &LinkedShaderIr) -> usize {
        let mut i = 0;
        loop {
            if shader.entry_points[i].eq(self) {
                break i;
            }

            i += 1;
        }
    }
}

impl VertexInputIr {
    pub const fn attrs(&self) -> Iter<&'static TypeIr> {
        Iter(Inner::Attributes { ty: self.0, idx: 0 }, PhantomData)
    }

    pub const fn id(&self, shader: &LinkedShaderIr) -> usize {
        let mut i = 0;
        loop {
            if shader.vertex_inputs[i].eq(self) {
                break i;
            }

            i += 1;
        }
    }
}

impl FragInputIr {
    pub const fn attrs(&self) -> Iter<&'static TypeIr> {
        Iter(Inner::Attributes { ty: self.0, idx: 0 }, PhantomData)
    }

    pub const fn id(&self, shader: &LinkedShaderIr) -> usize {
        let mut i = 0;
        loop {
            if shader.frag_inputs[i].eq(self) {
                break i;
            }

            i += 1;
        }
    }
}

impl FragOutputIr {
    pub const fn attrs(&self) -> Iter<&'static TypeIr> {
        Iter(Inner::Attributes { ty: self.0, idx: 0 }, PhantomData)
    }

    pub const fn id(&self, shader: &LinkedShaderIr) -> usize {
        let mut i = 0;
        loop {
            if shader.frag_outputs[i].eq(self) {
                break i;
            }

            i += 1;
        }
    }
}
