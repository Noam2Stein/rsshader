use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, Ident, Item, ItemStruct, Meta, Type};

mod output;
use output::*;

#[proc_macro_attribute]
pub fn gpu(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);

    let mut output = match parse_macro_input!(item as Item) {
        Item::Struct(input) => Output::Struct { input, vertex_impl: None, fragment_impl: None },
        Item::Fn(input) => Output::Fn { input, pipeline_stage: None },
        _ => Output::Err(
            quote_spanned! {
                attr.span() =>
                compile_error!("this item type can't be used as a gpu item");
            }
        )
    };

    let mut errs = Vec::new();
    
    for spec in &attr {
        match spec {
            Meta::Path(spec) => match spec.get_ident() {
                Some(spec) => match spec.to_string().as_str() {
                    "vertex" => match &mut output {
                        Output::Struct { input: _, vertex_impl, fragment_impl: _ } => {
                            if let Some(_) = vertex_impl {
                                errs.push(
                                    quote_spanned! {
                                        spec.span() =>
                                        compile_error!("duplicate spec");
                                    }
                                );
                            }
                            else {
                                *vertex_impl = Some(());
                            }
                        }
                        _ => errs.push(
                            quote_spanned! {
                                attr.span() =>
                                compile_error!("this item type can't be used as a gpu(vertex) item");
                            }
                        ),
                    },
                    "fragment" => match &mut output {
                        Output::Struct { input, vertex_impl: _, fragment_impl } => {
                            let mut field_index = 0;
                            for field in &mut input.fields {
                                field.attrs.retain(|attr|
                                    if attr.path().require_ident().map_or(false, |ident|
                                        ident.to_string() == "fragment_pos"
                                    ) {
                                        if let Some(_) = fragment_impl {
                                            errs.push(
                                                quote_spanned! {
                                                    spec.span() =>
                                                    compile_error!("expected 1 field to be marked #[fragment_pos]");
                                                }
                                            );
                                        }
                                        else {
                                            *fragment_impl = Some(field_index);
                                        }

                                        false
                                    }
                                    else {
                                        true
                                    }
                                );

                                field_index += 1;
                            }

                            if *fragment_impl == None {
                                errs.push(
                                    quote_spanned! {
                                        spec.span() =>
                                        compile_error!("expected 1 field to be marked #[fragment_pos]");
                                    }
                                );

                                *fragment_impl = Some(0);
                            }
                        }
                        _ => errs.push(
                            quote_spanned! {
                                attr.span() =>
                                compile_error!("this item type can't be used as a gpu(fragment) item");
                            }
                        ),
                    },
                    "vertex_fn" => match &mut output {
                        Output::Fn { input: _, pipeline_stage } => {
                            if let Some(_) = pipeline_stage {
                                errs.push(
                                    quote_spanned! {
                                        attr.span() =>
                                        compile_error!("fn can't be a vertex_fn because it is already mapped to a pipeline stage");
                                    }
                                )
                            }
                            else {
                                *pipeline_stage = Some(PipelineStage::Vertex);
                            }
                        },
                        _ => errs.push(
                            quote_spanned! {
                                attr.span() =>
                                compile_error!("this item type can't be used as a gpu(fragment) item");
                            }
                        ),
                    },
                    "fragment_fn" => match &mut output {
                        Output::Fn { input: _, pipeline_stage } => {
                            if let Some(_) = pipeline_stage {
                                errs.push(
                                    quote_spanned! {
                                        attr.span() =>
                                        compile_error!("fn can't be a fragment_fn because it is already mapped to a pipeline stage");
                                    }
                                )
                            }
                            else {
                                *pipeline_stage = Some(PipelineStage::Fragment);
                            }
                        },
                        _ => errs.push(
                            quote_spanned! {
                                attr.span() =>
                                compile_error!("this item type can't be used as a gpu(fragment) item");
                            }
                        ),
                    },
                    _ => {
                        let message = format!("'{spec}' is not a gpu specification");
                        errs.push(
                            quote_spanned! {
                                spec.span() =>
                                compile_error!(#message);
                            }
                        );
                    }
                },
                None => errs.push(
                    quote_spanned! {
                        spec.span() =>
                        compile_error!("expected an ident");
                    }
                ),
            },
            Meta::List(spec) => match spec.path.get_ident() {
                Some(spec) => match spec.to_string().as_str() {
                    _ => errs.push(
                        {
                            let message = format!("'{spec}' is not a gpu specification");
                            quote_spanned! {
                                spec.span() =>
                                compile_error!(#message);
                            }
                        }
                    ),
                },
                None => errs.push(
                    quote_spanned! {
                        spec.span() =>
                        compile_error!("expected an ident");
                    }
                ),
            },
            Meta::NameValue(spec) => match spec.path.get_ident() {
                Some(spec) => match spec.to_string().as_str() {
                    _ => errs.push(
                        {
                            let message = format!("'{spec}' is not a gpu specification");
                            quote_spanned! {
                                spec.span() =>
                                compile_error!(#message);
                            }
                        }
                    ),
                },
                None => errs.push(
                    quote_spanned! {
                        spec.span() =>
                        compile_error!("expected an ident");
                    }
                ),
            }
        }
    }

    quote! {
        #output
        #(
            #errs
        )*
    }.into()
}