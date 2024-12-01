use proc_macro::TokenStream as TokenStream1;
use syn::Ident;

mod gpu_fn;
mod gpu_type;
mod render_pipeline;

#[inline(always)]
#[proc_macro_attribute]
pub fn gpu_type(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    gpu_type::gpu_type(input_attrib, input_item)
}
#[inline(always)]
#[proc_macro_attribute]
pub fn gpu_fn(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    gpu_fn::gpu_fn(input_attrib, input_item)
}

#[inline(always)]
#[proc_macro]
pub fn render_pipeline(input: TokenStream1) -> TokenStream1 {
    render_pipeline::render_pipeline(input)
}

fn get_fn_desc_item_ident(fn_ident: &Ident) -> Ident {
    Ident::new(&format!("GPU_FN_{fn_ident}"), fn_ident.span())
}
fn get_expr_desc_item_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("GPU_EXPR_{ident}"), ident.span())
}
