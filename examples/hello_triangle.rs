use rsshader::{format::Wgsl, shader_core::*};

#[gpu(vertex)]
struct Vertex {
    pos: Vec2,
    color: Vec3,
}
#[gpu(fragment)]
struct Fragment {
    #[fragment_pos]
    pos: Vec4,
    color: Vec4,
}

#[gpu(vertex_fn)]
fn vs_main(vertex: Vertex) -> Fragment {
    Fragment {
        pos: vec4((vertex.pos, 0.0, 1.0)),
        color: vec4((vertex.color, 1.0)),
    }
}
#[gpu(fragment_fn)]
fn fs_main(fragment: Fragment) -> Vec4 {
    fragment.color
}

const HELLO_TRIANGLE: RenderPipeline<Vertex> =
    RenderPipeline::new::<vs_main_GPUFn, fs_main_GPUFn>().optimize::<Wgsl>();

fn main() {
    println!("{}", HELLO_TRIANGLE.format::<Wgsl>().as_str())
}
