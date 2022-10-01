pub use std::sync::{Arc, Mutex};
pub use async_std::sync::RwLock;
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use std::fmt::{Display, Debug};
pub use std::collections::HashMap;
pub use std::marker::PhantomData;
// pub use workflow_allocator::prelude::*;
// pub use workflow_allocator::utils::generate_random_pubkey;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
// pub use workflow_ux::*;
// pub use crate::dom::{document,get_element_by_id};
pub use workflow_i18n::i18n;
pub use workflow_log::{log_trace,log_warning,log_error};
pub use crate::{document,window};
pub use crate::theme::*;
pub use crate::control::{Control,ElementBindingContext};
pub use crate::form::{FormHandlers, FormResult};
pub use crate::controls::helper::FieldHelper;

// TODO review and namespace all controls
pub use crate::controls::*;
pub use element_wrapper::ElementWrapper;

pub use crate::controls::form::{FormControl, FormControlBase};
// pub use crate::controls::terminal::Terminal;

// TODO merge with Control
pub use crate::layout::Elemental;

pub use crate::layout::ElementLayout;
pub use crate::layout::ElementLayoutStyle;
pub use crate::layout::DefaultFunctions;
pub use crate::controls::base_element::BaseElement;
pub use crate::controls::select::FlowMenuBase;
pub use crate::create_el;
pub use web_sys::{
    Document,
    Element,
    HtmlElement,
    HtmlLinkElement,
    HtmlImageElement,
    HtmlInputElement,
    HtmlHrElement,
    CustomEvent,
    EventTarget,
    Node,
    MutationObserver,
    MutationObserverInit,
    MutationRecord,
    SvgElement,
    SvgPathElement
};
pub use workflow_core::enums::EnumTrait;
pub use workflow_core::id::Id;
pub use crate::menu::{MenuItem,MenuGroup,SectionMenu};
pub use crate::app_menu;
pub use crate::popup_menu::PopupMenu;
pub use crate::module::{Module,ModuleInterface};
pub use crate::attributes::Attributes;
pub use crate::docs::Docs;
pub use crate::view;
pub use crate::bottom_menu::{BottomMenu, BottomMenuItem};
pub use crate::workspace;
pub use crate::view::Container;
pub use crate::find_el;
pub use crate::panel::*;


pub use crate::application::global as application;

pub use async_trait::async_trait;

pub use workflow_ux_macros::Module;
pub use workflow_ux_macros::declare_module;


pub type Callback<E> = Box<dyn FnMut(E)->crate::result::Result<()>>;
pub type CallbackNoArgs = Box<dyn FnMut()->crate::result::Result<()>>;
pub type OptionalCallback<T> = Rc<RefCell<Option<Callback<T>>>>;
pub type OptionalCallbackNoArgs = Rc<RefCell<Option<CallbackNoArgs>>>;
