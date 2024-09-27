use quote::quote_spanned;
use syn::{parse2, spanned::Spanned, Ident, Local, Pat, Stmt};

pub fn validation_stmt(input: &Local) -> Option<Stmt> {
    let ident: &Ident = match &input.pat {
        Pat::Ident(pat) => &pat.ident,
        Pat::Type(pat) => match &*pat.pat {
            Pat::Ident(pat) => &pat.ident,
            pat => {
                return Some(
                    parse2(quote_spanned! {
                        pat.span() =>
                        compile_error!("expected an ident")
                    })
                    .unwrap(),
                )
            }
        },
        pat => {
            return Some(
                parse2(quote_spanned! {
                    pat.span() =>
                    compile_error!("expected an ident")
                })
                .unwrap(),
            )
        }
    };

    Some(
        parse2(quote_spanned! {
            ident.span() =>
            {
                fn validate_let<T: rsshader::constructs::GPUType>(_x: &T) {}

                validate_let(&#ident)
            };
        })
        .unwrap(),
    )
}
