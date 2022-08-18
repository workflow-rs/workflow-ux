use crate::prelude::*;
use crate::controls::Callback;
use std::{convert::Into, marker::PhantomData};
use js_sys::Array;
use workflow_ux::result::Result;


#[wasm_bindgen]
extern "C" {

    // The `FlowMultiMenuBase` class.
    # [wasm_bindgen (extends = BaseElement , js_name = FlowMenu , typescript_type = "FlowMenu")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowMultiMenuBase;

    // Getter for the `value` field of this object.
    # [wasm_bindgen (structural , method , getter , js_class = "FlowMultiMenuBase" , js_name = value)]
    pub fn value(this: &FlowMultiMenuBase) -> JsValue;

    # [wasm_bindgen (structural , method , js_class = "FlowMultiMenuBase" , js_name = select)]
    pub fn _select(this: &FlowMultiMenuBase, values:JsValue);

    // Remove old/current options and set new option
    # [wasm_bindgen (structural , method , js_class = "FlowMultiMenuBase" , js_name = changeOptions)]
    pub fn change_options(this: &FlowMultiMenuBase, options:Array);
}


impl FlowMultiMenuBase{
    pub fn select<S: ToString>(self: &FlowMultiMenuBase, selection:Vec<S>){
        let select:Vec<String> = (&selection).iter().map(|a| a.to_string()).collect();
        self._select(JsValue::from_serde(&select).unwrap());
    }
}

#[derive(Clone)]
pub struct MultiSelect<E> {
    pub element : Element,
    values : Rc<RefCell<Vec<String>>>,
    on_change_cb: Rc<RefCell<Option<Callback<Vec<String>>>>>,
    p:PhantomData<E>
}


impl<E> MultiSelect<E>
where E: EnumTrait<E>
{
    
    pub fn element(&self) -> FlowMultiMenuBase {
        self.element.clone().dyn_into::<FlowMultiMenuBase>().expect("Unable to case to FlowMultiMenuBase")
    }
    // pub fn element(&self) -> Element {
    //     self.element.clone()
    //     // self.element.clone().dyn_into::<FlowMultiMenuBase>().expect("Unable to case to FlowMultiMenuBase")
    // }
    pub fn focus(&self) -> Result<()> {
        self.element().focus_form_control()?;
        Ok(())
    }

    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<MultiSelect<E>> {
        let doc = document();
        let element = doc.create_element("flow-select")?;
            

        let mut init_value: String = String::from("");
        let items = E::list();
        for item in items.iter() {
            let menu_item = doc.create_element("flow-menu-item")?;
            menu_item.set_attribute("value", item.as_str())?;
            menu_item.set_inner_html(item.descr());
            element.append_child(&menu_item)?;
            if init_value.eq(""){
                init_value = String::from(item.as_str())
            }
        }
        
        element.set_attribute("multiple", "true")?;
        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }
        let values:Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let control = MultiSelect {
            element,
            values,
            on_change_cb:Rc::new(RefCell::new(None)),
            p:PhantomData
        };
        control.init_events()?;
        Ok(control)
    }

    fn init_events(&self) -> Result<()>{
        let el = self.element.clone().dyn_into::<FlowMultiMenuBase>().expect("init_events(): Unable to cast to FlowMultiMenuBase");
        let values = self.values.clone();
        let cb_opt = self.on_change_cb.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
            log_trace!("MultiSelect: select event: {:?}", event);
            let items:Vec<String> = el.value().into_serde().unwrap();

            log_trace!("MultiSelect: current_values: {:?}", items);
            let mut values = values.borrow_mut();
            *values = items.clone();

            match &mut*cb_opt.borrow_mut(){
                Some(cb)=>{
                    cb(items)
                },
                None=>{}
            };

        }) as Box<dyn FnMut(_)>);
        self.element.add_event_listener_with_callback("select", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    pub fn values(&self) -> Vec<String> {
        self.values.borrow().clone()
    }

    pub fn on_change(&self, callback:Callback<Vec<String>>){
        *self.on_change_cb.borrow_mut() = Some(callback);
    }
}