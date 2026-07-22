# `rsshader`

This crate, which ended up not being implemented, would have let you write
shaders directly in Rust then translate them into any shader language at compile
time.

```rust
use ggmath::{Vec2, Vec4};
use rsshader::{shader_item, wgsl_shader};

fn main() {
  println!("{SHADER}");
}

const SHADER: &str = wgsl_shader!(vs_main, fs_main);

#[shader_item(vertex)]
struct Vertex {
  position: Vec2<f32>,
}

#[shader_item(fragment)]
struct Fragment {
  #[shader(position)]
  position: Vec4<f32>,
  color: Vec2<f32>,
}

#[shader_item(vertex)]
fn vs_main(vertex: Vertex) -> Fragment {
  Fragment {
    position: vertex.position.extend(0.0).extend(1.0),
    color: vertex.position,
  }
}

#[shader_item(vertex)]
fn fs_main(fragment: Fragment) -> Vec4 {
  fragment.color.extend(0.0).extend(1.0)
}
```

Notice how these shader items are fully separate, but still reference each
other. This crate's macros do not get the full context in their token stream.
This means that the majority of logic, including linking items and translating
into shader languages, must be performed through the type system and in const
contexts. The crate would have also fully supported generics.

These requirements make this crate insanely difficult to implement. Every tiny
feature requires finding the perfect trick in both the type system and const
contexts, and to invoke it perfectly all from macros. But I guarantee this is
all possible on 2025 stable Rust.

Of course, this crate would have a bunch of usability problems, that *are*
solved by `rust-gpu`:

- What if you do not want to include the final shader code in your binary? (this
  is common when you have a huge amount of shaders.)

- Third party crate support requires them to support `rsshader` directly.

- Hilariously bad compile times, as we are performing in insane amount of logic
  in const blocks, including translating into the shader language.

If I had an infinite amount of time, I would implement this crate just for the
challange. I previously had tiny versions of it working, but not nearly the full
thing. Implementing this crate would be a cool challange if you are bored.
