use super::*;

pub struct WGSL;

impl ShaderFormat for WGSL {
    fn format_render_pipeline<V: GPUType>(_render_pipeline: &RenderPipeline<V>) -> String {
        format!("todo!")
    }
}

impl Seal for WGSL {}
