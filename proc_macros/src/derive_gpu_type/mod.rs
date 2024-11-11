use syn::DeriveInput;

use super::*;

pub fn derive_gpu_type(input: TokenStream1) -> TokenStream1 {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    quote! {
        impl #generics GPUType for #ident {

        }
    }
    .into()
}
