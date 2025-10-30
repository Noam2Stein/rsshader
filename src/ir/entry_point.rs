use rsshader_macros::ConstEq;

use crate::ir::{BodyIr, LinkedShaderIr, TypeIr};

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

pub struct AttrIter {
    ty: &'static TypeIr,
    idx: usize,
}

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
    pub const fn iter(&self) -> AttrIter {
        AttrIter { ty: self.0, idx: 0 }
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
    pub const fn iter(&self) -> AttrIter {
        AttrIter { ty: self.0, idx: 0 }
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
    pub const fn iter(&self) -> AttrIter {
        AttrIter { ty: self.0, idx: 0 }
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

impl AttrIter {
    pub const fn next(&mut self) -> Option<&'static TypeIr> {
        let ty = Self::peek(self.ty, self.idx);
        self.idx += 1;

        ty
    }

    const fn peek(ty: &'static TypeIr, idx: usize) -> Option<&'static TypeIr> {
        match ty {
            TypeIr::Primitive(_) | TypeIr::Vector(_) => {
                if idx == 0 {
                    Some(ty)
                } else {
                    None
                }
            }

            TypeIr::Struct(ty) => {
                let mut field_idx = 0;
                let mut idx_in_field = idx;
                loop {
                    if field_idx < ty.fields.len() {
                        let field_attr_count = Self::count(ty.fields[field_idx]);
                        if idx_in_field < field_attr_count {
                            break Some(Self::peek(ty.fields[field_idx], idx_in_field).unwrap());
                        } else {
                            field_idx += 1;
                            idx_in_field -= field_attr_count;
                        }
                    } else {
                        break None;
                    }
                }
            }
        }
    }

    const fn count(ty: &'static TypeIr) -> usize {
        match ty {
            TypeIr::Primitive(_) | TypeIr::Vector(_) => 1,

            TypeIr::Struct(ty) => {
                let mut sum = 0;
                let mut i = 0;
                while i < ty.fields.len() {
                    sum += Self::count(ty.fields[i]);
                    i += 1;
                }

                sum
            }
        }
    }
}
