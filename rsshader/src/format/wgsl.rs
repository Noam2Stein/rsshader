use crate::constructs::*;

use super::*;

#[derive(Debug, Clone)]
pub struct Wgsl {
    s: String,
}
impl Wgsl {
    #[inline(always)]
    pub fn str(&self) -> String {
        self.s.clone()
    }
    #[inline(always)]
    pub fn formatted_str(&self) -> String {
        self.s.clone()
    }

    pub(crate) fn from_render_pipeline<V: GPUVertex>(_pipeline: &RenderPipeline<V>) -> Self {
        Self {
            s: format!("1 & 2 & 3 & 4 1 & & 3 4 & & 2 3 & 2 &"),
        }
    }
}
impl RenderPipelineFormat for Wgsl {
    fn get<V: GPUVertex>(pipeline: &RenderPipeline<V>) -> Self {
        let mut wgsl = pipeline.wgsl.lock().unwrap();
        if let Some(output) = &*wgsl {
            output.clone()
        } else {
            let output = Self::from_render_pipeline(pipeline);
            *wgsl = Some(output.clone());
            output
        }
    }
}
