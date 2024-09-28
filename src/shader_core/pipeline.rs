use std::{marker::PhantomData, mem, sync::Mutex};

use crate::{constructs::*, format::*};

pub struct RenderPipeline<V: Vertex> {
    danny: PhantomData<V>,
    pub(crate) wgsl: Mutex<Option<Wgsl>>,
}
impl<V: Vertex> RenderPipeline<V> {
    pub const fn new<VFn: VertexFn<Input = V>, FFn: FragmentFn<Input = VFn::Output>>() -> Self {
        Self {
            danny: unsafe { mem::transmute(()) },
            wgsl: Mutex::new(None),
        }
    }
    pub const fn optimize<F: RenderPipelineFormat>(self) -> Self {
        self
    }
    pub fn format<F: RenderPipelineFormat>(&self) -> F {
        F::get(self)
    }
}
