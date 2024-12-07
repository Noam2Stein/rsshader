use std::{marker::PhantomData, mem::forget, ops::Deref};

use crate::{GPUFn, GPUType, ShaderFormat};

pub struct RenderPipeline<V: GPUType> {
    danny: PhantomData<V>,
    wgsl: &'static str,
}
impl<V: GPUType> RenderPipeline<V> {
    pub const unsafe fn new_unchecked(
        vertex_fn: &'static GPUFnDesc,
        fragment_fn: &'static GPUFnDesc,
    ) -> Self {
        Self {
            danny: std::mem::transmute(()),
            vertex_fn,
            fragment_fn,
        }
    }

    pub const fn new<
        VFn: GPUFn + Deref<Target = fn(V) -> F>,
        FFn: GPUFn + Deref<Target = fn(F) -> O>,
        F: GPUType,
        O: FragmentFnOutput,
    >(
        _f_vertex: VFn,
        _f_fragment: FFn,
    ) -> Self {
        forget(_f_vertex);
        forget(_f_fragment);

        Self {
            danny: unsafe { std::mem::transmute(()) },
            vertex_fn: &VFn::FN_DESC,
            fragment_fn: &FFn::FN_DESC,
        }
    }

    pub const fn optimize<F: ShaderFormat>(self) -> Self {
        self
    }
    pub fn format<F: ShaderFormat>(&self) -> String {
        F::format_render_pipeline(self)
    }

    pub const fn vertex_fn(&self) -> &'static GPUFnDesc {
        self.vertex_fn
    }
    pub const fn fragment_fn(&self) -> &'static GPUFnDesc {
        self.fragment_fn
    }
}

pub trait FragmentFnOutput: GPUType {}

impl FragmentFnOutput for [f32; 4] {}
