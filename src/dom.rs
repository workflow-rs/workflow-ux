use crate::prelude::*;
use ahash::AHashMap;
use std::{str::FromStr, sync::Arc};
use thiserror::Error;
use wasm_bindgen::JsCast;
use workflow_core::id::Id;
use workflow_wasm::prelude::*;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Js Error: {0}")]
    JsError(String),
}

pub trait Element {
    fn element(&self) -> web_sys::Element;
}

static mut DOM: Option<Dom> = None;
pub fn global() -> &'static mut Dom {
    unsafe { DOM.as_mut().unwrap() }
}

pub fn register(el: &Arc<dyn Element>) {
    let id = Id::new();
    el.element()
        .set_attribute("id", &id.to_string())
        .expect("[Dom] Unable to set element id");
    let dom = global();
    dom.elements.insert(id, el.clone());
}

pub struct Dom {
    elements: AHashMap<Id, Arc<dyn Element>>,
    dom_listener: Option<Callback<CallbackClosure<js_sys::Array>>>,
}

impl Dom {
    pub fn init() {
        let dom = Dom {
            elements: AHashMap::default(),
            dom_listener: None,
        };

        unsafe {
            DOM = Some(dom);
        }

        global().init_observer().expect("Unable to init observer");
    }

    pub fn init_observer(&mut self) -> Result<(), Error> {
        let body = document()
            .get_elements_by_tag_name("body")
            .item(0)
            .expect("Unable to get body element");

        let callback = callback!(move |array: js_sys::Array| -> Result<(), JsValue> {
            let records: Vec<MutationRecord> = array
                .iter()
                .map(|val| val.dyn_into::<MutationRecord>().unwrap())
                .collect();

            let elements = &mut global().elements;
            for record in records.iter() {
                let nodes = record.removed_nodes();
                for idx in 0..nodes.length() {
                    let node = nodes.item(idx).unwrap();
                    // let node_name = node.node_name();
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if let Some(id) = el.get_attribute("workflow-id") {
                            if let Ok(id) = Id::from_str(&id) {
                                elements.remove(&id);
                            }
                        }
                    }
                }
            }

            Ok(())
            // log_trace!("= = = = = = = = MutationObserver called : {:?}", data);
        });

        let observer = MutationObserver::new(callback.as_ref())
            .map_err(|err| Error::JsError(format!("{err:?}")))?;
        self.dom_listener = Some(callback);
        let mut options = MutationObserverInit::new();
        options.child_list(true);
        options.subtree(true);
        observer
            .observe_with_options(&body, &options)
            .map_err(|e| Error::JsError(format!("{e:?}")))?;

        Ok(())
    }

    // pub
}
