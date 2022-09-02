use crate::prelude::*;
use std::marker::PhantomData;
use web_sys::{Element, EventTarget, Node};
use std::fmt::Debug;
use workflow_ux::result::Result;

#[wasm_bindgen]
extern "C" {
    // The `FlowRadioBtnsBase` class.
    #[wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = FlowRadioBtns , typescript_type = "FlowRadioBtns")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowRadioBtnsBase;

    // Getter for the `value` field of this object.
    #[wasm_bindgen (structural , method , getter , js_class = "FlowRadioBtnsBase" , js_name = value)]
    pub fn value(this: &FlowRadioBtnsBase) -> String;
}

#[derive(Clone)]
pub struct RadioBtns<E> {
    pub element : Element,
    value : Rc<RefCell<String>>,
    on_change_cb:Rc<RefCell<Option<Callback<E>>>>,
    p:PhantomData<E>
}

impl<E> RadioBtns<E>
where E: EnumTrait<E>+'static+Display
{
    pub fn element(&self) -> Element {
        self.element.clone()
        // Ok(self.element.clone().dyn_into::<Element>()?)
    }

    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<RadioBtns<E>> {
        let doc = document();
        let element = doc
            .create_element("flow-radio-btns")?;
            
        let mut init_value: String = String::from("");
        let items = E::list();
        for item in items.iter() {
            let radio = doc.create_element("flow-radio-btn")?;
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

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let btns = RadioBtns::<E>{
            element,
            value,
            on_change_cb:Rc::new(RefCell::new(None)),
            p:PhantomData
        };

        btns.init()?;

        Ok(btns)
    }

    fn init(&self)-> Result<()>{
        let el = self.element.clone().dyn_into::<FlowRadioBtnsBase>().expect("Unable to cast to FlowRadioBtnsBase");
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
            let detail = event.detail();
            log_trace!("flow radio changed event detail: {:?}", detail);
            
            let current_element_value = el.value();

            if let Some(variant) = E::from_str(current_element_value.as_str()){
                log_trace!("variant: {}", variant);

                let mut value = value.borrow_mut();
                log_trace!("new value: {:?}, old value: {}", current_element_value, value);

                *value = current_element_value;

                if let Some(cb) =  &mut*cb_opt.borrow_mut(){
                    cb(variant);
                };
            }
        }) as Box<dyn FnMut(_)>);
        self.element.add_event_listener_with_callback("select", closure.as_ref().unchecked_ref())?;
        closure.forget();
    
        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn on_change(&self, callback:Callback<E>){
        *self.on_change_cb.borrow_mut() = Some(callback);
    }
}
