use crate::controls::form::FormControlBase;
pub use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, Element, MouseEvent};
use workflow_ux::result::Result;
use workflow_wasm::callback::CallbackMap;
use workflow_wasm::prelude::callback;

pub trait BaseElementTrait {
    fn show_form_control(&self, show: bool) -> Result<()> {
        if let Some(ct) = self.closest_form_control()? {
            if show {
                ct.remove_attribute("hide")?;
            } else {
                ct.set_attribute("hide", "true")?;
            }
        }

        Ok(())
    }
    fn closest_form_control(&self) -> Result<Option<FormControlBase>>;
}

impl BaseElementTrait for Element {
    fn closest_form_control(&self) -> Result<Option<FormControlBase>> {
        if let Some(el) = self.closest("flow-form-control")? {
            return Ok(Some(el.dyn_into::<FormControlBase>()?));
        }

        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub struct ElementWrapper {
    pub element: Element,
    pub callbacks: CallbackMap,
}
impl ElementWrapper {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            callbacks: CallbackMap::new(),
        }
    }

    pub fn on<F>(&mut self, name: &str, t: F) -> Result<()>
    where
        F: FnMut(CustomEvent) -> Result<()> + 'static,
    {
        let callback = callback!(t);
        self.element
            .add_event_listener_with_callback(name, callback.as_ref())?;
        self.callbacks.insert(callback)?;
        Ok(())
    }

    pub fn on_click<F>(&mut self, t: F) -> Result<()>
    where
        F: FnMut(MouseEvent) -> Result<()> + 'static,
    {
        let callback = callback!(t);
        self.element
            .add_event_listener_with_callback("click", callback.as_ref())?;
        self.callbacks.insert(callback)?;
        Ok(())
    }
}
