use quote::{quote, quote_spanned};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, Ident, Item, Meta, Type};

#[proc_macro_attribute]
pub fn gpu(attrib: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attrib = parse_macro_input!(attrib with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let item = parse_macro_input!(item as Item);

    let mut inserts = Vec::new();
    
    let addition = match attrib.first().map_or(None, |first| first.path().get_ident()) {
        spec => {
            if let Some(invalid_spec) = spec {
                inserts.push(
                    {
                        let message = format!("'{invalid_spec}' is not a gpu specification");
                        quote_spanned! {
                            spec.span() =>
                            compile_error!(#message);
                        }
                    }  
                );
            }
            match &item {
                Item::Struct(item) => {
                    let ident = &item.ident;
                    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    
                    let field_idents = item.fields.iter().map(|field| &field.ident).collect::<Box<[&Option<Ident>]>>();
                    let field_types = item.fields.iter().map(|field| &field.ty).collect::<Box<[&Type]>>();
                
                    quote! {
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
                                    write!(f, "\t{}: ", stringify!(#field_idents))?;
                                    <#field_types as rsshader::constructs::GPUType>::wgsl_ident(f)?;
                                    writeln!(f, ",")?;
                                )*
                
                                write!(f, "}}")
                            }
                        }
                        impl #impl_generics rsshader::constructs::GPUStruct for #ident #ty_generics #where_clause {
                
                        }
                    }
                }
                _ => quote_spanned! {
                    attrib.span() =>
                    compile_error!("this item type can't be used as a gpu item");
                }
            }
        }
    };

    quote! {
        #item
        #addition
        #(
            #inserts
        )*
    }.into()
}