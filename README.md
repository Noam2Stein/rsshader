*** everything here is unfinished and not ready for development, and anything mentioned in the README may not be implemented yet. ***

# rsshader

turns a sub-section of Rust into shaders in format like spirv and wgsl. a safe, productive, and performant way to create shaders.

```rust
use rsshader::*;

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
    println!("{}", HELLO_TRIANGLE.format::<Wgsl>().as_str())
}
```
## features

### compilation-time validation

all gpu items are validated at compile time.
```rust
#[gpu]
struct FoolishStruct {
    yihihiha: String, // ERROR: the trait bound `String: GPUType` is not satisfied
}
```

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

### WGPU integration

the 'wgpu' feature adds integration for wgpu to execute shaders safely.

### 0-cost abstraction

the generated output is identical to shaders directly written in the output formats.

### future-optimization proof API

with current stable rust alot of the translation process has to be done at runtime because of missing ```const fn``` features. the API is designed so that when the needed features are stablized, the crate would translate at compile-time.
