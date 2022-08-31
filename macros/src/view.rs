
use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
// use proc_macro2::{Span, Ident};
use quote::quote;
use syn::{
    DeriveInput,
// Result, 
    Error,
    parse_macro_input, 
    // PathArguments, 
    punctuated::Punctuated, 
    // Expr, 
    Token, 
    parse::{Parse, ParseStream}, parse_quote, Expr, 
    // ExprPath, PathSegment,
};
// use std::convert::Into;
// use proc_macro::TokenStream;
// use quote::{quote};
// // use quote::{quote, quote_spanned};
// use syn::{
//     Error,
//     Expr,
//     DeriveInput,
//     parse_macro_input, Type, PathArguments, parse::{ParseBuffer, ParseStream, Parse},
//     parse2, Token, punctuated::Punctuated, parse_quote
//     // spanned::Spanned
// };

#[allow(dead_code)]


struct ParsableNamedField {
    pub field: syn::Field,
}

impl Parse for ParsableNamedField {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        let field = syn::Field::parse_named(input)?;

        Ok(ParsableNamedField {
            field,
        })
    }
}



struct Attributes {
    pub evict : TokenStream2
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // input.len();

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();

        if parsed.len() > 1 {
            return Err(Error::new_spanned(
                parsed,
                format!("usage: #[view[(evict::Allow or evict::Disallow)]]")
            ));
        }


        let evict = if parsed.len() < 1 {
            quote!{ workflow_ux::view::Eviction::Allow }
        } else {

            let mut iter = parsed.iter();
            let expr = iter.next().clone().unwrap().clone();
            // let evict_disposition = 
            match &expr {
                Expr::Path(_) => quote! { #expr }, //.clone(),
                _ => {
                    // quote!{ workflow_ux::view::Eviction::Allow }//.into()
                    return Err(Error::new_spanned(
                        expr,
                        format!("the first argument should be eviction dispotition enum (Eviction::Allow or Eviciton::Disallow))")
                    ));
                }
            }
        };

        // let evict_dispotition = input.parse::<syn::Path>()?;
        // let evict_dispotition = input.parse::<syn::Path>()?;
        
        Ok(Attributes { 
            evict : evict.clone()
        })
    }
}


pub fn view(attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("panel attrs: {:#?}",attr);
    // let layout_attributes = parse_macro_input!(attr as Args);
    // println!("************************ \n\n\n\n\n{:#?}",cattr);

    let attributes = parse_macro_input!(attr as Attributes);

    // let struct_decl = item.clone();
    let struct_decl_ast = item.clone();
    let mut ast = parse_macro_input!(struct_decl_ast as DeriveInput);
    // let struct_name = &ast.ident;
    let struct_name = &ast.ident;
    let struct_params = &ast.generics;
    // let struct_params = &ast.generics;

    let ast = match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {

                    let punctuated_fields: Punctuated<ParsableNamedField, Token![,]> = parse_quote! {
                        element : web_sys::Element,
                        module : Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>>,
                    };
                
                    fields
                        .named
                        .extend(punctuated_fields.into_iter().map(|p| p.field));
                }   
                _ => {
                    ()
                }
            }              
            
            &ast
            // return quote! {
            //     #ast
            // }.into();
        }
        _ => {
            return Error::new_spanned(
                struct_name,
                format!("#[view] macro only supports structs")
            )
            .to_compile_error()
            .into();
        }
    };

    let _evict = attributes.evict.clone(); //.to_token_stream();

    let ts = quote!{

        #ast

        unsafe impl #struct_params Send for #struct_name #struct_params { }
        unsafe impl #struct_params Sync for #struct_name #struct_params { }
        
        impl #struct_name #struct_params {
            // fn element() -> workflow_allocator::result::Result<web_sys::Element> { 
            fn element() -> web_sys::Element { 
                workflow_ux::document().create_element("workspace-view").expect("unable to create workspace-view element")
            }
        }

        impl workflow_ux::view::View for #struct_name {

            fn element(&self) -> web_sys::Element {
                self.element.clone()
            }
            fn module(&self) -> Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>> {
                self.module.clone()
            }
            fn typeid(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }
            // fn evict(&self) -> workflow_ux::view::Eviction {
            //     #evict
            // }
        }

    };
    ts.into()
}

