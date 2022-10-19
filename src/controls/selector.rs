use workflow_ux::prelude::*;
use workflow_ux::result::Result;
use std::{convert::Into, marker::PhantomData};


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
    pub element_wrapper : ElementWrapper,
    value : Arc<Mutex<String>>,
    p:PhantomData<E>,
    on_change_cb:Arc<Mutex<Option<CallbackNoArgs>>>,
}

impl<E> Selector<E>
where E: EnumTrait<E>+Display
{
    
    pub fn element(&self) -> FlowSelectorBase {
        self.element_wrapper.element.clone().dyn_into::<FlowSelectorBase>()
            .expect("Unable to cast element to FlowSelectorBase")
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
        let value = Arc::new(Mutex::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut control = Selector {
            element_wrapper: ElementWrapper::new(element),
            value,
            p:PhantomData,
            on_change_cb:Arc::new(Mutex::new(None))
        };

        control.init()?;

        Ok(control)
    }

    fn init(&mut self)->Result<()>{

        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper.on("select", move |event| -> Result<()> {

            log_trace!("Selector:flow select event: {:?}", event);
            let new_value = el.value();

            log_trace!("Selector:current value: {:?}", new_value);
            if let Some(op) = E::from_str(new_value.as_str()){
                log_trace!("op: {}", op);
            }
            let mut value = value.lock().unwrap();
            log_trace!("Selector:current value: {:?}", new_value);

            *value = new_value;

            if let Some(cb) =  cb_opt.lock().unwrap().as_mut(){
                return Ok(cb()?);
            }
            
            Ok(())

        })?;

        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.lock().unwrap().clone()
    }

    pub fn on_change(&self, callback:CallbackNoArgs){
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }
}
