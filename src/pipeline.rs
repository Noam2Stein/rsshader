use std::{marker::PhantomData, mem, sync::LazyLock};

use crate::constructs::{FragmentFn, Vertex, VertexFn};

pub struct RenderPipeline<V: Vertex> {
    danny: PhantomData<V>,
    lazylock: LazyLock<String>,
}
impl<V: Vertex> RenderPipeline<V> {
    pub const fn new<VFn: VertexFn<I = V>, FFn: FragmentFn<I = VFn::O>>() -> Self {
        Self {
            danny: unsafe { mem::transmute(()) },
            lazylock: LazyLock::new(|| format!("1 2 3 4 5 _ 7 _")),
        }
    }
    pub fn wgsl(&self) -> &str {
        &self.lazylock
    }
}
