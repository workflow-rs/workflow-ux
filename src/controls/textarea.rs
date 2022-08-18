use crate::prelude::*;
use workflow_ux::result::Result;
// use workflow_ux::error::Error;

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
    pub element : FlowTextareaBase,
    value : Rc<RefCell<String>>,
}

impl Textarea {
    
    pub fn element(&self) -> FlowTextareaBase {
        self.element.clone().dyn_into::<FlowTextareaBase>().expect("Unable to cast to FlowTextareaBase")
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
            .create_element("flow-textarea")?
            .dyn_into::<FlowTextareaBase>()
            .map_err(|err|format!("Unable to create & cast FlowTextareaBase {:#?}",err))?;

        let init_value: String = String::from("");
        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }


        let value = Rc::new(RefCell::new(init_value));

        {
            let el = element.clone();
            let value = value.clone();
            let closure = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
                let current_element_value = el.value();
                let mut value = value.borrow_mut();
                log_trace!("current value: {:?}", current_element_value);

                *value = current_element_value;

            }) as Box<dyn FnMut(_)>);
            element.add_event_listener_with_callback("changed", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        Ok(Textarea { 
            layout : layout.clone(),
            element,
            value,
        })
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }
}

