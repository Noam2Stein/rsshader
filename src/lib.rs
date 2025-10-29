#![no_std]

pub mod ir;
pub mod lang;
pub mod reflection;

pub use rsshader_macros::shader_item;

#[doc(hidden)]
pub use rsshader_macros;
