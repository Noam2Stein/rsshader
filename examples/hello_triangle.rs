use rsshader::shader_core::*;

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
        pos: Vec4 {
            x: vertex.pos.x,
            y: vertex.pos.y,
            z: 0.0,
            w: 1.0,
        },
        color: Vec4 {
            x: vertex.color.x,
            y: vertex.color.y,
            z: vertex.color.z,
            w: 1.0,
        },
    }
}
#[gpu(fragment_fn)]
fn fs_main(fragment: Fragment) -> Vec4 {
    fragment.color
}

const PIPELINE: RenderPipeline<Vertex> = RenderPipeline::new::<vs_main_GPUFn, fs_main_GPUFn>();

fn main() {
    println!("{}", PIPELINE.wgsl())
}
