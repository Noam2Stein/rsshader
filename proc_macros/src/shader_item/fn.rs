use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemFn, Signature, spanned::Spanned};

use crate::shader_item::util::Labels;

pub fn shader_item(item: ItemFn, errors: &mut Vec<Error>, labels: &mut Labels) -> TokenStream {
    if item.sig.generics.params.len() > 0 || item.sig.generics.where_clause.is_some() {
        errors.push(Error::new(
            item.sig.span(),
            "generic functions are not supported yet",
        ));
        return quote! { #item };
    }

    labels.find("vertex");
    labels.find("fragment");

    let ItemFn {
        vis,
        sig: Signature { ident, .. },
        ..
    } = &item;

    quote! {
        #item

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #vis struct #ident {}

        impl rsshader::reflection::ShaderFn for #ident {
            const IR: rsshader::ir::FnIr = todo!();
        }
    }
}
