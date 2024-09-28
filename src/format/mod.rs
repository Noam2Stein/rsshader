use crate::{constructs::*, shader_core::*};

mod wgsl;
pub use wgsl::*;

pub trait RenderPipelineFormat {
    fn get<V: Vertex>(pipeline: &RenderPipeline<V>) -> Self;
}
