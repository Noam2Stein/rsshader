use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Ident, Token, parse::Parser, punctuated::Punctuated};

#[derive(Debug, Clone)]
pub struct Labels {
    used: Vec<Ident>,
    unused: Vec<Ident>,
    duplicates: Vec<Ident>,
}

impl Labels {
    pub fn new(meta: TokenStream) -> syn::Result<Self> {
        let labels = Punctuated::<Ident, Token![,]>::parse_terminated.parse2(meta)?;

        let mut unused = Vec::with_capacity(labels.len());
        let mut duplicates = Vec::new();

        for label in labels {
            if unused.contains(&label) {
                duplicates.push(label);
            } else {
                unused.push(label);
            }
        }

        Ok(Self {
            used: Vec::with_capacity(unused.len()),
            unused,
            duplicates,
        })
    }

    pub fn find(&mut self, label: &str) -> Option<&Ident> {
        if self.used.iter().any(|label2| label2 == label) {
            return Some(self.used.iter().find(|&label2| label2 == label).unwrap());
        }

        if let Some(index) = self.unused.iter().position(|label2| label2 == label) {
            self.used.push(self.unused.remove(index));

            Some(self.used.last().unwrap())
        } else {
            None
        }
    }

    pub fn errors(self) -> TokenStream {
        let mut errors = TokenStream::new();

        for unused in self.unused {
            errors.extend(quote_spanned! { unused.span() => compile_error!("unused label"); });
        }

        for duplicate in self.duplicates {
            errors
                .extend(quote_spanned! { duplicate.span() => compile_error!("duplicate label"); });
        }

        errors
    }
}
