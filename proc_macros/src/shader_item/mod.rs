use quote::quote;
use syn::{Item, parse_macro_input};

use crate::shader_item::labels::Labels;

mod labels;
mod r#struct;

pub fn shader_item(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);

    let mut labels = match Labels::new(attr.into()) {
        Ok(labels) => labels,
        Err(error) => {
            let error = error.to_compile_error();
            return quote! { #item #error }.into();
        }
    };

    let item_output = match item {
        Item::Struct(item) => r#struct::shader_item(item, &mut labels),
        Item::Const(item) => {
            quote! { #item compile_error!("constants do not need to be annotated with #[shader_item]"); }
        }
        Item::Use(item) => {
            quote! { #item compile_error!("use statements do not need to be annotated with #[shader_item]"); }
        }
        _ => quote! { #item compile_error!("unsupported item type"); },
    };

    let labels_errors = labels.errors();

    quote! { #item_output #labels_errors }.into()
}
