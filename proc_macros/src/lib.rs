use proc_macro::TokenStream as TokenStream1;

mod derive_gpu_type;
mod gpu_fn;

#[inline(always)]
#[proc_macro_derive(GPUType)]
pub fn derive_gpu_type(input: TokenStream1) -> TokenStream1 {
    derive_gpu_type::derive_gpu_type(input)
}
#[inline(always)]
#[proc_macro_attribute]
pub fn gpu_fn(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    gpu_fn::gpu_fn(input_attrib, input_item)
}
