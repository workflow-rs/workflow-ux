pub use std::sync::{Arc, Mutex};
// pub use async_std::sync::RwLock;
pub use crate::control::{Control, ElementBindingContext};
pub use crate::controls::helper::FieldHelper;
pub use crate::form::{FormData, FormDataValue, FormHandler};
pub use crate::theme::*;
pub use crate::{document, window};
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::fmt::{Debug, Display};
pub use std::marker::PhantomData;
pub use std::rc::Rc;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
pub use workflow_i18n::{dict as i18n_dict, i18n};
pub use workflow_log::{log_debug, log_error, log_trace, log_warning};

// TODO review and namespace all controls
pub use crate::controls::*;
pub use element_wrapper::ElementWrapper;

pub use crate::controls::form::{FormControl, FormControlBase};
// pub use crate::controls::terminal::Terminal;

// TODO merge with Control
pub use crate::layout::Elemental;

pub use crate::app_menu;
pub use crate::attributes::Attributes;
pub use crate::bottom_menu::{BottomMenu, BottomMenuItem};
pub use crate::controls::base_element::BaseElement;
pub use crate::controls::builder::{Builder, ListBuilder, ListBuilderItem, ListRow};
pub use crate::controls::select::FlowMenuBase;
pub use crate::create_el;
pub use crate::docs::Docs;
pub use crate::find_el;
pub use crate::layout::DefaultFunctions;
pub use crate::layout::ElementLayout;
pub use crate::layout::ElementLayoutStyle;
pub use crate::menu::{MenuGroup, MenuItem, SectionMenu};
pub use crate::module::{Module, ModuleInterface};
pub use crate::pagination::*;
pub use crate::panel::*;
pub use crate::popup_menu::PopupMenu;
pub use crate::progress::*;
pub use crate::qrcode;
pub use crate::view;
pub use crate::view::{Container, ContainerStack, Evict};
pub use crate::workspace;
pub use crate::{
    async_trait, async_trait_with_send, async_trait_without_send, workflow_async_trait,
};
pub use web_sys::{
    CustomEvent, Document, Element, EventTarget, HtmlElement, HtmlHrElement, HtmlImageElement,
    HtmlInputElement, HtmlLinkElement, MutationObserver, MutationObserverInit, MutationRecord,
    Node, SvgElement, SvgPathElement,
};
pub use workflow_core::enums::EnumTrait;
pub use workflow_core::id::Id;

pub use crate::application::global as application;

pub use workflow_ux_macros::declare_module;
pub use workflow_ux_macros::Module;

pub type CallbackFn<E> = Box<dyn FnMut(E) -> crate::result::Result<()>>;
pub type CallbackFnNoArgs = Box<dyn FnMut() -> crate::result::Result<()>>;
pub type OptionalCallbackFn<T> = Arc<Mutex<Option<CallbackFn<T>>>>;
pub type OptionalCallbackFnNoArgs = Arc<Mutex<Option<CallbackFnNoArgs>>>;
