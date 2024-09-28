use super::*;

pub unsafe trait GPULerp: GPUType {
    fn lerp(&self, other: &Self, t: f32) -> Self;
}
