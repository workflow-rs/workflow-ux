use workflow_ux::error::Error;
use workflow_ux::result::Result;
use wasm_bindgen::JsValue;
use workflow_html::{html, Html, Render};
use workflow_log::log_error;
use crate::markdown::markdown_to_html;
use crate::controls::md::MD;
use web_sys::{
    Window,
    Document,
    Element,
    Location,
    Storage,
};

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("unable to get `document` node");
    document
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn location() -> Location {
    window().location()
}

pub fn local_storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

pub fn find_el(selector:&str, error_msg:&str) -> Result<Element>{
    let el_opt = match document().query_selector(selector){
        Ok(el_opt) => el_opt,
        Err(err)=>{
            log_error!("MissingElement:error: {:?}, selector:{selector}, error_msg:{error_msg}", err);
            return Err(Error::MissingElement(error_msg.into(), selector.into()));
        }
    };
    let element = match el_opt{
        Some(el)=>el,
        None=>return Err(Error::MissingElement(error_msg.into(), selector.into() ))
    };

    Ok(element)
}

pub fn create_el(tag:&str, attrs:Vec<(&str, &str)>, html:Option<&str>) -> Result<Element>{
    let doc = document();
    let mut tag_name = tag;
    let mut classes:Option<js_sys::Array> = None;
    if tag_name.contains("."){
        let mut parts = tag_name.split(".");
        let tag = parts.next().unwrap();
        let array = js_sys::Array::new();
        for a in parts{
            array.push(&JsValue::from(a));
        }
        classes = Some(array);
        tag_name = tag;
    }
    let el = doc.create_element(tag_name)?;

    for (name, value) in attrs{
        el.set_attribute(name, value)?;
    }
    if let Some(classes) = classes{
        el.class_list().add(&classes)?;
    }

    if let Some(html) = html{
        el.set_inner_html(html);
    }

    Ok(el)
}

pub fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

pub fn markdown(str:&str)->crate::result::Result<Html>{
    let body = markdown_to_html(str);
    /*
    //let stream: proc_macro2::TokenStream = str.parse().unwrap();
    //let str = format!("{}", str);
    let res = str_to_html!(
        {println!("{}", str)}
    );
    workflow_log::log_trace!("resresres: {}", res);
    */

    //workflow_log::log_trace!("html body: {}", body);

    //Ok(MD::new(body)?)

    Ok(html!{<MD body />}?)
}

