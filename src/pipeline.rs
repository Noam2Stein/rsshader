use std::{marker::PhantomData, mem, sync::LazyLock};

use crate::constructs::{Fragment, FragmentFn, Vertex, VertexFn};

pub struct RenderPipeline<V: Vertex> {
    danny: PhantomData<V>,
    lazylock: LazyLock<String>,
}
impl<V: Vertex> RenderPipeline<V> {
    pub const fn new<F: Fragment, VFn: VertexFn<V, F>, FFn: FragmentFn<F>>() -> Self {
        Self {
            danny: unsafe { mem::transmute(()) },
            lazylock: LazyLock::new(|| format!("1 2 3 4 5 _ 7 _")),
        }
    }
    pub fn wgsl(&self) -> &str {
        &self.lazylock
    }
}
