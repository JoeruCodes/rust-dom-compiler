extern crate proc_macro;
mod parse_impl;
mod helpers;
use helpers::{capitalize_first_char, to_upper_camel_case};
use parse_impl::{Args, ArgsStruct, Items};
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};

use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, Data, DeriveInput, Error, Field, Fields, Ident, ItemEnum, ItemStruct, LitBool, LitStr, Meta, MetaNameValue, Result, Type
};
use syn::Lit;
#[proc_macro_attribute]
pub fn css_property(args: TokenStream, item: TokenStream) -> TokenStream {
    let item_n = Items::from(item);

    match item_n {
        Items::Enum(mut item_enum) => {
            let Args { sister, inherit } = parse_macro_input!(args as Args);
            let initial_ident = Ident::new("Initial", item_enum.ident.span());
            let inherit_ident = Ident::new("Inherit", item_enum.ident.span());
            let mut final_output = quote! {};
            if inherit {
                let initial_variant = syn::Variant {
                    attrs: Vec::new(),
                    ident: initial_ident.clone(),
                    fields: syn::Fields::Unit,
                    discriminant: None,
                };

                let inherit_variant = syn::Variant {
                    attrs: Vec::new(),
                    ident: inherit_ident.clone(),
                    fields: syn::Fields::Unit,
                    discriminant: None,
                };

                item_enum.variants.push(initial_variant);
                item_enum.variants.push(inherit_variant);
            }
            final_output.extend(quote! {
                #[derive(Hash, PartialEq, Eq, Debug)]
                #item_enum
            });
            for name in sister.iter() {
                let n_enum_ident = if name.starts_with("_") {
                    Ident::new(&format!("{}{}", &item_enum.ident, &name[1..]), item_enum.ident.span())
                } else if name.ends_with("_") {
                    Ident::new(&format!("{}{}", &name[..name.len() - 1], &item_enum.ident), item_enum.ident.span())
                } else {
                    Ident::new(name, item_enum.ident.span())
                };

                let variants = item_enum.variants.iter().cloned().collect::<Vec<_>>();

                final_output.extend(quote! {
                    #[derive(Hash, PartialEq, Eq, Debug)]
                    pub enum #n_enum_ident {
                        #( #variants ),*
                    }
                });
            }

            return final_output.into();
        },
        Items::Struct(mut item_struct) => {
            let mut final_output = quote! {};
            let ArgsStruct { inherit, initial_inherit_wrap } = parse_macro_input!(args as ArgsStruct);
            if inherit {
                let original_ident = &item_struct.ident;
                // Define the enum variants
                let enum_ident = Ident::new(&format!("{}Inherited", original_ident), original_ident.span());
                let some_variant = Ident::new("Some", Span::call_site().into());
                let inherit_variant = Ident::new("Inherit", Span::call_site().into());
                let initial_variant = Ident::new("Initial", Span::call_site().into());

                let expanded = quote! {
                    #[derive(Hash, Debug)]
                    pub enum #enum_ident {
                        #some_variant(#original_ident),
                        #inherit_variant,
                        #initial_variant
                    }
                };
                let some_variant_impl = quote! {
                    impl From<#original_ident> for #enum_ident {
                        fn from(item: #original_ident) -> Self {
                            #enum_ident::#some_variant(item)
                        }
                    }
                };
                final_output.extend(expanded);
                final_output.extend(some_variant_impl);
            }
            if !initial_inherit_wrap.is_empty() {
                for tgt_field_names in initial_inherit_wrap.iter() {
                    for field in item_struct.fields.iter_mut() {
                        let field_ident = &field.ident;
                        if let Some(field_orig) = field_ident {
                            if field_orig == tgt_field_names {
                                let enum_ident = Ident::new(
                                    &format!("{}Inherited", to_upper_camel_case(&capitalize_first_char(&field_orig.to_string()))),
                                    field_orig.span()
                                );
                                let some_variant = Ident::new("Some", Span::call_site().into());
                                let inherit_variant = Ident::new("Inherit", Span::call_site().into());
                                let initial_variant = Ident::new("Initial", Span::call_site().into());
                                let field_type_ident = if let syn::Type::Path(path) = &field.ty {
                                    if let Some(segment) = path.path.segments.last() {
                                        &segment.ident
                                    } else {
                                        panic!("Field type not found for field '{}'", field_orig);
                                    }
                                } else {
                                    panic!("Field type not found for field '{}'", field_orig);
                                };
                                let expanded = quote! {
                                    #[derive(Hash, Debug)]
                                    pub enum #enum_ident {
                                        #some_variant(#field_type_ident),
                                        #inherit_variant,
                                        #initial_variant
                                    }
                                };
                                let some_variant_impl = quote! {
                                    impl From<#field_type_ident> for #enum_ident {
                                        fn from(item: #field_type_ident) -> Self {
                                            #enum_ident::#some_variant(item)
                                        }
                                    }
                                };
                                final_output.extend(expanded);
                                final_output.extend(some_variant_impl);
                                field.ty = syn::parse_quote! { #enum_ident };
                            }
                        }
                    }
                }
            }
            final_output.extend(quote! {#item_struct});
            return final_output.into();
        }
    }
}

#[proc_macro_derive(ExportField, attributes(export))]
pub fn export_field_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut structs = Vec::new();
    
    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            for field in fields_named.named {
                let mut generate_export_struct = false;
                for attr in field.attrs.iter() {
                    if attr.path().is_ident("export") {
                        generate_export_struct = true;
                    }
                }
                
                if generate_export_struct {
                    let field_name = field.ident.clone().unwrap();
                    let field_type = field.ty.clone();
                    let new_struct_name = format_ident!("{}{}", name, to_upper_camel_case(&field_name.to_string()));
                    let empty_struct = quote! {
                        struct #new_struct_name(#field_type);
                    };
                    
                    structs.push(empty_struct);
                }
            }
        }
    }
    
    let expanded = quote! {
        #(#structs)*
    };
    
    TokenStream::from(expanded)
}

