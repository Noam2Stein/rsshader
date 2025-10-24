use quote::quote;
use syn::{Item, parse_macro_input};

mod r#struct;

pub fn shader_item(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);

    match item {
        Item::Struct(item) => r#struct::shader_item(item).into(),
        _ => quote! { compile_error!("Unsupported item type") }.into(),
    }
}
