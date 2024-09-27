use super::*;

pub unsafe trait VertexFn: GPUFn {
    type I: Vertex;
    type O: Fragment;

    fn invoke(input: Self::I) -> Self::O;

    fn validate() {}
}
