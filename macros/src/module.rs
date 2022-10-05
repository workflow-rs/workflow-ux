use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, ExprArray,
    Result, parse_macro_input,
    punctuated::Punctuated, Expr, Token, 
    parse::{Parse, ParseStream}, Error,
    DeriveInput
};
use convert_case::{Case, Casing};

#[derive(Debug)]
struct Module {
    struct_name : Expr,//Path,
    struct_name_string : String,
    container_types : Option<ExprArray>
}

impl Parse for Module {
    fn parse(input: ParseStream) -> Result<Self> {

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 1 || parsed.len() > 2 {
            return Err(Error::new_spanned(
                parsed,
                format!("usage: declare_modile!(<module_name> [, <container type array>])")
            ));
        }
        
        let mut iter = parsed.iter();
        let expr = iter.next().clone().unwrap().clone();
        let struct_name = match &expr {
            Expr::Path(_path) => expr,
            _ => {
                return Err(Error::new_spanned(
                    expr,
                    format!("the first argument should be the module struct")
                ));
            }
        };

        let container_types = if parsed.len() > 1 {
            let container_types_expr = iter.next().unwrap().clone();
            match container_types_expr {
                Expr::Array(array) => Some(array),
                _ => {
                    return Err(Error::new_spanned(
                        container_types_expr,
                        format!("the second argument should be and array of container types (ids)")
                    ));
                }
            }
        } else {
            None
        };

        let struct_name_string = quote!{ #struct_name }.to_string();

        let handlers = Module {
            struct_name,
            struct_name_string,//: module_struct_string,
            container_types,
        };
        Ok(handlers)
    }
}


pub fn declare_module(input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as Module);
    module_impl(
        module.struct_name,
        &module.struct_name_string,
        module.container_types,
        "".to_string()
    ).into()
}

pub fn derive_module(input: TokenStream) -> TokenStream {

    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let mut required_module_str = "".to_string();
    for a in &ast.attrs{
        if let Some(i) = a.path.get_ident(){
            let name = i.to_string();
            //println!("attrs::::::{:?}, tokens:{:?}", name, a.tokens);
            if !name.eq("required_module"){
                continue;
            }
            let mut tokens = a.tokens.clone().into_iter();
            if let Some(tt) = tokens.next(){
                if tt.to_string().eq("="){
                    if let Some(tt) = tokens.next(){
                        let mod_name = tt.to_string().replace("\"", "").to_lowercase();
                        required_module_str = format!("_{}", Ident::new(&mod_name, Span::call_site()));
                        //println!("RequiredModule attr found: {}", required_module_str);
                    }
                }
            }
        }
    }
    //println!("attrs::::::ast.attrs:{:?}", ast.attrs);
    // let struct_params = &ast.ident;

    let _fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        return Error::new_spanned(
            struct_name,
            format!("#[derive(Module)] supports only struct declarations")
        )
        .to_compile_error()
        .into();
    };

    let struct_name_string = quote!{ #struct_name}.to_string();//.to_lowercase();
    let path = syn::parse_str::<Expr>(&struct_name_string).expect("Unable to parse strut name as expression");
    
    module_impl(
        path,
        &struct_name_string,
        None,
        required_module_str
    ).into()

}

fn module_impl(module_struct : Expr, module_name : &str, container_types : Option<ExprArray>, required_module_str:String) -> TokenStream {

    let module_name = module_name//.to_lowercase();
        .from_case(Case::Camel)
        .to_case(Case::Snake);


    let module_register_ = Ident::new(&format!("module_register_{}_wasm{}", module_name, required_module_str), Span::call_site());

    let container_types = match container_types {
        None => quote!{[]},
        Some(container_types) => {
            quote!{
                #container_types
            }
        }
    };
    
    (quote!{

        // impl workflow_ux::module::ModuleInterface for #module_struct {
        //     fn type_id(self : Arc<Self>) -> Option<std::any::TypeId> { Some(std::any::TypeId::of::<#module_struct>()) }
        // }
        
        unsafe impl Send for #module_struct { }
        unsafe impl Sync for #module_struct { }


        impl #module_struct {
            pub async fn register_module() -> workflow_ux::result::Result<()> {
                let module = #module_struct::new()
                    .map_err(|err| 
                        workflow_ux::error::Error::ModuleRegistrationFailure(
                            #module_name.to_string(),
                            format!("{:?}", err).to_string()
                        )
                    )?;
                    // .expect(format!("Failure registering module {}", #module_name));
                Ok(workflow_ux::module::register(#module_name, std::sync::Arc::new(module),&#container_types).await?)
            }
    
            pub fn get() -> Option<std::sync::Arc<#module_struct>> {
                match workflow_ux::module::get_interface::<#module_struct>(#module_name)
                {
                    Some(module) => Some(module),
                    None => {
                        workflow_log::log_error!("error: module '{}' not found in the registry", #module_name);
                        None
                    },
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        #[macro_use]
        mod wasm {
            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn #module_register_() -> workflow_ux::result::Result<()> {
                Ok(super::#module_struct::register_module().await?)
            }
        }

    }).into()
}

// pub fn identifier(input: TokenStream) -> TokenStream {
//     let string = input.to_string();
//     (quote!{ #string }).into()
// }
