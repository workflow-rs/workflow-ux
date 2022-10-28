use std::collections::HashMap;
use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::{
    Span,
    // Ident,
    Punct,
    Spacing,
    TokenTree, Literal
    // Group,
};
//  use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
// use syn::parse::ParseBuffer;
use syn::{
    Type,
    Visibility, Error, Ident,
};
use syn::{
    parse_macro_input,
    PathArguments,
    DeriveInput,
    punctuated::Punctuated,
    Token,
};
use workflow_macro_tools::attributes::*;



#[derive(Debug)]
struct Field {
    // opts : Opts,
    // args : Option<FieldAttributes>,
    args : HashMap<String,Args>,
    field_name : syn::Ident,
    // name : String,
    // literal_key_str : LitStr,
    type_name : Type,
    // type_name_path : Option<syn::Path>,
    // type_name_generic : Option<syn::Path>,
    visibility: Visibility,
    // type_name_str : String,
    type_name_str_lower_case : String,
    // type_name_args : Option<String>,
    // docs : Vec<String>,
    docs : Vec<Literal>,
}

// impl Field {
//     pub fn is_meta(&self) -> bool { self.name == "meta"  || self.name == "_meta" }
// }

impl ToTokens for Field{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        //tokens.append_all(&self.attrs);
        self.visibility.to_tokens(tokens);
        //if let Some(ident) = &self.field_name {
            self.field_name.to_tokens(tokens);
            tokens.append(Punct::new(':', Spacing::Alone));
            //TokensOrDefault(&self.colon_token).to_tokens(tokens);
        //}
        self.type_name.to_tokens(tokens);
    }
}



fn get_type_info(field_type: &syn::Type) -> (Option<syn::Path>, Option<syn::Path>) {
    match field_type {
        Type::Path(type_path) => {
            let target = type_path.path.segments.last().unwrap();
            let type_name_str = target.ident.to_string();
            let type_name_args = match type_name_str.as_str() {
                "Query" | "Section" | "Pane" => {
                    match &target.arguments {
                        PathArguments::AngleBracketed(params) => {
                            let args = params.args.clone();
                            if args.len() != 1 {
                                None
                            } else {
                                let first = args[0].clone();
                                match first {
                                    syn::GenericArgument::Type(Type::Path(type_path)) => {
                                        Some((type_path.path.clone(),type_name_str))
                                    },
                                    _ => None,
                                }
                            }
                        },
                        _ => None
                    }
                },
                _ => None
            };

            match type_name_args {
                Some((type_name_args,_type_name_str)) => {
                    let type_path = type_path.clone();
                    let mut path = type_path.path.clone();
                    let last = path.segments.last_mut().unwrap();
                    last.arguments = syn::PathArguments::None;
        
                    (Some(path), Some(type_name_args))
                },
                None => {
                    (None,None)
                }
            }
        },
        _ => {
            (None,None)
        }
    }
}


// fn get_field_attributes(attr: &Attribute) -> syn::Result<FieldAttributes> {
//     let attributes: FieldAttributes = attr.parse_args().unwrap();
//     Ok(attributes)
// }

// fn get_field_attributes(attr: &Attribute) -> syn::Result<FieldAttributes> {
//     let attributes: FieldAttributes = attr.parse_args().unwrap();
//     Ok(attributes)
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Form,
    Stage,
    Section,
    Pane,
    Panel,
    Page,
    Group,
    Html,
}

