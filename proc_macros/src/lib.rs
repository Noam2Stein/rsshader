mod const_eq;

#[proc_macro_derive(ConstEq)]
pub fn const_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const_eq::const_eq(input)
}
