use quote::quote;
use syn::{Item, parse_macro_input};

mod r#struct;

pub fn shader_item(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);

    match item {
        Item::Struct(item) => r#struct::shader_item(item),
        Item::Const(item) => {
            quote! { #item compile_error!("constants do not need to be annotated with #[shader_item]"); }
        }
        Item::Use(item) => {
            quote! { #item compile_error!("use statements do not need to be annotated with #[shader_item]"); }
        }
        _ => quote! { #item compile_error!("unsupported item type"); },
    }
    .into()
}
