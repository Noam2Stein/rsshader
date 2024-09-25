#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use constructs::GPUType;
use rsshader::{shader_core::*, *};
use utils::*;
struct Vertex<T: GPUType> {
    f: T,
    pos: Vec2,
    color: Vec4,
}
impl<T: GPUType> rsshader::constructs::GPUType for Vertex<T> {
    fn wgsl_ident(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut hasher = std::hash::DefaultHasher::new();
        <std::any::TypeId as std::hash::Hash>::hash(
            &std::any::TypeId::of::<Self>(),
            &mut hasher,
        );
        f.write_fmt(
            format_args!(
                "Type___{0}",
                <std::hash::DefaultHasher as std::hash::Hasher>::finish(&hasher),
            ),
        )
    }
    fn wgsl_declaration(f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("struct "))?;
        Self::wgsl_ident(f)?;
        f.write_fmt(format_args!(" {{\n"))?;
        (/*ERROR*/)
    }
}
fn main() {
    {
        ::std::io::_print(
            format_args!("{0}\n", fn_display(Vertex::<u8>::wgsl_declaration)),
        );
    };
    {
        ::std::io::_print(
            format_args!("{0}\n", fn_display(Vertex::<u32>::wgsl_declaration)),
        );
    };
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
    pub fn fn_display(f: impl Fn(&mut Formatter) -> fmt::Result) -> impl Display {
        FnDisplay { f }
    }
}
