use proc_macro::TokenStream as TokenStream1;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Error, Ident, Token};

pub fn render_pipeline(input: TokenStream1) -> TokenStream1 {
    let Input {
        vertex_fn,
        fragment_fn,
    } = parse_macro_input!(input as Input);

    quote! {
        const {
            fn validate_render_pipeline() {
                fn validate_render_pipeline_helper<V: rsshader::GPUType, F: rsshader::GPUType, O: rsshader::FragmentFnOutput>(
                    _vertex_fn: fn(V) -> F,
                    _fragment_fn: fn(F) -> O,
                ) {}

                validate_render_pipeline_helper(
                    *#vertex_fn,
                    *#fragment_fn,
                );
            }

            unsafe { rsshader::RenderPipeline::new_unchecked("haha funny wgsl its bad anyway") }
        }
    }
    .into()
}

struct Input {
    vertex_fn: Ident,
    fragment_fn: Ident,
}
impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vertex_fn = input
            .parse()
            .map_err(|err| Error::new(err.span(), "expected the vertex fn's ident"))?;

        input.parse::<Token![,]>()?;

        let fragment_fn = input
            .parse()
            .map_err(|err| Error::new(err.span(), "expected the fragment fn's ident"))?;

        Ok(Self {
            vertex_fn,
            fragment_fn,
        })
    }
}
