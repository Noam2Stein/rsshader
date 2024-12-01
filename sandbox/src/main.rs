use rsshader::*;

fn main() {
    println!("{}", FUNNY.format::<WGSL>());
}

#[gpu_type]
struct Vertex {
    _pos: [f32; 3],
    _color: [f32; 3],
}

#[gpu_fn]
fn vs_main(vertex: Vertex) -> Vertex {
    vertex
}

#[gpu_fn]
fn fs_main(fragment: Vertex) -> Vertex {
    fragment
}

const FUNNY: RenderPipeline<Vertex> = render_pipeline!(vs_main, fs_main).optimize::<WGSL>();
