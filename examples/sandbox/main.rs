use rsshader::constructs::GPUType;

use utils::*;

mod shader;
use shader::*;

fn main() {
    println!("{}", fn_display(Vertex::wgsl_declaration));
    println!("{}", fn_display(Fragment::wgsl_declaration));
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