// #[proc_macro_attribute]
pub fn macro_handler(layout: Layout, attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("panel attrs: {:#?}",attr);
    // println!("*** STARTING LAYOUT PROCESSING");

    let layout_attributes = parse_macro_input!(attr as Args);
    // println!("************************ \n\n\n\n\n{:#?}",cattr);

    // let struct_decl = item.clone();
    let struct_decl_ast = item.clone();
    let ast = parse_macro_input!(struct_decl_ast as DeriveInput);

    // println!("************************ \n\n\n\n\n");
    // println!("ast data: {:#?}", ast);
    // println!("************************ \n\n\n\n\n");

    let struct_name = &ast.ident;
    let struct_params = &ast.generics;
    let struct_name_string = quote!{ #struct_name}.to_string();

    // let generics = 

    let mut generics_only = ast.generics.clone();
    generics_only.params = {
        let mut params : Punctuated<syn::GenericParam, Token![,]> = Punctuated::new();
        for param in generics_only.params.iter() {
            match param {
                syn::GenericParam::Type(_) => {
                    params.push(param.clone());
                },
                _ => {}
            }
        }
        params
    };

    // let has_generics = generics_only.params.len() > 0;
    let _where_clause = match generics_only.where_clause.clone() {
        Some(where_clause) => quote!{ #where_clause },
        None => quote!{}
    };



    let struct_fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        return Error::new_spanned(
            struct_name,
            format!("#[panel] macro only supports structs")
        )
        .to_compile_error()
        .into();
    };

    let mut fields : Vec<Field> = Vec::new();
    for struct_field in struct_fields.named.iter() {
        let field_name: syn::Ident = struct_field.ident.as_ref().unwrap().clone();
        // let name: String = field_name.to_string();

        let ty = struct_field.ty.clone();
        let type_name_str = match ty {
            Type::Path(type_path) => {
                let target = type_path.path.segments.last().unwrap();
                target.ident.to_string()
            },
            _ => {
                return Error::new_spanned(
                    field_name,
                    format!("layout macro - unable to resolve type path")
                )
                .to_compile_error()
                .into();
            }

        };

        let type_name_str_lower_case = type_name_str.to_lowercase();

        let attrs: Vec<_> =
            struct_field.attrs.iter().filter(|attr| 
                attr.path.is_ident("field") || 
                attr.path.is_ident("layout") || 
//                attr.path.is_ident("option") ||   // should be processed before and retain types
                attr.path.is_ident("section") || 
                attr.path.is_ident("pane") || 
                attr.path.is_ident(&type_name_str_lower_case)
            ).collect();
        // if attrs.len() > 1 {
        //     return Error::new_spanned(
        //         struct_name,
        //         format!("#[field]: more than one #[field()] attributes while processing {}", name)
        //     )
        //     .to_compile_error()
        //     .into();
    
        // }

        // let args = Args::new();
        // println!("+++++++++++++PROCESSING ARGS");
        // println!("ARGS PROCESSING:{}, attrs:{:#?}", type_name_str_lower_case, attrs);

        let mut args: HashMap<String,Args> = HashMap::new();
        
        for attr in attrs.iter() {
            let name = attr.path.segments.first().unwrap().ident.clone().to_string();//.to_string();
            let attr_args: Args = match get_attributes(attr) {
                Some(args) => args,
                _ => { Args::new() }
            };
        
            match args.get_mut(&name) {
                Some(current_args) => {
                    for (k,v) in attr_args.map.iter() {
                        current_args.map.insert(k.clone(),v.clone());
                    }
                    // current_args.extend(attr_args);
                }, 
                None => {
                    args.insert(name, attr_args);
                }
            }
        }

        // let args = match attrs.first() {
        //     Some(attr) => {
        //         match get_attributes(attr) {
        //             Some(attr) => attr,
        //             _ => { Args::new() }
        //         }
        //     },
        //     _ => { Args::new() }
        // };
        // println!("+++++++++++++PROCESSING ARGS DONE");
        //println!("ARGS DONE:{}, args:{:#?}", type_name_str_lower_case, args);
        // let args = get_attributes(attrs.first().unwrap());
        // let args = if attrs.len() > 0 {
        //     let attr = attrs.remove(0);
        //     Some(get_attributes(attr).unwrap())
        // } else {
        //     None
        // };

        let type_name = struct_field.ty.clone();
        let visibility = struct_field.vis.clone();
        // let type_name_for_ident = type_name.clone();

        let (_type_name_path,_type_name_generic) = 
            get_type_info(&type_name);
        
        // match type_name_for_ident {
        //     Type::Path(mut type_path) => {
        //         let target = type_path.path.segments.last_mut().unwrap();
        //         let type_name_args = match &target.arguments {
        //             PathArguments::AngleBracketed(params) => {
        //                 let args = params.args.clone();
        //                 // args.iter()

        //                 Some(0)
        //                 // let mut ts = proc_macro2::TokenStream::new();
        //                 // params.args.clone().to_tokens(&mut ts);
        //                 // let lifetimes = ts.to_string();
        //                 // target.arguments = PathArguments::None;
        //                 // Some(lifetimes)
        //             },
        //             _ => None
        //         };

        //         (Some(type_path), Some(type_name_args))
        //     },
        //     _ => {
        //         (None,None)
        //     }
        // };

        let mut docs = Vec::new();
        for attr in struct_field.attrs.iter() {

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
        let field = Field {
            args,
            field_name,
            // name,
            // literal_key_str,
            type_name,
            // type_name_str,
            type_name_str_lower_case,
            // type_name_path,
            // type_name_generic,
            visibility,
            docs
            // type_name_args,
        };

        fields.push(field);
    }
// println!("******************************** FIELD DONE");
    let mut field_initializers = Vec::new();
    for field in fields.iter() {
        // println!("******************************** FIELD A {:?}",field);
        // println!("******************************** FIELD A");
        
        let field_name = &field.field_name;
        // if field_name == "layout" {
        //     continue;
        // }
        let type_name = &field.type_name;
        let docs = field.docs.clone();
        // let title = match field.args.map.get()
        
        // let title = match &field.args.get("title") {
        //     Some(Some(value)) => {
        //         let title = value.to_token_stream();
        //         // let title = args.title.clone().to_token_stream();
        //         quote! {
        //         // form_control.set_title("SUPER DUPER")?;
        //             form_control.set_title(#title)?;
        //         }
        //     },
        //     _ => {quote!{}}
        // };

        let no_args = Args::new();
        let field_args = field.args.get(&String::from("field")).unwrap_or(&no_args);
        let ctl_args = field.args.get(&field.type_name_str_lower_case).unwrap_or(&field_args);
        //println!("ctl_name: {:#?}, ctl_args:{:#?}, args: {:#?}", field.type_name_str_lower_case, ctl_args, field.args);

        let pane_args = field.args.get(&String::from("pane")).unwrap_or(&no_args);
        let layout_args = field.args.get(&String::from("layout")).unwrap_or(&pane_args);
        // let pane_args = field.args.get(&String::from("pane")).unwrap_or(&no_args);
        // let layout_args = field.args.get(&String::from("section")).unwrap_or(&pane_args);



        let ctl_attrs_kv = ctl_args.to_string_kv();
        let ctl_attrs_k : Vec<String> = ctl_attrs_kv.iter().map(|item|item.0.to_string()).collect();
        let ctl_attrs_v : Vec<String> = ctl_attrs_kv.iter().map(|item|item.1.to_string()).collect();
        let layout_attrs_kv = layout_args.to_string_kv();
        let layout_attrs_k : Vec<String> = layout_attrs_kv.iter().map(|item|item.0.to_string()).collect();
        let layout_attrs_v : Vec<String> = layout_attrs_kv.iter().map(|item|item.1.to_string()).collect();

        //println!("ctl: {}, ctl_attrs_k:{:#?}, ctl_attrs_v:{:#?}", field.type_name_str_lower_case, ctl_attrs_k, ctl_attrs_v);

        match layout {

            Layout::Html => {

                let field_name_string = quote!{ #field_name }.to_string();
                
                field_initializers.push(quote!{
                    let #field_name = {
                        let mut ctl_attributes = Attributes::new();
                        let ctl_attr_list : Vec<(String,String)> = vec![#(( #ctl_attrs_k.to_string(),#ctl_attrs_v.to_string() ) ), *];
                        for (k,v) in ctl_attr_list.iter() {
                            ctl_attributes.insert(k.to_string(),v.clone());
                        }
                        // let mut layout_attributes = Attributes::new();
                        // let layout_attr_list : Vec<(String,String)> = vec![#(( #layout_attrs_k.to_string(),#layout_attrs_v.to_string() ) ), *];
                        // for (k,v) in layout_attr_list.iter() {
                        //     layout_attributes.insert(k.to_string(),v.clone());
                        // }
                        let docs : Vec<&str> = vec![#( #docs ), *];
                        // let #field_name = #type_name::new(&_layout, &ctl_attributes, &docs)?;  // pane-ctl

                        // @html
                        let el = html.hooks().get(#field_name_string);
                        match el { 
                            Some(el) => {
                                hooks_used.remove(#field_name_string);
                                let element_binding_context = workflow_ux::control::ElementBindingContext::new(&_layout,el,&ctl_attributes,&docs);
                                let #field_name = #type_name::try_from(element_binding_context)?;
                                // let #field_name = #type_name::try_from((&_layout, el, &ctl_attributes, &docs))?;
                                #field_name
                            },
                            None => {
                                return Err(workflow_ux::error::error!("Layout: missing HTML binding for {}", #field_name_string));
                            }
                        }
                    };
                });
            },
            _ => {
                field_initializers.push(quote!{
                    let #field_name = {
                        let mut ctl_attributes = Attributes::new();
                        let ctl_attr_list : Vec<(String,String)> = vec![#(( #ctl_attrs_k.to_string(),#ctl_attrs_v.to_string() ) ), *];
                        for (k,v) in ctl_attr_list.iter() {
                            ctl_attributes.insert(k.to_string(),v.clone());
                        }
                        let mut layout_attributes = Attributes::new();
                        let layout_attr_list : Vec<(String,String)> = vec![#(( #layout_attrs_k.to_string(),#layout_attrs_v.to_string() ) ), *];
                        for (k,v) in layout_attr_list.iter() {
                            layout_attributes.insert(k.to_string(),v.clone());
                        }
                        // println!("********* ATTRIBUTE LIST: {:#?}",attr_list);
                        // println!("********* ATTRIBUTE MAP: {:#?}",attributes);
                        let docs : Vec<&str> = vec![#( #docs ), *];
                        let #field_name = #type_name::new(&_layout, &ctl_attributes, &docs)?;  // pane-ctl
                        let child = #field_name.element();
                        _layout.append_child(&child, &layout_attributes, &docs)?;
                        #field_name
                    };
                });
            },            
        }


/*
        match &field.type_name_generic {
            // ~ LAYOUT / OPTIONAL?
            Some(generic) => {
                let type_name_str = field.type_name_str.clone().unwrap();
                let layout_style = match type_name_str.as_str() {
                    "Block" => quote! { workflow_ux::layout::ElementLayoutStyle::Block },
                    "Pane" => quote! { workflow_ux::layout::ElementLayoutStyle::Pane },
                    "Panel" => quote! { workflow_ux::layout::ElementLayoutStyle::Panel },
                    "Stage" => quote! { workflow_ux::layout::ElementLayoutStyle::Stage },
                    "Group" => quote! { workflow_ux::layout::ElementLayoutStyle::Group },
                    _ => {
                        return Error::new_spanned(
                            field.type_name.clone(),
                            format!("struct type is not supported")
                        )
                        .to_compile_error()
                        .into();
                    }
                };

                let type_name_path = &field.type_name_path.clone().unwrap();
                field_initializers.push(quote!{
                    let #field_name = {
                        let mut attributes = HashMap::new();
                        let attr_list : Vec<(String,String)> = vec![#(( #attrs_k.to_string(),#attrs_v.to_string() ) ), *];
                        println!("********* ATTRIBUTE LIST: {:#?}",attr_list);
                        for (k,v) in attr_list.iter() {
                            attributes.insert(k.to_string(),v.clone());
                        }
                        println!("********* ATTRIBUTE MAP: {:#?}",attributes);
                        let docs : Vec<&str> = vec![#( #docs ), *];
                        let #field_name = #type_name_path::new(&layout,&attributes,&docs)?;  // pane-ctl
                        let child = #field_name.element();
                        layout.append_child(&child,&attributes,&docs)?;
                        #field_name
                    };
                });
        
            }, // ~ CONTROLS
            None => {
                field_initializers.push(quote!{
                    let #field_name = {
                        let mut attributes = HashMap::new();
                        let attr_list : Vec<(String,String)> = vec![#(( #attrs_k.to_string(),#attrs_v.to_string() ) ), *];
                        println!("********* ATTRIBUTE LIST: {:#?}",attr_list);
                        for (k,v) in attr_list.iter() {
                            attributes.insert(k.to_string(),v.clone());
                        }
                        println!("********* ATTRIBUTE MAP: {:#?}",attributes);
                        let docs : Vec<&str> = vec![#( #docs ), *];
                        let #field_name = #type_name::new(&layout,&attributes,&docs)?;  // pane-ctl
                        let child = #field_name.element();
                        layout.append_child(&child,&attributes,&docs)?;
                        #field_name
                    };
                });
        
            }
        }
*/
    }

    // println!("******************************** FIELD DONE XX");




    let mut field_idents_str:Vec<String> = vec![];
    let mut field_idents : Vec<Ident> = fields.iter().map(|f| {
        field_idents_str.push(f.field_name.to_string());
        f.field_name.clone()
    }).collect();

    // let pane_ident = match pane_attributes.get("ident") {
    //     Some(ident) => ident.to_token_stream(), //clone(),
    //     _ => { 
    //         return Error::new_spanned(
    //             struct_name,
    //             format!("missing ident in pane attributes")
    //         )
    //         .to_compile_error()
    //         .into();
    //     }
    // };

    let _layout_title = match layout_attributes.get("title") {
        Some(Some(ident)) => ident.to_token_stream(), //clone(),
        _ => { 

            match layout {
                Layout::Form => quote! { "" },
                Layout::Section => { quote! { 
                    return Error::new_spanned(
                        struct_name,
                        format!("missing title in layout attributes")
                    )
                    .to_compile_error()
                    .into();

                }},
                Layout::Stage => quote! { "" },
                Layout::Pane => quote! { "" },
                Layout::Panel => quote! { "" },
                Layout::Page => quote! { "" },
                Layout::Group => quote! { "" },
                Layout::Html => quote! { "" },
            }
        }
    };
    //println!("\n layout: {:?} _layout_title: {}", layout, _layout_title);

    let mut init_helper = quote!{};
    let mut init_helper_def = quote!{};
    let mut init_extra_props = quote!{};
    let mut init_extra_props_def = quote!{};
    let mut layout_loading = quote!{};
    let mut layout_binding = quote!{};
    let impl_def = quote!{
        unsafe impl #struct_params Send for #struct_name #struct_params{}
        unsafe impl #struct_params Sync for #struct_name #struct_params{}
    };
    let layout_style = match layout {
        Layout::Form => {
            layout_loading = quote! {layout.load().await?;};

            init_extra_props = quote! {
                pub _footer: workflow_ux::form_footer::FormFooter,
            };
            field_idents.push(Ident::new("_footer", Span::call_site()));
            field_initializers.push(quote! {
                let mut _footer = {
                    let mut layout_attributes = Attributes::new();
                    let mut ctl_attributes = Attributes::new();
                    let docs: Vec<&str> = vec![];
                    let footer = workflow_ux::form_footer::FormFooter::new(&_layout, &ctl_attributes, &docs)?;
                    let child = footer.element();
                    _layout.append_child(&child, &layout_attributes, &docs)?;
                    footer
                };
            });

            layout_binding = quote! ({
                let layout_clone = view.layout();
                let mut locked = layout_clone.lock().expect(&format!("Unable to lock form {} for footer binding.", #struct_name_string));
                locked._footer.bind_layout(#struct_name_string.to_string(), view.clone())?;
            });

            init_helper_def = quote!{
                pub fn set_submit_btn_text<T:Into<String>>(&self, text:T)->workflow_ux::result::Result<()>{
                    self._footer.set_submit_btn_text(text)?;
                    Ok(())
                }
            };

            //impl_def = quote!{
            //    unsafe impl #struct_params Send for #struct_name #struct_params{}
            //};

            quote! { workflow_ux::layout::ElementLayoutStyle::Form }
        },
        Layout::Section => quote! { workflow_ux::layout::ElementLayoutStyle::Section },
        Layout::Stage => {
            init_helper = quote! { 
                layout.init_footer()?;
                layout.set_stage_index(0)?;
            };
            init_extra_props = quote! {
                _stage_index:usize,
                _stages:Vec<&'static str>,
            };

            init_extra_props_def = quote! {
                _stage_index: 0,
                _stages: Vec::from([ #( #field_idents_str ),* ]),
            };

            init_helper_def = quote! {
                pub fn validate_stage(&self)->workflow_ux::result::Result<bool>{
                    let stage_name = self._stages[self._stage_index];
                    match stage_name{
                        #( #field_idents_str => {
                            return Ok(self.#field_idents.validate_stage()?);
                            /*
                            let result = self.#field_idents.validate_stage()?;
                            log_trace!("validate_stage: result: {:?}", result);
                            return Ok(result);
                            */
                        } ),*

                        _=>{}
                    };
                    /*
                    if let Some(stage) = stage_result{
                        
                    };

                    log_trace!("validate_stage:stage_name: {:?}", stage_name);*/
                    Ok(true)
                }
                pub fn update_stage_visibility(&self)-> workflow_ux::result::Result<bool> {
                    /*
                    let last_stage_name = self._stages[old_index];
                    match last_stage_name{
                        #( #field_idents_str => {
                            self.#field_idents.element().set_attribute("hide", "true");
                        } ),*

                        _=>{}
                    };
                    */

                    let next_stage_name = self._stages[self._stage_index];
                    let mut result = false;

                    #(
                        if !#field_idents_str.eq(next_stage_name) {
                            self.#field_idents.element().set_attribute("hide", "true")?;
                        }
                    )*

                    
                    match next_stage_name{
                        #( #field_idents_str => {
                            self.#field_idents.element().remove_attribute("hide")?;
                            result = true;
                        } ),*

                        _=>{}
                    };

                    Ok(result)
                }
                pub fn show_prev_stage(&mut self) -> workflow_ux::result::Result<bool>{
                    //if !self.validate_stage()?{
                    //    return Ok(false);
                    //}
                    if self._stage_index == 0{
                        Ok(false)
                    }else{
                        Ok(self.set_stage_index(self._stage_index - 1)?)
                    }
                }
                pub fn show_next_stage(&mut self) -> workflow_ux::result::Result<bool>{
                    if !self.validate_stage()?{
                        return Ok(false);
                    }
                    //if this is last stage
                    if self._stage_index+1 == self._stages.len(){
                        Ok(false)
                    }else{
                        Ok(self.set_stage_index(self._stage_index + 1)?)
                    }
                }
                pub fn set_stage_index(&mut self, index:usize) -> workflow_ux::result::Result<bool>{
                    self._stage_index = index;

                    let footer = self._layout.get_stage_footer()?;
                    if self._stage_index == 0 {
                        footer.disable_btn("previous");
                    }else{
                        footer.enable_btn("previous");
                    }

                    //if this is last stage
                    if self._stage_index+1 == self._stages.len() {
                        footer.show_btn("submit");
                    }else{
                        footer.hide_btn("submit");
                        footer.show_btn("next");
                    }

                    Ok(self.update_stage_visibility()?)
                }
                pub fn init_footer(&self) -> workflow_ux::result::Result<()> {
                    self._layout.init_footer()?;
                    let footer = self._layout.get_stage_footer()?;
                    /*
                    let element = self._layout.root_element();
                    let footer_node = element.query_selector("workflow-stage-footer")?
                        .ok_or("ElementLayout::init_footer() - failure to find stage footer")?;
                    let footer = footer_node.dyn_into::<workflow_ux::controls::stage_footer::StageFooter>()?;
                    */
                    //trace!("footer: {:#?}", footer);
                    
                    //trace!("stages: {:#?}, length:{}", stages, stages.len());
                    //let len = self._stages.len();
                    //let mut stage_index = 0;

                    let mut this = self.clone();
                    let closure = Closure::wrap(Box::new(move |event: workflow_ux::controls::stage_footer::StageFooterBtnEvent| {
                        
                        //trace!("footer button click: {:#?}", event);
                        let btn = event.btn();
                        //trace!("footer button click event, btn: {:?}", btn);
                        match btn.as_str(){
                            "next"=>{
                                workflow_log::log_trace!("footer next btn clicked");
                                this.show_next_stage().expect("footer next btn click failed");
                            },
                            "previous"=>{
                                workflow_log::log_trace!("footer previous btn clicked");
                                this.show_prev_stage().expect("footer prev btn click failed");
                            },
                            "submit"=>{
                                workflow_log::log_trace!("footer submit btn clicked");
                                this.submit().expect("footer submit btn click failed");
                            },
                            _=>{

                            }
                        }

                    }) as Box<dyn FnMut(_)>);
                    footer.add_event_listener_with_callback("btn-click", closure.as_ref().unchecked_ref())?;
                    closure.forget();

                    Ok(())
                }

                pub fn update_footer(&self, attributes: &Attributes) -> workflow_ux::result::Result<()> {
                    Ok(self._layout.update_footer(attributes)?)
                }
            };

            quote! { workflow_ux::layout::ElementLayoutStyle::Stage }
        },
        Layout::Pane => quote! { workflow_ux::layout::ElementLayoutStyle::Pane },
        Layout::Panel => quote! { workflow_ux::layout::ElementLayoutStyle::Panel },
        Layout::Page => quote! { workflow_ux::layout::ElementLayoutStyle::Page },
        Layout::Group => quote! { workflow_ux::layout::ElementLayoutStyle::Group },
        Layout::Html => quote! { workflow_ux::layout::ElementLayoutStyle::Html },
    };

    // let attrs_kv: Vec<(String,String)> = Vec::new();
    let attrs_kv = layout_attributes.to_string_kv();
    // let attrs_k : Vec<TokenStream> = attrs_kv.iter().map(|item| {
    let attrs_k : Vec<String> = attrs_kv.iter().map(|item| {
        let v = item.0.clone();
        let ts : TokenStream = str::parse::<TokenStream>(&v).unwrap();
        ts.to_string()
    }).collect();
    // let attrs_v : Vec<TokenStream> = attrs_kv.iter().map(|item|item.1.clone()).collect();
    let attrs_v : Vec<String> = attrs_kv.iter().map(|item|item.1.to_string()).collect();


    // let struct_path_with_generics = if has_generics {
    //     quote!{ #struct_name::#generics_only }
    // } else {
    //     quote!{ #struct_name }
    // };

    // let struct_path_with_params = if struct_params.params.is_empty() {
    //     quote!{ #struct_name }
    // } else {
    //     quote!{ #struct_name::#struct_params }
    // };

    let declare_ctors = match layout {
        Layout::Html => quote! {


            pub fn from_html(
                html : &workflow_html::Html,
                // html : &(Vec<web_sys::Element>, std::collections::BTreeMap<String, web_sys::Element>)
                // this should be eliminated
                // parent_el_to_be_removed_when_html_is_fed_directly_here: &web_sys::Element, 
                // // this tree should be direct output of html! macro to execute render_html() on
                // html_tree : std::collections::BTreeMap<&'static str, web_sys::Element>
            ) -> workflow_ux::result::Result<#struct_name #struct_params> {

                // let (roots, hooks) = html;
                let mut hooks_used : workflow_ux::hash::HashSet<String> = html.hooks().keys().cloned().collect();

                // let attr_list : Vec<(String,String)> = vec![#(( #attrs_k.to_string(),#attrs_v.to_string() ) ), *];
                // let mut attributes = Attributes::new();
                // for (k,v) in attr_list.iter() {
                //     attributes.insert(k.to_string(),v.clone());
                // }

                // let layout_style = #layout_style;
                // let layout = ElementLayout::new(layout, layout_style, &attributes)?;
                let _layout = ElementLayout::try_new_for_html()?;

                // parent_el_to_be_removed_when_html_is_fed_directly_here.append_child(&_layout.element())?;

                html.inject_into(&_layout.element())?;
                // for el in roots.iter() {
                //     _layout.element().append_child(el)?;
                // }
                // let _layout = ElementLayout::new(parent_layout, layout_style, &attributes)?;

                #( #field_initializers ) *

                if !hooks_used.is_empty() {
                    let unused = hooks_used.into_iter().collect::<Vec<String>>().join(",");
                    workflow_log::log_error!("{}",workflow_ux::error::Error::MissingLayoutBindings(#struct_name_string.into(), unused));
                }

                // let el = #struct_name :: #struct_params{
                let mut layout = #struct_name {
                    _layout,
                    // #init_extra_props_def
                    #( #field_idents ),*
                };

                layout.init()?;

                // #init_helper

                Ok(layout)

            }



        },
        _ => quote! {

            pub async fn try_create_layout_view(
                module : Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>>
            ) -> workflow_ux::result::Result<std::sync::Arc<workflow_ux::view::Layout<Self, ()>>> {
                Ok(Self::try_create_layout_view_with_data(module, Option::<()>::None).await?)
            }
            
            pub async fn try_create_layout_view_with_data<D:Send + 'static>(
                module : Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>>,
                data: Option<D>
            ) -> workflow_ux::result::Result<std::sync::Arc<workflow_ux::view::Layout<Self, D>>> {
                let el = workflow_ux::document().create_element("div")?;
                let mut layout = Self::try_inject(&el)?;
                #layout_loading
                let view = workflow_ux::view::Layout::try_new(module, layout, data)?;
                #layout_binding
                Ok(view)
            }
            pub fn try_new()-> workflow_ux::result::Result<#struct_name #struct_params> {
                let el = workflow_ux::document().create_element("div")?;
                let layout = Self::try_inject(&el)?;
                Ok(layout)
            }
            
            pub fn try_inject(parent: &web_sys::Element) -> workflow_ux::result::Result<#struct_name #struct_params> {
                let root = ElementLayout::try_inject(parent, #layout_style)?; 
                let attributes = Attributes::new();
                let docs = Docs::new();

                Ok(#struct_name::new(&root, &attributes, &docs)?)
            }

            pub fn new(parent_layout : &ElementLayout, attributes: &Attributes, docs : &Docs) -> workflow_ux::result::Result<#struct_name #struct_params> {

                let attr_list : Vec<(String,String)> = vec![#(( #attrs_k.to_string(),#attrs_v.to_string() ) ), *];
                let mut attributes = Attributes::new();
                for (k,v) in attr_list.iter() {
                    attributes.insert(k.to_string(),v.clone());
                }

                let layout_style = #layout_style;
                // let layout = ElementLayout::new(layout, layout_style, &attributes)?;
                let _layout = ElementLayout::new(parent_layout, layout_style, &attributes)?;

                #( #field_initializers ) *

                // let el = #struct_name :: #struct_params{
                let mut layout = #struct_name {
                    _layout,
                    #init_extra_props_def
                    #( #field_idents ),*
                };

                layout.init()?;

                #init_helper

                Ok(layout)
            }


        }
    };
    


    let ts = quote!{

        pub struct #struct_name #struct_params{
            _layout : ElementLayout,
            #init_extra_props
            #( #fields ),*
        }

        #impl_def

        impl #struct_params   #struct_name #struct_params{

            // pub fn element(&self) -> Element {
            //     self._layout.element()
            // }

            pub fn show(&self, show:bool) -> workflow_ux::result::Result<()> {
                let el = self._layout.element();
                if show{
                    Ok(el.remove_attribute("hidden")?)
                }else{
                    Ok(el.set_attribute("hidden", "true")?)
                }
            }

            #declare_ctors

            // pub fn show(&self, show:bool) -> workflow_ux::result::Result<()> {
            //     let el = self._layout.element();
            //     if show{
            //         return el.remove_attribute("hidden");
            //     }else{
            //         return el.set_attribute("hidden", "true");
            //     }
            // }
            
            pub fn layout(&self) -> ElementLayout {
                self._layout.clone()
            }

            #init_helper_def

            // pub fn test() -> bool { true }

            // pub fn ident() -> &'static str {
            //     #pane_ident
            // }
        }

        impl #struct_params   workflow_ux::layout::DefaultFunctions for #struct_name #struct_params{}

        impl #struct_params   workflow_ux::layout::Elemental for #struct_name #struct_params{
            fn element(&self) -> web_sys::Element {
                self._layout.element()
            }
        }

        impl #struct_params   Clone for #struct_name #struct_params{
            fn clone(&self) -> #struct_name #struct_params {
                #struct_name{
                    _layout : self._layout.clone(),
                    #init_extra_props_def
                    #( #field_idents : self.#field_idents.clone()),*
                }

            }
        }

        impl #struct_params   Into<Element> for #struct_name #struct_params{
            fn into(self) -> Element {
                self._layout.element()//.expect("Into<Element>::into() unable to load layout inner")
                    // .inner()
                    // .ok_or(&format!("unable to lock layout inner"))?
                    // .element
                    // .clone()
            }
        }

    }.into();

    // println!("******************************** FINISHED QUOTING");


    ts
}








