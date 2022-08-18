use crate::prelude::*;
use std::marker::PhantomData;
use web_sys::{Element, EventTarget, Node};
use std::fmt::Debug;
use workflow_ux::result::Result;


#[wasm_bindgen]
extern "C" {
    // The `FlowRadiosBase` class.
    #[wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = FlowRadios , typescript_type = "FlowRadios")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowRadiosBase;

    // Getter for the `value` field of this object.
    #[wasm_bindgen (structural , method , getter , js_class = "FlowRadiosBase" , js_name = value)]
    pub fn value(this: &FlowRadiosBase) -> String;
}

#[derive(Clone)]
pub struct Radios<E> {
    pub element : Element,
    value : Rc<RefCell<String>>,
    p:PhantomData<E>
}

impl<E> Radios<E>
where E: EnumTrait<E> + Display
{
    
    pub fn element(&self) -> Element {
        self.element.clone()
        // Ok(self.element.clone().dyn_into::<Element>()?)
    }

    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Radios<E>> {
        let doc = document();
        let element = doc
            .create_element("flow-radios")?;
            

        let mut init_value: String = String::from("");
        let items = E::list();
        for item in items.iter() {
            let radio = doc.create_element("flow-radio")?;
            radio.set_attribute("inputvalue", item.as_str())?;
            radio.set_inner_html(item.descr());
            element.append_child(&radio)?;
            if init_value.eq(""){
                init_value = String::from(item.as_str())
            }
        }
        
        element.set_attribute("selected", init_value.as_str())?;

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }
        let value = Rc::new(RefCell::new(init_value));

        {
            let el = element.clone().dyn_into::<FlowRadiosBase>().expect("Unable to cast to FlowRadioBase");
            let value = value.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {

                
                log_trace!("flow radio changed event: {:?}", event);
                let detail = event.detail();
                log_trace!("flow radio changed event detail: {:?}", detail);
                
                let current_element_value = el.value();

                log_trace!("#####current value: {:?}", current_element_value);
                if let Some(op) = E::from_str(current_element_value.as_str()){
                    log_trace!("op: {}", op);
                }
                let mut value = value.borrow_mut();
                log_trace!("current value: {:?}", current_element_value);

                *value = current_element_value;
                

            }) as Box<dyn FnMut(_)>);
            element.add_event_listener_with_callback("select", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        Ok(Radios {
            element,
            value,
            p:PhantomData
        })
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }
}