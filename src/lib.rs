use std::marker::PhantomData;

pub mod constructs;
pub mod shader_core;

pub use rsshader_proc_macros::*;

pub struct Pipeline<V: constructs::Vertex> {
    danny: PhantomData<V>,
}
