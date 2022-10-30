extern crate self as workflow_ux;
extern crate downcast;


pub mod prelude;

pub mod error;
pub mod result;
pub mod utils;
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
pub mod dialog;
pub mod progress;
pub mod markdown;
pub mod pagination;
pub use workflow_core::{
    async_trait,
    async_trait_without_send,
    async_trait_with_send,
    workflow_async_trait
};

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

pub use utils::*;