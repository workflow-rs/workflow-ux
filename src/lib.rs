extern crate self as workflow_ux;
extern crate downcast;


pub mod prelude;

pub mod error;
pub mod result;
pub mod dom;
pub mod attributes;
pub mod docs;
pub mod control;
pub mod controls;
pub mod icon;
pub mod theme;
pub mod menu;
pub use menu::app_menu;
pub use menu::main_menu;
pub use menu::popup_menu;
pub use menu::bottom_menu;
pub mod layout;
pub mod module;
pub mod application;
pub mod workspace;
// pub mod enums;
pub mod view;
pub mod link;
pub mod image;
pub mod form;
pub mod panel;
pub mod app_drawer;
pub mod form_footer;
pub mod user_agent;
pub mod task;

pub mod macros {
    pub use workflow_ux_macros::*;
}

// pub use dom::{ document, window };

pub mod hash {
    pub use ahash::AHashSet as HashSet;
    pub use ahash::AHashMap as HashMap;
}


// use std::sync::Arc;
// pub fn workspace() -> std::sync::Arc<workspace::Workspace> {
//     workflow_ux::application::global().expect("Missing global application object").workspace()
// }

use wasm_bindgen::JsValue;
use web_sys::{
    Window,
    Document,
    Element,
    Location
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

pub fn find_el(selector:&str, error_msg:&str)->std::result::Result<Element, error::Error>{
    let element = match document().query_selector(selector).expect(&error::Error::MissingElement(error_msg.into(), selector.into() ).to_string()){
        Some(el)=>el,
        None=>return Err(error::Error::MissingElement(error_msg.into(), selector.into() ))
    };

    Ok(element)
}

pub fn create_el(tag:&str, attrs:Vec<(&str, &str)>, html:Option<&str>)->std::result::Result<Element, error::Error>{
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
