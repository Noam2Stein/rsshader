use std::fmt::{self, Display, Formatter};

use crate::constructs::*;

pub trait Element: GPUType {

}
macro_rules! element_ty {
    ($ty:ty, $wgsl_ident:literal) => {
        impl GPUType for $ty {
            fn wgsl_ident(f: &mut Formatter) -> fmt::Result {
                $wgsl_ident.fmt(f)
            }
            fn wgsl_declaration(_f: &mut Formatter) -> fmt::Result {
                Ok(())
            }
        }
        impl Element for $ty {

        }       
    };
}
element_ty!(f32, "f32");
element_ty!(f64, "f64");
element_ty!(u8, "u8");
element_ty!(u16, "u16");
element_ty!(u32, "u32");
element_ty!(u64, "u64");
element_ty!(u128, "u128");
element_ty!(i8, "i8");
element_ty!(i16, "i16");
element_ty!(i32, "i32");
element_ty!(i64, "i64");
element_ty!(i128, "i128");

macro_rules! vec_ty {
    ($ident:ident($($component:ident), +), $wgsl_ident:literal) => {
        pub struct $ident<T: Element = f32> {
            $(
                pub $component: T,
            )+
        }
        impl<T: Element> GPUType for $ident<T> {
            fn wgsl_ident(f: &mut Formatter) -> fmt::Result {
                write!(f, "{}<", $wgsl_ident)?;
                T::wgsl_ident(f)?;
                write!(f, ">")
            }
            fn wgsl_declaration(_f: &mut Formatter) -> fmt::Result {
                Ok(())
            }
        }
    };
}
vec_ty!(Vec2(x, y), "vec2");
vec_ty!(Vec3(x, y, z), "vec3");
vec_ty!(Vec4(x, y, z, w), "vec4");