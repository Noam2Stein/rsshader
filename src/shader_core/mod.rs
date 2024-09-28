use crate::constructs::*;

pub mod pipeline;
pub use pipeline::*;

pub use rsshader_proc_macros::{gpu, gpufn};

pub trait Element: GPUType + Copy + PartialEq + PartialOrd + Default {}
macro_rules! element_ty {
    ($ty:ty, $wgsl_ident:literal) => {
        unsafe impl GPUType for $ty {}
        impl Element for $ty {}
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

unsafe impl GPULerp for f32 {
    #[inline(always)]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}
unsafe impl GPULerp for f64 {
    #[inline(always)]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self * (1.0 - t) as f64 + other * t as f64
    }
}

macro_rules! vec_ty {
    ($ident:ident($($component:ident), +), $wgsl_ident:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $ident<T: Element = f32> {
            $(
                pub $component: T,
            )+
        }
        unsafe impl<T: Element> GPUType for $ident<T> {}

        unsafe impl<T: Element + GPULerp> GPULerp for $ident<T> {
            #[inline(always)]
            fn lerp(&self, other: &Self, t: f32) -> Self {
                Self {
                    $(
                        $component: self.$component.lerp(&other.$component, t),
                    )+
                }
            }
        }
    };
}
vec_ty!(Vec2(x, y), "vec2");
vec_ty!(Vec3(x, y, z), "vec3");
vec_ty!(Vec4(x, y, z, w), "vec4");

pub unsafe trait IntoVec<V> {
    fn into_vec(self) -> V;
}
unsafe impl<V> IntoVec<V> for V {
    fn into_vec(self) -> V {
        self
    }
}

#[inline(always)]
pub fn vec2<T: Element>(value: impl IntoVec<Vec2<T>>) -> Vec2<T> {
    value.into_vec()
}
#[inline(always)]
pub fn vec3<T: Element>(value: impl IntoVec<Vec3<T>>) -> Vec3<T> {
    value.into_vec()
}
#[inline(always)]
pub fn vec4<T: Element>(value: impl IntoVec<Vec4<T>>) -> Vec4<T> {
    value.into_vec()
}

unsafe impl<T: Element> IntoVec<Vec2<T>> for (T, T) {
    fn into_vec(self) -> Vec2<T> {
        Vec2 {
            x: self.0,
            y: self.1,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec3<T>> for (T, T, T) {
    fn into_vec(self) -> Vec3<T> {
        Vec3 {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec3<T>> for (Vec2<T>, T) {
    fn into_vec(self) -> Vec3<T> {
        Vec3 {
            x: self.0.x,
            y: self.0.y,
            z: self.1,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec3<T>> for (T, Vec2<T>) {
    fn into_vec(self) -> Vec3<T> {
        Vec3 {
            x: self.0,
            y: self.1.x,
            z: self.1.y,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (T, T, T, T) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0,
            y: self.1,
            z: self.2,
            w: self.3,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (Vec2<T>, T, T) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.1,
            w: self.2,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (T, Vec2<T>, T) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0,
            y: self.1.x,
            z: self.1.y,
            w: self.2,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (T, T, Vec2<T>) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0,
            y: self.1,
            z: self.2.x,
            w: self.2.y,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (Vec3<T>, T) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.0.z,
            w: self.1,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (T, Vec3<T>) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0,
            y: self.1.x,
            z: self.1.y,
            w: self.1.z,
        }
    }
}
unsafe impl<T: Element> IntoVec<Vec4<T>> for (Vec2<T>, Vec2<T>) {
    fn into_vec(self) -> Vec4<T> {
        Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.1.x,
            w: self.1.y,
        }
    }
}
