use super::*;

pub unsafe trait Fragment: GPUType {
    fn pos(&self) -> Vec4;

    fn validate() {}
}
