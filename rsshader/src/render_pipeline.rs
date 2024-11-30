use std::marker::PhantomData;

use super::*;

pub use rsshader_proc_macros::render_pipeline;

pub struct RenderPipeline<V: GPUType> {
    danny: PhantomData<V>,
    vertex_fn: &'static GPUFnDesc<'static>,
    fragment_fn: &'static GPUFnDesc<'static>,
}
impl<V: GPUType> RenderPipeline<V> {
    pub const unsafe fn new_unchecked(
        vertex_fn: &'static GPUFnDesc<'static>,
        fragment_fn: &'static GPUFnDesc<'static>,
    ) -> Self {
        Self {
            danny: std::mem::transmute(()),
            vertex_fn,
            fragment_fn,
        }
    }

    pub const fn optimize<F: ShaderFormat>(self) -> Self {
        self
    }
    pub fn format<F: ShaderFormat>(&self) -> String {
        F::format_render_pipeline(self)
    }

    pub const fn vertex_fn(&self) -> &'static GPUFnDesc<'static> {
        self.vertex_fn
    }
    pub const fn fragment_fn(&self) -> &'static GPUFnDesc<'static> {
        self.fragment_fn
    }
}
