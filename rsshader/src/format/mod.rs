use crate::{constructs::*, shader_core::*};

mod wgsl;
pub use wgsl::*;

pub trait RenderPipelineFormat {
    fn get<V: GPUVertex>(pipeline: &RenderPipeline<V>) -> Self;
}
