# rsshader

a crate that turns a sub-section of Rust into shaders in various format like spirv and wgsl.

```rust
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
        pos: vec4((vertex.pos, 0.0, 1.0)),
        color: vec4((vertex.color, 1.0)),
    }
}
#[gpu(fragment_fn)]
fn fs_main(fragment: Fragment) -> Vec4 {
    fragment.color
}

const HELLO_TRIANGLE: RenderPipeline<Vertex> = RenderPipeline::new::<gpufn!(vs_main), gpufn!(fs_main)>();

fn main() {
    println!("{}", HELLO_TRIANGLE.wgsl())
}

```
## features

### compile time validation

shaders are validated at compile time to be core runnable on the gpu.

### extendability

make APIs that can be used to make shaders.
```rust
use rsshader::shader_core::*;

#[gpu]
pub struct Camera {
    pub world_to_cam: Mat4,
    pub perspective: f32,
}
#[gpu]
impl Camera {
    pub fn world_to_screen_pos(&self, pos: Vec3) -> Vec4 {
        let cam_pos = self.world_to_cam.multiply_pos(pos);
        vec4((cam_pos, 1.0.lerp(cam_pos.z, self.perspective)))
    }
}
```
