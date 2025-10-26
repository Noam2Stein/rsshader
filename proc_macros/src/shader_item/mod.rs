use quote::quote;
use syn::{Item, parse_macro_input};

use crate::shader_item::util::Labels;

mod util;

mod r#struct;

pub fn shader_item(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);

    let mut errors = Vec::new();
    let mut labels = Labels::from_shader_item(attr.into(), &mut errors);

    let item_output = match item {
        Item::Struct(item) => r#struct::shader_item(item, &mut errors, &mut labels),
        Item::Const(item) => {
            quote! { #item compile_error!("constants do not need to be annotated with #[shader_item]"); }
        }
        Item::Use(item) => {
            quote! { #item compile_error!("use statements do not need to be annotated with #[shader_item]"); }
        }
        _ => quote! { #item compile_error!("unsupported item type"); },
    };

    labels.finish(&mut errors);

    let errors = errors.into_iter().map(|error| error.to_compile_error());

    quote! { #item_output #(#errors)* }.into()
}
