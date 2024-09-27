#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod shader {
    use rsshader::{gpufn, shader_core::*, RenderPipeline};
    pub struct Vertex {
        pub pos: Vec2,
        pub color: Vec4,
    }
    unsafe impl rsshader::constructs::GPUType for Vertex {
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
            f.write_fmt(format_args!("\t{0}", "pos :"))?;
            <Vec2 as rsshader::constructs::GPUType>::wgsl_ident(f)?;
            f.write_fmt(format_args!(",\n"))?;
            f.write_fmt(format_args!("\t{0}", "color :"))?;
            <Vec4 as rsshader::constructs::GPUType>::wgsl_ident(f)?;
            f.write_fmt(format_args!(",\n"))?;
            f.write_fmt(format_args!("}}"))
        }
    }
    unsafe impl rsshader::constructs::Vertex for Vertex {}
    pub struct Fragment {
        #[allow(dead_code)]
        pub pos: Vec4,
        pub color: Vec4,
    }
    unsafe impl rsshader::constructs::GPUType for Fragment {
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
            f.write_fmt(format_args!("\t{0}", "@ builtin(position) pos :"))?;
            <Vec4 as rsshader::constructs::GPUType>::wgsl_ident(f)?;
            f.write_fmt(format_args!(",\n"))?;
            f.write_fmt(format_args!("\t{0}", "color :"))?;
            <Vec4 as rsshader::constructs::GPUType>::wgsl_ident(f)?;
            f.write_fmt(format_args!(",\n"))?;
            f.write_fmt(format_args!("}}"))
        }
    }
    unsafe impl rsshader::constructs::Fragment for Fragment {
        fn pos(&self) -> rsshader::shader_core::Vec4 {
            self.pos
        }
    }
    #[allow(unused)]
    pub fn vs_main(input: Vertex) -> Fragment {
        <Fragment as rsshader::constructs::GPUType>::validate();
        <Vertex as rsshader::constructs::GPUType>::validate();
        let f: i32 = 0;
        Fragment {
            pos: Vec4 {
                x: input.pos.x,
                y: input.pos.y,
                z: 0.0,
                w: 1.1,
            },
            color: input.color,
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct vs_main_GPUFn {}
    unsafe impl rsshader::constructs::GPUFn for vs_main_GPUFn {
        fn wgsl_ident(f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut hasher = std::hash::DefaultHasher::new();
            <std::any::TypeId as std::hash::Hash>::hash(
                &std::any::TypeId::of::<Self>(),
                &mut hasher,
            );
            f.write_fmt(
                format_args!(
                    "fn___{0}",
                    <std::hash::DefaultHasher as std::hash::Hasher>::finish(&hasher),
                ),
            )
        }
        fn wgsl_declaration(f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_fmt(format_args!("fn "))?;
            Self::wgsl_ident(f)?;
            f.write_fmt(format_args!("() {{\n"))?;
            f.write_fmt(format_args!("var {0}: ", "f"))?;
            <i32 as rsshader::constructs::GPUType>::wgsl_ident(f)?;
            f.write_fmt(format_args!(";\n"))?;
            f.write_fmt(format_args!("}}\n"))
        }
    }
    unsafe impl rsshader::constructs::VertexFn for vs_main_GPUFn {
        type I = Vertex;
        type O = Fragment;
        fn invoke(input: Vertex) -> Fragment {
            vs_main(input)
        }
    }
    #[allow(unused)]
    pub fn fs_main(input: Fragment) -> Vec4 {
        <Vec4 as rsshader::constructs::GPUType>::validate();
        <Fragment as rsshader::constructs::GPUType>::validate();
        input.color
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub struct fs_main_GPUFn {}
    unsafe impl rsshader::constructs::GPUFn for fs_main_GPUFn {
        fn wgsl_ident(f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut hasher = std::hash::DefaultHasher::new();
            <std::any::TypeId as std::hash::Hash>::hash(
                &std::any::TypeId::of::<Self>(),
                &mut hasher,
            );
            f.write_fmt(
                format_args!(
                    "fn___{0}",
                    <std::hash::DefaultHasher as std::hash::Hasher>::finish(&hasher),
                ),
            )
        }
        fn wgsl_declaration(f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_fmt(format_args!("fn "))?;
            Self::wgsl_ident(f)?;
            f.write_fmt(format_args!("() {{\n"))?;
            f.write_fmt(format_args!("}}\n"))
        }
    }
    unsafe impl rsshader::constructs::FragmentFn for fs_main_GPUFn {
        type I = Fragment;
        fn invoke(input: Fragment) -> rsshader::shader_core::Vec4 {
            fs_main(input)
        }
    }
    pub const HELLO_TRIANGLE: RenderPipeline<Vertex> = RenderPipeline::new::<
        vs_main_GPUFn,
        fs_main_GPUFn,
    >();
}
fn main() {
    {
        ::std::io::_print(format_args!("{0}\n", shader::HELLO_TRIANGLE.wgsl()));
    };
}
