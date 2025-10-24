mod const_eq;
mod shader;
mod shader_item;

#[proc_macro_derive(ConstEq)]
pub fn const_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const_eq::const_eq(input)
}

#[proc_macro]
pub fn shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    shader::shader(input)
}

#[proc_macro_attribute]
pub fn shader_item(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    shader_item::shader_item(attr, item)
}
