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
pub mod layout;
pub mod module;
pub mod application;
// pub mod enums;
pub mod view;
pub mod link;
pub mod image;
pub mod form;

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
    Document
};

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("unable to get `document` node");
    document
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}
