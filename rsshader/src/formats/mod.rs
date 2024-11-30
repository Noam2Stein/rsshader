mod wgsl;
pub use wgsl::*;

use crate::{GPUType, RenderPipeline};

#[allow(private_bounds)]
pub trait ShaderFormat: Seal {
    fn format_render_pipeline<V: GPUType>(render_pipeline: &RenderPipeline<V>) -> String;
}

trait Seal {}
