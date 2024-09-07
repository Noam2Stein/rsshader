# RS Shader

A shading-language tightly-coupled with rust and its module system.

* RS-Shader is translated to WGSL which means its cross platform and efficient.

## Syntax

RS-Shader has Rust-like syntax with changes made to fit shader development.

## Module System

RS-Shaders are saved in rssh files and are included as a "shader module" in rust code.

example:

shader_utils.rssh:

pub fn add(a: f32, b: f32) -> f32 {
  a + b
}

mod.rs:

shader_mod!(pub shader_utils)

then shader_utils can be imported in other rssh files through their relative path as a rust module, and some rssh items are exposed to rust code (such as pipelines and vertex structs).
