use wasm_bindgen::JsCast;
use workflow_ux::result::Result;
pub use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, MouseEvent, Element};
use crate::controls::listener::Listener;
use crate::controls::form::FormControlBase;

pub trait BaseElementTrait{
    fn show_form_control(&self, show:bool)->Result<()>{
        if let Some(ct) = self.closest_form_control()?{
            if show{
                ct.remove_attribute("hide")?;
            }else{
                ct.set_attribute("hide", "true")?;
            }
        }

        Ok(())
    }
    fn closest_form_control(&self)->Result<Option<FormControlBase>>;
}

impl BaseElementTrait for Element{
    fn closest_form_control(&self)->Result<Option<FormControlBase>>{
        if let Some(el) = self.closest("flow-form-control")?{
            return Ok(Some(el.dyn_into::<FormControlBase>()?));
        }

        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub struct ElementWrapper{
    pub element : Element,
    listeners: Vec<Listener<CustomEvent>>,
    click_listeners:Vec<Listener<MouseEvent>>,
}
impl ElementWrapper{
    pub fn push_listener(&mut self, listener: Listener<CustomEvent>){
        self.listeners.push(listener);
    }
    pub fn push_click_listener(&mut self, listener: Listener<MouseEvent>){
        self.click_listeners.push(listener);
    }
    pub fn new(element : Element)->Self{
        Self { element, listeners: Vec::new(), click_listeners: Vec::new() }
    }

    pub fn on<F>(&mut self, name:&str, t:F) ->Result<()>
    where
        F: FnMut(CustomEvent) ->Result<()> + 'static
    {
        let listener = Listener::new(t);
        self.element.add_event_listener_with_callback(name, listener.into_js())?;
        self.listeners.push(listener);
        Ok(())
    }

    pub fn on_click<F>(&mut self, t:F) ->Result<()>
    where
        F: FnMut(MouseEvent) -> Result<()> + 'static
    {
        let listener = Listener::new(t);
        self.element.add_event_listener_with_callback("click", listener.into_js())?;
        self.click_listeners.push(listener);
        Ok(())
    }
}
