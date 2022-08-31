// Concept abandoned

use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::{
    TokenTree //, Literal
};
use quote::{quote, ToTokens};
use syn::{
    Error, Ident, Variant,
};
use syn::{
    parse_macro_input,
    DeriveInput,
};
use workflow_macro_tools::attributes::*;

use convert_case::{Case, Casing};

#[derive(Debug)]
struct Icon {
    // opts : Opts,
    // args : Option<FieldAttributes>,
    pub args : Args,
    // field_name : syn::Ident,
    // name : String,
    variant: Variant,
    // literal_key_str : LitStr,
    // type_name : Type,
    // type_name_ident : Option<TypePath>,
    // visibility: Visibility,
    // type_name_str : String,
    // type_name_args : Option<String>,
    // docs : Vec<String>,
    // docs : Vec<Literal>,
}

// pub fn macro_print_enum_trait() -> TokenStream{
//     quote!{
//         pub trait EnumTrait<T>:Display+Debug{
//             fn list()->Vec<T>;
//             fn descr(&self)->&'static str;
//             fn as_str(&self)->&'static str;
//             fn from_str(str:&str)->Option<T>;
//         }
//     }.into()
// }


impl ToTokens for Icon {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        //tokens.append_all(&self.attrs);
        // self.visibility.to_tokens(tokens);
        //if let Some(ident) = &self.field_name {
            self.variant.to_tokens(tokens);
            // tokens.append(Punct::new(':', Spacing::Alone));
            //TokensOrDefault(&self.colon_token).to_tokens(tokens);
        //}
        // self.type_name.to_tokens(tokens);
    }
}


