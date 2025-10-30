use std::ops::Add;

use rsshader::shader_item;

fn main() {
    println!("Hello, world!");
}

#[shader_item(vector)]
struct Vector<const N: usize, T>([T; N]);

#[shader_item]
impl<const N: usize, T> Vector<N, T> {
    #[builtin(new_vec)]
    pub fn new(array: [T; N]) -> Self {
        Self(array)
    }

    #[builtin(new_vec)]
    pub fn splat(value: T) -> Self {
        Self([value; N])
    }
}

#[shader_item]
impl<const N: usize, T: Add> Add for Vector<N, T> {
    type Output = Vector<N, T::Output>;

    #[builtin(add)]
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.0.into_iter().zip(rhs.0.into_iter()))
    }
}
