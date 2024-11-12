use rsshader::*;

fn main() {
    println!("{:?}", Vertex::DESC);
}

#[derive(GPUType)]
struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
}
