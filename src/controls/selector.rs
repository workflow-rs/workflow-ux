use workflow_ux::prelude::*;
use workflow_ux::result::Result;
// use crate::layout::ElementLayout;
use std::{convert::Into, marker::PhantomData};
use web_sys::{Element, EventTarget, Node};
use std::fmt::Debug;


#[wasm_bindgen]
extern "C" {
    // The `FlowSelectorBase` class.
    #[wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = FlowSelector , typescript_type = "FlowSelector")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowSelectorBase;

    // Getter for the `value` field of this object.
    #[wasm_bindgen (structural , method , getter , js_class = "FlowSelectorBase" , js_name = value)]
    pub fn value(this: &FlowSelectorBase) -> String;
}

#[derive(Clone)]
pub struct Selector<E> {
    pub element : Element,
    value : Rc<RefCell<String>>,
    p:PhantomData<E>
}

impl<E> Selector<E>
where E: EnumTrait<E>+Display
{
    
    pub fn element(&self) -> Element {
        // Ok(self.element.clone().dyn_into::<Element>()?)
        self.element.clone()
    }

    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Selector<E>> {
        let doc = document();
        let element = doc
            .create_element("flow-selector")?;
        
        let mut item_type = "flow-menu-item";
        let mut value_prop = "value";
        for (k,v) in attributes.iter() {
            if k.eq("item_type"){
                item_type = v.as_str();
            }else if k.eq("value_prop"){
                value_prop = v.as_str();
            }
        }

        let mut init_value: String = String::from("");
        let items = E::list();
        for item in items.iter() {
            let menu_item = doc.create_element(item_type)?;
            menu_item.set_attribute(value_prop, item.as_str())?;
            menu_item.set_inner_html(item.descr());
            element.append_child(&menu_item)?;
            if init_value.eq(""){
                init_value = String::from(item.as_str())
            }
        }
        
        element.set_attribute("selected", init_value.as_str())?;
        element.set_attribute("label", "Menu")?;
        element.set_attribute("valueattr", value_prop)?;

        for (k,v) in attributes.iter() {
            if k.eq("item_type") || k.eq("value_prop"){
                continue;
            }
            element.set_attribute(k,v)?;
        }
        let value = Rc::new(RefCell::new(init_value));

        {
            let el = element
                .clone()
                .dyn_into::<FlowSelectorBase>()
                .expect("Unable to cast element to FlowSelectorBase");
            let value = value.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {

                
                log_trace!("Selector:flow select event: {:?}", event);
                let current_element_value = el.value();

                log_trace!("Selector:current value: {:?}", current_element_value);
                if let Some(op) = E::from_str(current_element_value.as_str()){
                    log_trace!("op: {}", op);
                }
                let mut value = value.borrow_mut();
                log_trace!("Selector:current value: {:?}", current_element_value);

                *value = current_element_value;
                

            }) as Box<dyn FnMut(_)>);
            element.add_event_listener_with_callback("select", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        Ok(Selector {
            element,
            value,
            p:PhantomData
        })
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }
}
