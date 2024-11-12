use proc_macro::TokenStream as TokenStream1;

mod derive_gpu_type;

#[proc_macro_derive(GPUType)]
pub fn derive_gpu_type(input: TokenStream1) -> TokenStream1 {
    derive_gpu_type::derive_gpu_type(input)
}
