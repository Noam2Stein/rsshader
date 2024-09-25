use quote::ToTokens;
use syn::{parse2, FnArg, ItemFn, ReturnType};

use super::*;

pub enum PipelineStage {
    Vertex,
    Fragment,
}

pub enum Output {
    Struct {
        input: ItemStruct,
        vertex_impl: Option<()>,
        fragment_impl: Option<usize>,
    },
    Fn {
        input: ItemFn,
        pipeline_stage: Option<PipelineStage>,
    },
    Err(TokenStream),
}
impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct { input, vertex_impl, fragment_impl } => {
                input.to_tokens(tokens);

                let ident = &input.ident;
                let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
            
                let field_idents = input.fields.iter().map(|field| &field.ident).collect::<Box<[&Option<Ident>]>>();
                let field_types = input.fields.iter().map(|field| &field.ty).collect::<Box<[&Type]>>();
                let field_fragment_pos_attrs = (0..input.fields.len()).map(|i|
                    if Some(i) == *fragment_impl {
                        Some(
                            quote! { @builtin(position) }
                        )
                    }
                    else {
                        None
                    }
                );

                tokens.extend(quote! {
                    impl #impl_generics rsshader::constructs::GPUType for #ident #ty_generics #where_clause {
                        fn wgsl_ident(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            let mut hasher = std::hash::DefaultHasher::new();
                            <std::any::TypeId as std::hash::Hash>::hash(&std::any::TypeId::of::<Self>(), &mut hasher);
                            write!(f, "Type___{}", <std::hash::DefaultHasher as std::hash::Hasher>::finish(&hasher))
                        }
                        fn wgsl_declaration(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            write!(f, "struct ")?;
                            Self::wgsl_ident(f)?;
                            writeln!(f, " {{")?;
                            #(
                                write!(f, "\t{}", stringify!(#field_fragment_pos_attrs #field_idents: ))?;
                                <#field_types as rsshader::constructs::GPUType>::wgsl_ident(f)?;
                                writeln!(f, ",")?;
                            )*
            
                            write!(f, "}}")
                        }
                    }
                });

                if let Some(_) = vertex_impl {
                    tokens.extend(
                        quote! {
                            impl #impl_generics rsshader::constructs::Vertex for #ident #ty_generics #where_clause {

                            }
                        }
                    );
                }
                if let Some(_) = fragment_impl {
                    tokens.extend(
                        quote! {
                            impl #impl_generics rsshader::constructs::Fragment for #ident #ty_generics #where_clause {

                            }
                        }
                    );
                }
            },
            Self::Fn { input, pipeline_stage } => {
                if let Some(pipeline_stage) = pipeline_stage {
                    let mut fun = input.clone();

                    fun.block.stmts.insert(0, parse2(
                    if input.sig.inputs.len() == 1 {
                        let arg = input.sig.inputs.first().unwrap();
                                match arg {
                                    FnArg::Typed(ty) => {
                                        let ty = &ty.ty;
                                        match pipeline_stage {
                                            PipelineStage::Vertex => quote! {
                                                <#ty as rsshader::constructs::Vertex>::validate();
                                            },
                                            PipelineStage::Fragment => quote! {
                                                <#ty as rsshader::constructs::Fragment>::validate();
                                            }
                                        }
                                    },
                                    FnArg::Receiver(_) => {
                                        quote! {
                                            compile_error!("a pipeline fn has to have 1 argument");
                                        }
                                    }
                                }
                            }
                            else {
                                quote! {
                                    compile_error!("a pipeline fn has to have 1 argument");
                                }
                            }
                    ).unwrap());

                    fun.block.stmts.insert(0,
                        parse2(
                            {
                                let ty: Type = match &fun.sig.output {
                                    ReturnType::Type(_, ty) => *(*ty).clone(),
                                    ReturnType::Default => parse2(quote! { () }).unwrap(),
                                };
                                match pipeline_stage {
                                    PipelineStage::Vertex => quote! {
                                        <#ty as rsshader::constructs::Fragment>::validate();
                                    },
                                    PipelineStage::Fragment => quote! {
                                        fn validate_correct_output(x: #ty) -> rsshader::shader_core::Vec4 {
                                            x
                                        }
                                    }
                                }
                            }
                        ).unwrap()
                    );

                    fun.to_tokens(tokens);
                }
                else {
                    input.to_tokens(tokens);
                }
            },
            Self::Err(output) => output.to_tokens(tokens),
        }
    }
}