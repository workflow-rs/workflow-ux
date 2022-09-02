use crate::prelude::*;
use crate::layout::ElementLayout;
use std::convert::Into;
use web_sys::{EventTarget, Node, Element};
use workflow_ux::result::Result;
use workflow_ux::error::Error;


#[wasm_bindgen]
extern "C" {
    // The `FlowInputBase` class.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    // *This API requires the following crate features to be activated: `Element`*
    #[wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = FlowInput , typescript_type = "FlowInput")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowInputBase;
    // Getter for the `namespaceURI` field of this object.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)
    // *This API requires the following crate features to be activated: `Element`*
    #[wasm_bindgen (structural , method , getter , js_class = "Element" , js_name = value)]
    pub fn value(this: &FlowInputBase) -> String;
}

#[derive(Clone)]
pub struct Input {
    pub layout : ElementLayout,
    pub element : FlowInputBase,
    value : Rc<RefCell<String>>,
}


impl Input {
    
    pub fn element(&self) -> FlowInputBase {
        self.element.clone().dyn_into::<FlowInputBase>().expect("Unable to cast element to FlowInputBase")
    }

    pub fn new(
        layout : &ElementLayout,
        attributes: &Attributes,
        docs : &Docs
    ) -> Result<Input> {
        let element = document()
            .create_element("flow-input")?
            .dyn_into::<FlowInputBase>()
            .expect("Unabel to cast Input into FlowInputBase");

        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        Ok(Self::create(element, layout.clone(), attributes, docs, String::from(""))?)
    }
    fn create(
        element: FlowInputBase,
        layout : ElementLayout,
        attributes: &Attributes,
        _docs : &Docs,
        init_value: String
    ) -> Result<Input> {

        element.set_attribute("value", init_value.as_str())?;
        element.set_attribute("label", "Input")?;
        element.set_attribute("placeholder", "Please enter")?;

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }


        let value = Rc::new(RefCell::new(init_value));
        
        // ~~~
        {
            let el = element.clone();
            let value = value.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {

                log_trace!("received changed event: {:?}", event);
                let current_element_value = el.value();
                log_trace!("#####current value: {:?}", current_element_value);
                let mut value = value.borrow_mut();
                log_trace!("current value: {:?}", current_element_value);

                *value = current_element_value;

            }) as Box<dyn FnMut(_)>);
            element.add_event_listener_with_callback("changed", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // ~~~
        {
            let el = element.clone();
            let value = value.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyEvent| {

                log_trace!("received key event: {:#?}", event);
                let current_element_value = el.value();
                log_trace!("current value: {:?}", current_element_value);
                let mut value = value.borrow_mut();
                log_trace!("current value: {:?}", current_element_value);

                *value = current_element_value;

            }) as Box<dyn FnMut(_)>);
            element.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
            element.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // ~~~

        Ok(Input { 
            layout,
            element,
            value,
        })
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }
    pub fn set_value(&self, value: String) -> Result<()>{
        self.element.set_attribute("value", &value)?;
        (*self.value.borrow_mut()) = value;
        Ok(())
    }
}


impl<'refs> TryFrom<ElementBindingContext<'refs>> for Input {
    type Error = Error;

    fn try_from(ctx : ElementBindingContext<'refs>) -> Result<Self> {
        Ok(Self::create(
            ctx.element.clone().dyn_into::<FlowInputBase>()?,
            ctx.layout.clone(),
            &ctx.attributes,
            &ctx.docs,
            String::new()
        )?)

    }
}