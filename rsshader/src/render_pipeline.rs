use std::marker::PhantomData;

use crate::GPUType;

pub use rsshader_proc_macros::render_pipeline;

pub struct RenderPipeline<V: GPUType> {
    danny: PhantomData<V>,
    wgsl: &'static str,
}
impl<V: GPUType> RenderPipeline<V> {
    pub const unsafe fn new_unchecked(wgsl: &'static str) -> Self {
        Self {
            danny: std::mem::transmute(()),
            wgsl,
        }
    }

    pub const fn wgsl(&self) -> &'static str {
        self.wgsl
    }
}

pub trait FragmentFnOutput: GPUType {}

impl FragmentFnOutput for [f32; 4] {}
