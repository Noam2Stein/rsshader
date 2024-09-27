use super::*;

pub unsafe trait VertexFn: GPUFn<Output: Fragment> {
    type Input: Vertex;

    fn validate() {}
}
