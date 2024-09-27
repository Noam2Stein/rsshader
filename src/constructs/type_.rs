pub unsafe trait GPUType: 'static {
    fn validate() {}
}
unsafe impl GPUType for () {}
