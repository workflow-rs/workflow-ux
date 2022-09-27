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
pub mod bottom_menu;
pub mod popup_menu;
pub mod layout;
pub mod module;
pub mod application;
pub mod workspace;
pub mod app_menu;
// pub mod enums;
pub mod view;
pub mod link;
pub mod image;
pub mod form;
pub mod panel;
pub mod form_footer;
pub mod user_agent;

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

use web_sys::{
    Window,
    Document,
    Element
};

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("unable to get `document` node");
    document
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
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
    let el = doc.create_element(tag)?;

    for (name, value) in attrs{
        el.set_attribute(name, value)?;
    }

    if let Some(html) = html{
        el.set_inner_html(html);
    }

    Ok(el)
}
