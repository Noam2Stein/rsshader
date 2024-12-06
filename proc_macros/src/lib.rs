use proc_macro::TokenStream as TokenStream1;
use syn::Ident;

mod gpu;

#[inline(always)]
#[proc_macro_attribute]
pub fn gpu(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    gpu::gpu(input_attrib, input_item)
}

fn get_expr_desc_item_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("GPU_EXPR_{ident}"), ident.span())
}
