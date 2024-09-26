use rsshader::{shader_core::*, WGSLPipeline};

#[gpu(vertex)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Vec4,
}
#[gpu(fragment)]
pub struct Fragment {
    #[fragment_pos]
    pub pos: Vec4,
    pub color: Vec4,
}

#[gpu(vertex_fn)]
pub fn vs_main(input: Vertex) -> Fragment {
    Fragment {
        pos: Vec4 {
            x: input.pos.x,
            y: input.pos.y,
            z: 0.0,
            w: 1.1,
        },
        color: input.color,
    }
}
#[gpu(fragment_fn)]
pub fn fs_main(input: Fragment) -> Vec4 {
    input.color
}

pub static HELLO_TRIANGLE: WGSLPipeline<Vertex> =
    WGSLPipeline::new::<Fragment, vs_main_as_gpu_fn, fs_main_as_gpu_fn>();
