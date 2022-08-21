use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    Result, parse_macro_input,
    punctuated::Punctuated, Expr, Token, 
    parse::{Parse, ParseStream}, Error,
};

#[derive(Debug)]
struct LinkWithCallback {
    // parent : Expr,
    // title : Expr,
    text : Expr,
    module_type : Ident,
    module_handler_fn : Ident,
    args : Vec<Expr>,
    // cls: String
} 

impl Parse for LinkWithCallback {
    fn parse(input: ParseStream) -> Result<Self> {

        let usage = "<text>, <ModuleType::handler_fn>, [args, ...]";

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 2 {
            return Err(Error::new_spanned(
                parsed,
                format!("not enough arguments - usage: {}", usage)
            ));
        }
        //  else if parsed.len() > 2 {
        //     return Err(Error::new_spanned(
        //         parsed,
        //         format!("too many arguments - usage: {}", usage)
        //     ));
        // }
        
        let mut iter = parsed.iter();
        // let parent = iter.next().clone().unwrap().clone();
        // let title = iter.next().clone().unwrap().clone();
        let text = iter.next().clone().unwrap().clone();
        let handler = iter.next().clone().unwrap().clone();
        // let mut next_exp = iter.next().clone().unwrap().clone();
        // let mut cls = String::from("");
        // match &next_exp {
        //     Expr::Lit(exp)=>{
        //         match &exp.lit{
        //             Lit::Str(str)=>{
        //                 cls = str.value();
        //             },
        //             _=>{

        //             }
        //         }
        //         next_exp = iter.next().clone().unwrap().clone();
        //     }
        //     _ => {

        //     }
        // }

        let (module_type, module_handler_fn) = match &handler {
            Expr::Path(expr_path) => {
                let segments = expr_path.path.segments.clone();
                if segments.len() != 2 {
                    return Err(Error::new_spanned(
                        handler.clone(),
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
                    handler.clone(),
                    format!("last argument should be a path to module handler fn <ModuleType::handler_fn>")
                ));
            }
        };

        let args : Vec<Expr> = iter.map(|v|v.clone()).collect();

        let link = LinkWithCallback {
            text,
            module_type,
            module_handler_fn,
            args,
            // cls
        };
        Ok(link)
    }
}


pub fn link_with_callback(input: TokenStream) -> TokenStream {
    let link = parse_macro_input!(input as LinkWithCallback);

    let text = link.text;
    // let cls = link.cls;
    let module_type = link.module_type;
    let module_handler_fn = link.module_handler_fn;
    let args = link.args;

    let (transforms, args) = if args.len() == 0 {
        (quote!{},quote!{})
    } else {
        (quote!{
            #(let #args = #args.clone();)*
        },
        quote!{
            #(#args),*
        })
    };

    (quote!{

        {
            workflow_ux::link::Link::new_for_callback(#text)?
            .with_callback(Box::new(move ||{
                #transforms
                // let target = target.clone();
                workflow_core::task::wasm::spawn(async move {
                    #module_type::get().#module_handler_fn(#args).await.map_err(|e| { log_error!("{}",e); }).ok();
                    // log_trace!("callback for link element: {:?}", target);
                    // target.select().ok();
                });
                Ok(())
            }))?
        }
    }).into()
}




#[derive(Debug)]
struct LinkWithUrl {
    text : Expr,
    url : Expr,
}

impl Parse for LinkWithUrl {
    fn parse(input: ParseStream) -> Result<Self> {

        let usage = "<text>, <url>, <ModuleType::handler_fn>";

        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() < 3 {
            return Err(Error::new_spanned(
                parsed,
                format!("not enough arguments - usage: {}", usage)
            ));
        } else if parsed.len() > 3 {
            return Err(Error::new_spanned(
                parsed,
                format!("too many arguments - usage: {}", usage)
            ));
        }
        
        let mut iter = parsed.iter();
        let text = iter.next().clone().unwrap().clone();
        let url = iter.next().clone().unwrap().clone();

        let link = LinkWithUrl {
            text,
            url,
        };
        Ok(link)
    }
}


pub fn link_with_url(input: TokenStream) -> TokenStream {
    let link = parse_macro_input!(input as LinkWithUrl);
    let text = link.text;
    let url = link.url;

    (quote!{

        {
            workflow_ux::link::Link::new_with_url(&#text,#url)?
        }


    }).into()


}

