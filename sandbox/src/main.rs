use rsshader::{gpu, GPUType};

fn main() {
    println!("{}", Funny::GPU_TYPE_INFO.item_info.wgsl_declaration)
}

#[gpu]
struct Funny {
    position: [f32; 2],
    color: [f32; 3],
}
