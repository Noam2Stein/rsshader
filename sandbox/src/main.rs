use rsshader::shader_core::*;

#[gpu(fragment)]
struct _Fragment {
    #[fragment_pos]
    pos: Vec4,
    #[flat]
    texture_id: u32,
}

fn main() {}
