use web_sys::{EventTarget, Node};

use crate::prelude::*;
use std::convert::Into;
use workflow_ux::result::Result;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = FlowFormControl , typescript_type = "FlowFormControl")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FormControlBase` class."]
    pub type FormControlBase;


    # [wasm_bindgen (structural , method , js_class = "FormControlBase" , js_name = focus)]
    pub fn focus(this: &FormControlBase);
}
pub struct FormControl {
    pub element: Element,
}

impl FormControl {

    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new() -> Result<FormControl> {
        Ok(FormControl::new_with_id(&Id::new().to_string())?)
    }

    pub fn new_with_id(self_id : &str) -> Result<FormControl> {

        let element = document()
            .create_element("flow-form-control")?;
        element.set_id(self_id);

        //element.set_attribute("focusable", "true")?;
        Ok(FormControl {
            element: element,
        })
    }

    pub fn set_title(&self, title: &str) -> Result<()> {
        let div = document()
            .create_element("div")?;
        div.set_attribute("slot","title")?;
        div.set_inner_html(title);
        self.element.append_child(&div)?;
        Ok(())
    }
    pub fn set_attribute(&self, k:&str, v:&str) -> Result<()>{
        self.element.set_attribute(k, v)?;
        Ok(())
    }

    pub fn set_info(&self, info: &str) -> Result<()> {
        let div = document()
            .create_element("div")?;
        div.set_attribute("slot","info")?;
        div.set_inner_html(info);
        self.element.append_child(&div)?;
        Ok(())
    }

    pub fn append_child(&self, child : &Element) -> Result<()> {
        self.element.append_child(child)?;
        Ok(())
    }
}

impl Into<Element> for FormControl {
    fn into(self) -> Element {
        self.element()
    }
}