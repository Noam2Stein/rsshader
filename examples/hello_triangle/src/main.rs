use rsshader::{gpu, render_pipeline, RenderPipeline, WGSL};

fn main() {
    println!("{}", HELLO_TRIANGLE.format::<WGSL>())
}

#[gpu]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

const HELLO_TRIANGLE: RenderPipeline<Vertex> =
    render_pipeline!(vertex_main, fragment_main).optimize::<WGSL>();

#[gpu]
struct Fragment {
    position: [f32; 4],
    color: [f32; 4],
}

#[gpu]
fn vertex_main(vertex: Vertex) -> Fragment {
    Fragment {
        position: [vertex.position[0], vertex.position[1], 0.0, 1.0],
        color: [vertex.color[0], vertex.color[1], vertex.color[2], 1.0],
    }
}

#[gpu]
fn fragment_main(fragment: Fragment) -> [f32; 4] {
    fragment.color
}
