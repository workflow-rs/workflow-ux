extern crate downcast;
extern crate self as workflow_ux;

pub mod prelude;

pub mod attributes;
pub mod control;
pub mod controls;
pub mod docs;
pub mod dom;
pub mod error;
pub mod icon;
pub mod menu;
pub mod result;
pub mod theme;
pub mod utils;
pub use menu::app_menu;
pub use menu::bottom_menu;
pub use menu::main_menu;
pub use menu::popup_menu;
pub mod app;
pub mod application;
pub mod form;
pub mod image;
pub mod layout;
pub mod link;
pub mod module;
pub mod panel;
pub mod view;
pub mod wasm;
pub mod workspace;
pub use app::layout as app_layout;
pub mod dialog;
pub mod events;
pub mod form_footer;
pub mod markdown;
pub mod pagination;
pub mod progress;
pub mod qrcode;
pub mod style;
pub mod task;
pub mod user_agent;
pub use workflow_async_trait::{async_trait, async_trait_with_send, async_trait_without_send};

/// dynamically configured re-export of async_trait as workflow_async_trait
/// that imposes `Send` restriction in native (non-WASM) and removes `Send`
/// restriction in WASM builds.
#[cfg(not(target_arch = "wasm32"))]
pub use workflow_async_trait::async_trait_with_send as workflow_async_trait;
/// dynamically configured re-export of async_trait as workflow_async_trait
/// that imposes `Send` restriction in native (non-WASM) and removes `Send`
/// restriction in WASM builds.
#[cfg(target_arch = "wasm32")]
pub use workflow_async_trait::async_trait_without_send as workflow_async_trait;

pub mod macros {
    pub use workflow_ux_macros::*;
}

pub mod hash {
    pub use ahash::AHashMap as HashMap;
    pub use ahash::AHashSet as HashSet;
}

// use std::sync::Arc;
// pub fn workspace() -> std::sync::Arc<workspace::Workspace> {
//     workflow_ux::application::global().expect("Missing global application object").workspace()
// }

pub use utils::*;
