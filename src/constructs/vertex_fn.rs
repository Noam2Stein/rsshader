use super::*;

pub unsafe trait GPUVertexFn: GPUFn<Output: GPUFragment> {
    type Input: GPUVertex;

    fn validate() {}
}
