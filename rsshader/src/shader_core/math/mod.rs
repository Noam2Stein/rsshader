use super::*;

pub use ggmath::primitive_aliases::{bool::*, f32::*, i32::*, u32::*};

use ggmath::{scalar::*, vector::*};

unsafe impl GPUType for u32 {}
unsafe impl GPUType for i32 {}
unsafe impl GPUType for f32 {}

unsafe impl GPULerp for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}

unsafe impl<T: Scalar + GPUType> GPUType for Vec2<T> {}
unsafe impl<T: Scalar + GPUType> GPUType for Vec3<T> {}
unsafe impl<T: Scalar + GPUType> GPUType for Vec4<T> {}

unsafe impl<T: Scalar + GPULerp> GPULerp for Vec2<T> {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::from_array([self[0].lerp(&other[0], t), self[1].lerp(&other[1], t)])
    }
}
unsafe impl<T: Scalar + GPULerp> GPULerp for Vec3<T> {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::from_array([
            self[0].lerp(&other[0], t),
            self[1].lerp(&other[1], t),
            self[2].lerp(&other[2], t),
        ])
    }
}
unsafe impl<T: Scalar + GPULerp> GPULerp for Vec4<T> {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::from_array([
            self[0].lerp(&other[0], t),
            self[1].lerp(&other[1], t),
            self[2].lerp(&other[2], t),
            self[3].lerp(&other[3], t),
        ])
    }
}
