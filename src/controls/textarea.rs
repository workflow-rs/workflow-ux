use crate::prelude::*;
use workflow_ux::result::Result;

#[wasm_bindgen]
extern "C" {
    // The `FlowTextareaBase` class.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    // *This API requires the following crate features to be activated: `Element`*
    #[wasm_bindgen (extends = BaseElement , js_name = FlowTextarea , typescript_type = "FlowTextarea")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowTextareaBase;
    // Getter for the `namespaceURI` field of this object.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)
    #[wasm_bindgen (structural , method , getter , js_class = "FlowTextarea" , js_name = value)]
    pub fn value(this: &FlowTextareaBase) -> String;
}

#[derive(Clone)]
pub struct Textarea {
    pub layout : ElementLayout,
    pub element_wrapper : ElementWrapper,
    value : Arc<Mutex<String>>,
    on_change_cb:Arc<Mutex<Option<CallbackNoArgs>>>,
}

//impl FieldHelpers for Textarea{}

impl Textarea {
    
    pub fn element(&self) -> FlowTextareaBase {
        self.element_wrapper.element.clone().dyn_into::<FlowTextareaBase>().expect("Unable to cast to FlowTextareaBase")
    }

    pub fn focus(&self) -> Result<()> {
        Ok(self.element().focus_form_control()?)
    }

    pub fn new(
        layout : &ElementLayout,
        attributes: &Attributes,
        _docs : &Docs
    ) -> Result<Textarea> {
        let element = document()
            .create_element("flow-textarea")?;

        let init_value: String = String::from("");
        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }


        let value = Arc::new(Mutex::new(init_value));

        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut control = Textarea { 
            layout : layout.clone(),
            element_wrapper: ElementWrapper::new(element),
            value,
            on_change_cb:Arc::new(Mutex::new(None))
        };

        control.init()?;

        Ok(control)
    }

    fn init(&mut self)->Result<()>{
        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper.on("changed", move |_event| ->Result<()> {
            let new_value = el.value();
            log_trace!("new value: {:?}", new_value);

            *value.lock().unwrap() = new_value;
            if let Some(cb) =  &mut*cb_opt.lock().unwrap(){
                return Ok(cb()?);
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.lock().unwrap().clone()
    }

    pub fn set_value<T: Into<String>>(&self, value:T)->Result<()>{
        let value = value.into();
        FieldHelper::set_value_attr(&self.element_wrapper.element, &value)?;
        *self.value.lock().unwrap() = value;
        Ok(())
    }
    
    pub fn on_change(&self, callback:CallbackNoArgs){
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }

}

