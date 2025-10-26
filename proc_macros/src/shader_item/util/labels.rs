use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    Attribute, Error, Ident, Token,
    parse::{Parse, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
};

#[derive(Debug)]
pub struct Labels {
    labels: HashMap<String, LabelInfo>,
}

#[derive(Debug)]
struct LabelInfo {
    span: Span,
    is_used: bool,
}

impl Labels {
    pub fn from_shader_item(meta: TokenStream, errors: &mut Vec<Error>) -> Self {
        let meta_span = meta.span();

        let Ok(label_idents) = Punctuated::<Ident, Token![,]>::parse_terminated.parse2(meta) else {
            errors.push(Error::new(meta_span, "expected a list of identifiers"));
            return Self {
                labels: HashMap::new(),
            };
        };

        let mut labels = HashMap::with_capacity(label_idents.len());

        for label in label_idents {
            let label_str = label.to_string();

            if labels.contains_key(&label_str) {
                errors.push(Error::new(label.span(), "duplicate attribute"));
            } else {
                labels.insert(
                    label_str,
                    LabelInfo {
                        span: label.span(),
                        is_used: false,
                    },
                );
            }
        }

        Self { labels }
    }

    pub fn from_attributes(attrs: &mut Vec<Attribute>, errors: &mut Vec<Error>) -> Self {
        let mut labels = HashMap::with_capacity(attrs.len());

        attrs.retain(|attr| {
            let Ok(label) = Ident::parse.parse2(attr.meta.to_token_stream()) else {
                return true;
            };

            let label_str = label.to_string();

            match label_str.as_str() {
                "position" => {}
                _ => return true,
            }

            if labels.contains_key(&label_str) {
                errors.push(Error::new(label.span(), "duplicate attribute"));
            } else {
                labels.insert(
                    label_str,
                    LabelInfo {
                        span: label.span(),
                        is_used: false,
                    },
                );
            }

            false
        });

        Self { labels }
    }

    pub fn find(&mut self, label: &str) -> Option<Span> {
        if let Some(label) = self.labels.get_mut(label) {
            label.is_used = true;

            Some(label.span)
        } else {
            None
        }
    }

    pub fn finish(self, errors: &mut Vec<Error>) {
        for label in self.labels.values() {
            if !label.is_used {
                errors.push(Error::new(label.span, "unused label"));
            }
        }
    }
}
