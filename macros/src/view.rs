use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::convert::Into;
// use proc_macro2::{Span, Ident};
//use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    parse_quote,
    //PathArguments,
    punctuated::Punctuated,
    //Ident,
    DeriveInput,
    //Result,
    Error,
    Expr,
    // ExprPath, PathSegment,
    //Expr,
    Token,
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

        Ok(ParsableNamedField { field })
    }
}

struct Attributes {
    pub evict: TokenStream2,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // input.len();

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();

        if parsed.len() > 1 {
            return Err(Error::new_spanned(
                parsed,
                format!("usage: #[view[(evict::Allow or evict::Disallow)]]"),
            ));
        }

        let evict = if parsed.len() < 1 {
            quote! { workflow_ux::view::Eviction::Allow }
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
            evict: evict.clone(),
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
                _ => (),
            }

            &ast
            // return quote! {
            //     #ast
            // }.into();
        }
        _ => {
            return Error::new_spanned(struct_name, format!("#[view] macro only supports structs"))
                .to_compile_error()
                .into();
        }
    };

    let _evict = attributes.evict.clone(); //.to_token_stream();

    let ts = quote! {

        #ast

        unsafe impl #struct_params Send for #struct_name #struct_params { }
        unsafe impl #struct_params Sync for #struct_name #struct_params { }

        impl #struct_name #struct_params {
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

pub fn html_view(item: TokenStream) -> TokenStream {
    let struct_decl_ast = item.clone();
    let mut ast = parse_macro_input!(struct_decl_ast as DeriveInput);
    let struct_name = &ast.ident;
    let struct_params = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    //let impl_generics = struct_params;
    //let ty_generics = struct_params;
    //let where_clause = quote!{};

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => match &mut struct_data.fields {
            syn::Fields::Named(fields) => {
                let mut has_html_field = false;
                for field in &fields.named {
                    let f_type = &field.ty;

                    if let Some(ident) = &field.ident {
                        let name = ident.to_string();
                        if name.eq("html") {
                            let check_list: Punctuated<ParsableNamedField, Token![,]> = parse_quote! {
                                f1: Arc<Mutex<Option<Arc<workflow_ux::view::Html>>>>,
                                f2: Arc<Mutex<Option<Arc<view::Html>>>>,
                                f3: Arc<Mutex<Option<Arc<Html>>>>,
                                f4: std::sync::Arc<Mutex<Option<std::sync::Arc<view::Html>>>>,
                                f5: std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<workflow_ux::view::Html>>>>,
                                f6: std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<view::Html>>>>,
                                f7: std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<Html>>>>
                            };
                            let f_type_str = format!("{}", f_type.to_token_stream());
                            let mut found = false;
                            for f in check_list {
                                let s = format!("{}", f.field.ty.to_token_stream());
                                if s.eq(&f_type_str) {
                                    found = true;
                                }
                            }

                            if !found {
                                continue;
                            }
                            has_html_field = true;
                            break;
                        }
                    }
                }

                if !has_html_field {
                    return Error::new_spanned(
                            struct_name,
                            format!("#[HtmlView] struct require 'html' member/property. \n`html: Arc<Mutex<Option<Arc<workflow_ux::view::Html>>>>`")
                        )
                        .to_compile_error()
                        .into();
                }
            }
            _ => (),
        },
        _ => {
            return Error::new_spanned(
                struct_name,
                format!("#[HtmlView] macro only supports structs"),
            )
            .to_compile_error()
            .into();
        }
    };

    let name = struct_name.to_string();
    let html_missing_msg = format!("{} requires inner html view", name);
    let evict_msg = format!("{} evict", name);
    let drop_msg = format!("{} drop", name);

    let ts = quote! {

        unsafe impl #struct_params Send for #struct_name #ty_generics #where_clause{ }
        unsafe impl #struct_params Sync for #struct_name #ty_generics #where_clause{ }

        impl #impl_generics  #struct_name #ty_generics #where_clause{
            pub fn get_html(&self)->workflow_ux::result::Result<Arc<workflow_ux::view::Html>>{
                let view = (*self.html.lock()?).clone().expect(#html_missing_msg);
                Ok(view)
            }
            pub async fn view_evict(self: Arc<Self>)->workflow_ux::result::Result<bool>{
                Ok(true)
            }
        }

        #[workflow_async_trait]
        impl #impl_generics workflow_ux::view::View for #struct_name #ty_generics #where_clause{
            fn element(&self) -> web_sys::Element {
                self.get_html().unwrap().element()
            }
            fn module(&self) -> Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>> {
                self.get_html().unwrap().module()
            }
            fn bottom_menus(&self)->Option<Vec<workflow_ux::bottom_menu::BottomMenuItem>>{
                self.get_html().unwrap().bottom_menus()
            }
            fn typeid(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }

            async fn evict(self:Arc<Self>) ->  Result<()>{
                log_info!(#evict_msg);

                if (self.clone() as Arc<dyn workflow_ux::view::Evict>).evict().await?{
                    let html = self.get_html().unwrap();
                    html.evict().await?;
                }

                Ok(())
            }
        }


        impl #impl_generics Drop for #struct_name #ty_generics #where_clause{
            fn drop(&mut self) {
                log_info!(#drop_msg);

                /*
                match self.unsubscribe(){
                    Ok(_)=>{}
                    Err(err)=>{
                        workflow_log::log_error!("View drop: unsubscribe error: {:?}", err);
                    }
                }
                */
            }
        }

    };

    ts.into()
}
