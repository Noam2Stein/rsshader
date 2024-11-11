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
    color: Vec3,
}

#[gpu(vertex_fn)]
fn vs_main(vertex: Vertex) -> Fragment {
    Fragment {
        pos: vec4((vertex.pos, 0.0, 1.0)),
        color: vertex.color,
    }
}
#[gpu(fragment_fn)]
fn fs_main(fragment: Fragment) -> Vec4 {
    vec4((fragment.color, 1.0))
}

const HELLO_TRIANGLE: RenderPipeline<Vertex> =
    RenderPipeline::new::<gpufn!(vs_main), gpufn!(fs_main)>().optimize::<Wgsl>();

fn main() {
    println!("{}", HELLO_TRIANGLE.format::<Wgsl>().formatted_str())
}
