use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::convert::Into;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, Result, Token,
};

#[derive(Debug)]
struct Menu {
    parent: Expr,
    title: Expr,
    icon: Expr,
    module_type: Ident,
    module_handler_fn: Ident,
}

impl Parse for Menu {
    fn parse(input: ParseStream) -> Result<Self> {
        let usage = "<parent>, <title>, <icon>, <ModuleType::handler_fn>";

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 4 {
            return Err(Error::new_spanned(
                parsed,
                format!("not enough arguments - usage: {}", usage),
            ));
        } else if parsed.len() > 4 {
            return Err(Error::new_spanned(
                parsed,
                format!("too many arguments - usage: {}", usage),
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
            }
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

#[derive(Debug)]
struct SectionMenu {
    parent: Expr,
    title: Expr,
    icon: Expr,
}

impl Parse for SectionMenu {
    fn parse(input: ParseStream) -> Result<Self> {
        let usage = "<parent>, <title>, <icon>";

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 3 {
            return Err(Error::new_spanned(
                parsed,
                format!("not enough arguments - usage: {}", usage),
            ));
        } else if parsed.len() > 3 {
            return Err(Error::new_spanned(
                parsed,
                format!("too many arguments - usage: {}", usage),
            ));
        }

        let mut iter = parsed.iter();
        let parent = iter.next().clone().unwrap().clone();
        let title = iter.next().clone().unwrap().clone();
        let icon = iter.next().clone().unwrap().clone();

        let menu = SectionMenu {
            parent,
            title,
            icon,
        };
        Ok(menu)
    }
}

#[derive(Debug)]
struct MenuGroup {
    parent: Expr,
    title: Expr,
}

impl Parse for MenuGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let usage = "<parent>, <title>";

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 2 {
            return Err(Error::new_spanned(
                parsed,
                format!("not enough arguments - usage: {}", usage),
            ));
        } else if parsed.len() > 2 {
            return Err(Error::new_spanned(
                parsed,
                format!("too many arguments - usage: {}", usage),
            ));
        }

        let mut iter = parsed.iter();
        let parent = iter.next().clone().unwrap().clone();
        let title = iter.next().clone().unwrap().clone();

        let menu = Self { parent, title };
        Ok(menu)
    }
}

pub fn section_menu(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as SectionMenu);
    let menu_type = Ident::new("SectionMenu", Span::call_site());
    menu_impl(menu_type, menu.parent, menu.title, menu.icon).into()
}

pub fn menu_group(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as MenuGroup);
    let menu_type = Ident::new("MenuGroup", Span::call_site());
    let parent = menu.parent;
    let title = menu.title;
    (quote! {

        workflow_ux::menu::#menu_type::new(&#parent,#title.into())?
        .with_callback(Box::new(move |target|{
            target.toggle().ok();
            Ok(())
        }))?

    })
    .into()
}

pub fn menu_item(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as Menu);
    let menu_type = Ident::new("MenuItem", Span::call_site());
    menu_with_callback(
        menu_type,
        menu.parent,
        menu.title,
        menu.icon,
        menu.module_type,
        menu.module_handler_fn,
    )
    .into()
}

pub fn popup_menu(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as Menu);

    let icon = menu.icon;
    let parent = menu.parent;
    let title = menu.title;
    let module_type = menu.module_type;
    let module_handler_fn = menu.module_handler_fn;

    (quote!{

        {
            workflow_ux::popup_menu::PopupMenuItem::new(&#parent, #title.into(), #icon)?
            .with_callback(Box::new(move |target|{
                if let Some(popup_menu) = workflow_ux::popup_menu::get_popup_menu(){
                    popup_menu.close().map_err(|e| { log_error!("unable to close popup menu: {}", e); }).ok();
                }
                let target = target.clone();
                workflow_core::task::wasm::dispatch(async move {
                    #module_type::get().unwrap().#module_handler_fn()
                    .await.map_err(|e| {
                        log_error!("{}",e);
                    }).ok();
                    //workflow_log::log_trace!("selecting target element: {:?}", target);
                    //target.select().ok();
                });
                Ok(())
            }))?
        }


    }).into()
}

pub fn popup_menu_link(input: TokenStream) -> TokenStream {
    let menu = parse_macro_input!(input as Menu);

    let icon = menu.icon;
    let parent = menu.parent;
    let title = menu.title;
    let module_type = menu.module_type;
    let menu = menu.module_handler_fn;

    (quote! {

        {
            workflow_ux::popup_menu::PopupMenuItem::new(&#parent, #title.into(), #icon)?
            .with_callback(Box::new(move |target|{
                if let Some(popup_menu) = workflow_ux::popup_menu::get_popup_menu(){
                    popup_menu.close().map_err(|e| {
                        log_error!("unable to close popup menu: {}", e);
                    }).ok();
                }
                let target = target.clone();
                workflow_core::task::wasm::dispatch(async move {
                    #module_type::get().unwrap().menu.#menu.activate()
                    .map_err(|e| {
                        log_error!("{}",e);
                    }).ok();
                    //workflow_log::log_trace!("selecting target element: {:?}", target);
                    //target.select().ok();
                });
                Ok(())
            }))?
        }


    })
    .into()
}

fn menu_impl(menu_type: Ident, parent: Expr, title: Expr, icon: Expr) -> TokenStream {
    (quote! {

        workflow_ux::menu::#menu_type::new(&#parent,#title.into(),#icon)?
        .with_callback(Box::new(move |target|{
            target.select().ok();
            Ok(())
        }))?

    })
    .into()
}

fn menu_with_callback(
    menu_type: Ident,
    parent: Expr,
    title: Expr,
    icon: Expr,
    module_type: Ident,
    module_handler_fn: Ident,
) -> TokenStream {
    (quote! {

        {
            workflow_ux::menu::#menu_type::new(&#parent,#title.into(),#icon)?
            .with_callback(Box::new(move |target|{
                let target = target.clone();
                workflow_core::task::wasm::dispatch(async move {
                    match #module_type::get().unwrap().#module_handler_fn().await{
                        Ok(v)=>{
                            //workflow_log::log_trace!("selecting target element: {:?}", target);
                            target.select().ok();
                        }
                        Err(e)=>{
                            log_error!("{}",e);
                        }
                    };
                });
                Ok(())
            }))?
        }


    })
    .into()
}
