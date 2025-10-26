use ggmath::{
    f32::{FVec2, FVec3, FVec4},
    vec4,
};
use rsshader::{shader_item, wgsl};

fn main() {
    println!("{}", wgsl!(vs_main, fs_main));
}

#[shader_item]
#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: FVec2,
    color: FVec3,
}

#[shader_item(fragment)]
#[derive(Debug, Copy, Clone)]
struct Fragment {
    #[position]
    position: FVec4,
    color: FVec4,
}

#[shader_item(vertex)]
fn vs_main(vertex: Vertex) -> Fragment {
    Fragment {
        position: vec4!(vertex.position, 0.0, 1.0),
        color: vec4!(vertex.color, 1.0),
    }
}

#[shader_item(fragment)]
fn fs_main(fragment: Fragment) -> FVec4 {
    fragment.color
}
