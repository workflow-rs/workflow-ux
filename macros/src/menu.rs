use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    Result, parse_macro_input,
    punctuated::Punctuated, Expr, Token, 
    parse::{Parse, ParseStream}, Error,
};

#[derive(Debug)]
struct Menu {
    parent : Expr,
    title : Expr,
    icon : Expr,
    module_type : Ident,
    module_handler_fn : Ident,
}

impl Parse for Menu {
    fn parse(input: ParseStream) -> Result<Self> {

        let usage = "<parent>, <title>, <icon>, <ModuleType::handler_fn>";

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 4 {
            return Err(Error::new_spanned(
                parsed,
                format!("not enough arguments - usage: {}", usage)
            ));
        } else if parsed.len() > 4 {
            return Err(Error::new_spanned(
                parsed,
                format!("too many arguments - usage: {}", usage)
            ));
        }
        
        let mut iter = parsed.iter();
        let parent = iter.next().clone().unwrap().clone();
        let title = iter.next().clone().unwrap().clone();
        let icon = iter.next().clone().unwrap().clone();
        let module_handler_path = iter.next().clone().unwrap().clone();

        let (module_type, module_handler_fn) = match &module_handler_path {
            Expr::Path(expr_path) => {
                let segments = expr_path.path.segments.clone();
                if segments.len() != 2 {
                    return Err(Error::new_spanned(
                        module_handler_path.clone(),
                        format!("module handler path should have the following format: ModuleType::handler_fn")
                    ));
                } else {
                    let mut segments = segments.iter();
                    let module_type = segments.next().clone().unwrap().ident.clone();
                    let handler_fn = segments.next().clone().unwrap().ident.clone();
                    (module_type, handler_fn)
                }
            },
            _ => {
                return Err(Error::new_spanned(
                    module_handler_path.clone(),
                    format!("last argument should be a path to module handler fn <ModuleType::handler_fn>")
                ));
            }
        };

        let menu = Menu {
            parent,
            title,
            icon,
            module_type,
            module_handler_fn,
        };
        Ok(menu)
    }
}


pub fn menu(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as Menu);
    let menu_type = Ident::new("MenuItem", Span::call_site());
    menu_impl(
        menu_type,
        menu.parent,
        menu.title,
        menu.icon,
        menu.module_type,
        menu.module_handler_fn
    ).into()
}

pub fn menu_group(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as Menu);
    let menu_type = Ident::new("MenuGroup", Span::call_site());
    menu_impl(
        menu_type,
        menu.parent,
        menu.title,
        menu.icon,
        menu.module_type,
        menu.module_handler_fn
    ).into()
}

pub fn popup_menu(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as Menu);
    
    /*
    let menu_type = Ident::new("MainMenu", Span::call_site());
    menu_impl(
        menu_type,
        menu.parent,
        menu.title,
        menu.icon,
        menu.module_type,
        menu.module_handler_fn
    ).into()
    */

    let icon = menu.icon;
    let parent = menu.parent;
    let title = menu.title;
    let module_type = menu.module_type;
    let module_handler_fn = menu.module_handler_fn;


    (quote!{

        {
            workflow_ux::popup_menu::MenuItem::new(&#parent, #title.into(), #icon)?
            .with_callback(Box::new(move |target|{
                if let Some(popup_menu) = workflow_ux::popup_menu::get_popup_menu(){
                    popup_menu.close().map_err(|e| { log_error!("unable to close popup menu: {}", e); }).ok();
                }
                let target = target.clone();
                workflow_core::task::wasm::spawn(async move {
                    #module_type::get().unwrap().#module_handler_fn().await.map_err(|e| { log_error!("{}",e); }).ok();
                    workflow_log::log_trace!("selecting target element: {:?}", target);
                    //target.select().ok();
                });
                Ok(())
            }))?
        }


    }).into()
}


fn menu_impl(
    menu_type : Ident,
    parent : Expr,
    title : Expr,
    icon : Expr,
    module_type : Ident,
    module_handler_fn : Ident,

) -> TokenStream {

    (quote!{

        {
            workflow_ux::menu::#menu_type::new(&#parent,#title.into(),#icon)?
            .with_callback(Box::new(move |target|{
                let target = target.clone();
                workflow_core::task::wasm::spawn(async move {
                    #module_type::get().unwrap().#module_handler_fn().await.map_err(|e| { log_error!("{}",e); }).ok();
                    workflow_log::log_trace!("selecting target element: {:?}", target);
                    target.select().ok();
                });
                Ok(())
            }))?
        }


    }).into()
}
