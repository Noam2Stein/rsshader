use quote::ToTokens;

pub mod struct_;
pub mod fn_;

use super::*;

pub enum GPUItem {
    Struct(struct_::GPUStruct),
    Fn (fn_::GPUFn),
    Err(TokenStream),
}
impl From<Item> for GPUItem {
    fn from(value: Item) -> Self {
        match value {
            Item::Struct(value) => GPUItem::Struct(value.into()),
            Item::Fn(value) => GPUItem::Fn(value.into()),
            _ => GPUItem::Err(
                quote! {
                    attr.span() =>
                    compile_error!("this item type can't be used as a gpu item");
                }
            )
        }
    }
}
impl ToTokens for GPUItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(item) => item.to_tokens(tokens),
            Self::Fn(item) => item.to_tokens(tokens),
            Self::Err(item) => item.to_tokens(tokens),
        }
    }
}