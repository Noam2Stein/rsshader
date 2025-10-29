#![no_std]

pub mod ir;
pub mod lang;

pub use rsshader_macros::shader_item;

#[doc(hidden)]
pub mod reflection;
#[doc(hidden)]
pub use rsshader_macros;
