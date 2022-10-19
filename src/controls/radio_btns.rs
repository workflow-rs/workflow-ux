use crate::prelude::*;
use std::marker::PhantomData;
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
    pub element_wrapper : ElementWrapper,
    value : Arc<Mutex<String>>,
    on_change_cb:Arc<Mutex<Option<Callback<E>>>>,
    p:PhantomData<E>
}

impl<E> RadioBtns<E>
where E: EnumTrait<E>+'static+Display
{
    pub fn element(&self) -> FlowRadioBtnsBase {
        self.element_wrapper.element.clone().dyn_into::<FlowRadioBtnsBase>().expect("Unable to cast Element to FlowRadioBtnsBase")
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
        let value = Arc::new(Mutex::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut btns = RadioBtns::<E>{
            element_wrapper: ElementWrapper::new(element),
            value,
            on_change_cb:Arc::new(Mutex::new(None)),
            p:PhantomData
        };

        btns.init()?;

        Ok(btns)
    }

    fn init(&mut self)-> Result<()>{
        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper.on("select", move |event| -> Result<()> {
            let detail = event.detail();
            log_trace!("flow radio changed event detail: {:?}", detail);
            
            let new_value = el.value();

            if let Some(variant) = E::from_str(new_value.as_str()){
                log_trace!("variant: {}", variant);

                let mut value = value.lock().unwrap();
                log_trace!("new value: {:?}, old value: {}", new_value, value);

                *value = new_value;

                if let Some(cb) =  cb_opt.lock().unwrap().as_mut(){
                    return Ok(cb(variant)?);
                }
            }
            Ok(())
        })?;
    
        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.lock().unwrap().clone()
    }

    pub fn on_change(&self, callback:Callback<E>){
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }
}
