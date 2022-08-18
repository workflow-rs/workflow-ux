use crate::prelude::*;
use crate::layout::ElementLayout;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct Checkbox {
    pub layout : ElementLayout,
    pub element : Element,
    value : Rc<RefCell<bool>>,
}

impl Checkbox {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(pane : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Checkbox> {  // pane-ctl

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html
        let element = document()
            .create_element("flow-checkbox")?;
        for (k,v) in attributes.iter() {
            if k.eq("title") || k.eq("html"){
                element.set_inner_html(v);
            }else{
                element.set_attribute(k,v)?;
            }
        }
        let value = Rc::new(RefCell::new(false));
        
        // ~~~
        {
            let el = element.clone();
            let value = value.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {

                log_trace!("received change event: {:?}", event);
                let current_element_value = el.get_attribute("checked").is_some();
                log_trace!("current value: {:?}", current_element_value);
                let mut value = value.borrow_mut();
                log_trace!("current value: {:?}", current_element_value);

                *value = current_element_value;

            }) as Box<dyn FnMut(_)>);
            element.add_event_listener_with_callback("changed", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(Checkbox { 
            layout : pane.clone(),
            element,
            value,
        })
    }

    pub fn value(&self) -> bool {
        *self.value.borrow()
    }
}