// #[proc_macro_attribute]
pub fn macro_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("panel attrs: {:#?}",attr);
    let enum_attributes = parse_macro_input!(attr as Args);
    // println!("************************ \n\n\n\n\n{:#?}",cattr);

    // let struct_decl = item.clone();
    let enum_decl_ast = item.clone();
    let ast = parse_macro_input!(enum_decl_ast as DeriveInput);

    // println!("************************ \n\n\n\n\n");
    // println!("ast data: {:#?}", ast);
    // println!("************************ \n\n\n\n\n");
    // if let Data::Enum(_enum) = &ast.data {
    //     let ident = &input.ident;
    // }

    let enum_name = &ast.ident;
    let _enum_params = &ast.generics;

    let enum_fields = if let syn::Data::Enum(syn::DataEnum {
        variants,
        // fields: syn::Enums::Named(ref fields),
        // fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        variants
    } else {
        return Error::new_spanned(
            enum_name,
            format!("#[enums] macro only supports enums")
        )
        .to_compile_error()
        .into();
    };

    let mut enums : Vec<Icon> = Vec::new();
    for variant in enum_fields.iter() {
        // let variant_name: syn::Ident = enum_variant.to_token_stream();//.as_ref().unwrap().clone();
        let name: String = variant.to_token_stream().to_string();

        let attrs: Vec<_> =
            variant.attrs.iter().filter(|attr| attr.path.is_ident("descr")).collect();
        if attrs.len() > 1 {
            return Error::new_spanned(
                enum_name,
                format!("#[field]: more than one #[field()] attributes while processing {}", name)
            )
            .to_compile_error()
            .into();
    
        }

        // let args = Args::new();
        // println!("+++++++++++++PROCESSING ARGS");
        let args = match attrs.first() {
            Some(attr) => {
                match get_attributes(attr) {
                    Some(attr) => attr,
                    _ => { Args::new() }
                }
            },
            _ => { Args::new() }
        };

        // println!("++++ENUM ARGS/ATTRIBUTES ARE: {:#?}",args);

        let mut docs = Vec::new();
        for attr in variant.attrs.iter() {

            let path_seg = attr.path.segments.last() ;//.unwrap();
            if !path_seg.is_some() { continue }
            let segment = path_seg.unwrap();
            if segment.ident == "doc" {
                let mut tokens = attr.tokens.clone().into_iter();
                match tokens.next() {
                    Some(TokenTree::Punct(_punct)) => {
                        match tokens.next() {
                            Some(TokenTree::Literal(lit)) => {
                                docs.push(lit.clone());
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        // println!("!!!!!!!!!!!!!!!!!!!!!!  ========= > {:#?}", field_name);
        let enum_instance = Icon {
            args,
            // field_name: variant_name,
            // name,
            // literal_key_str,
            variant : variant.clone(),
            // type_name_str,
            // type_na÷me_ident,
            // visibility,
            // docs
            // type_name_args,
        };

        enums.push(enum_instance);
    }

    
    // println!("******************************** FIELD DONE PTITLE");

    // let attrs_kv = enum_attributes.to_string_kv();
    // let attrs_k : Vec<String> = attrs_kv.iter().map(|item| {
    //     let v = item.0.clone();
    //     let ts : TokenStream = str::parse::<TokenStream>(&v).unwrap();
    //     ts.to_string()
    // }).collect();
    // let attrs_v : Vec<String> = attrs_kv.iter().map(|item|item.1.to_string()).collect();

    // println!("******************************** QUOTING enums: {:#?}", enums);

    let entries : Vec<Ident> = enums.iter().map(|e|e.variant.ident.clone()).collect();

    let strings : Vec<String> = entries.iter().map(|ident|{
            format!("{}::{}", enum_name, ident)
    }).collect();

    let consts : Vec<String> = entries.iter().map(|ident|{
            // format!("{}", ident)
            ident.to_string()
                .from_case(Case::Camel)
                // .without_boundaries(&Boundary::digit_letter())
                .to_case(Case::UpperSnake)
    }).collect();

    let icon_root = "../../root/resources/icons";
    let filenames : Vec<String> = entries.iter().map(|ident|{
            let ident_string = ident.to_string()
                .from_case(Case::Camel)
                // .without_boundaries(&Boundary::digit_letter())
                .to_case(Case::Kebab);
            format!("{}/{}", icon_root, ident_string)

    }).collect();

    // let vvv = vec!["AAA","BBB"];
    // let entries = enum_fields.to_token_stream();

    let mut descr : Vec<String> = Vec::new();
    for e in enums.iter() {
        let have_key = e.args.has("default");
        if !have_key{
            descr.push(format!("{}", e.variant.ident.clone()));
        }else if let Some(info) = e.args.get("default").unwrap(){
            descr.push(info.to_token_stream().to_string().replace('\"', ""));
        } else {
            descr.push(format!("{}", e.variant.ident.clone()));
        }
    }

    //let len = strings.len();

    let result = quote!{

        pub enum #enum_name {
            // #entries
            #( #entries ),*
            // #( #enums ),*
        }

        // const TOKEN_BYTES:&str = include_str!("../../root/tokens.json");
        mod data {
            #( const #consts:&str = include_str!(#filenames); ),*
        }

        impl #enum_name {

            pub fn test() -> bool { true }

            pub fn list() -> Vec<#enum_name> {
                vec![#( #enum_name::#entries ),*]
            }
            pub fn as_str(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings.into() }),*
                }
            }
            pub fn from_str(str:&str)->Option<#enum_name>{
                match str {
                    #( #strings => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }
            pub fn descr(&self) -> &'static str { 
                match self {
                    #( #enum_name::#entries => { #descr.into() }),*

                    // TODO: !!!  @aspect
                    // #( #enum_name::#entries => { #descr.into() }),*

                }
            }
        }

        impl workflow_ux::enums::EnumTrait<#enum_name> for #enum_name {
            fn list() -> Vec<#enum_name> {
                #enum_name::list()
            }
            fn descr(&self) -> &'static str {
                self.descr()
            }
            fn as_str(&self) -> &'static str {
                self.as_str()
            }

            fn from_str(str:&str)->Option<#enum_name>{
                #enum_name::from_str(str)
            }
        }

        impl std::fmt::Display for #enum_name{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

    }.into();

    // println!("######======######================================================================");
    // println!("{:#?}", result);
    // println!("######======######================================================================");

    result
}








