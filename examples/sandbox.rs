use constructs::GPUType;
use rsshader::{shader_core::*, *};

use utils::*;

#[gpu]
struct Vertex<T: GPUType> {
    f: T,
    pos: Vec2,
    color: Vec4,
}

fn main() {
    println!("{}", fn_display(Vertex::<u8>::wgsl_declaration));
    println!("{}", fn_display(Vertex::<u32>::wgsl_declaration));
}

mod utils {
    use std::fmt::{self, Display, Formatter};

    struct FnDisplay<F: Fn(&mut Formatter) -> fmt::Result> {
        f: F,
    }
    impl<F: Fn(&mut Formatter) -> fmt::Result> Display for FnDisplay<F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            (&self.f)(f)
        }
    }
    pub fn fn_display<>(f: impl Fn(&mut Formatter) -> fmt::Result) -> impl Display {
        FnDisplay {
            f
        }
    }
}