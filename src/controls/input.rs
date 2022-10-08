use crate::prelude::*;
use crate::layout::ElementLayout;
use std::convert::Into;
use crate::result::Result;
use crate::error::Error;
use crate::controls::listener::Listener;


#[wasm_bindgen]
extern "C" {
    // The `FlowInputBase` class.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    // *This API requires the following crate features to be activated: `Element`*
    #[wasm_bindgen (extends = BaseElement, js_name = FlowInput , typescript_type = "FlowInput")]
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
    pub attributes: Attributes,
    pub element_wrapper : ElementWrapper,
    value : Rc<RefCell<String>>,
}

impl Input {

    pub fn element(&self) -> FlowInputBase {
        self.element_wrapper.element.clone().dyn_into::<FlowInputBase>().expect("Unable to cast element to FlowInputBase")
    }

    pub fn new(
        layout : &ElementLayout,
        attributes: &Attributes,
        docs : &Docs
    ) -> Result<Input> {
        let element = document()
            .create_element("flow-input")?;

        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        Ok(Self::create(element, layout.clone(), attributes, docs, String::from(""))?)
    }
    fn create(
        element: Element,
        layout : ElementLayout,
        attributes: &Attributes,
        _docs : &Docs,
        mut init_value: String
    ) -> Result<Input> {

        element.set_attribute("value", init_value.as_str())?;
        //element.set_attribute("label", "Input")?;
        //element.set_attribute("placeholder", "Please enter")?;
        element.set_attribute("tab-index","0")?;

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
            if k.eq("value"){
                init_value = v.to_string();
            }
        }
        let value = Rc::new(RefCell::new(init_value));

        let mut input = Input { 
            layout,
            attributes:attributes.clone(),
            element_wrapper: ElementWrapper::new(element),
            value,
        };

        input.init()?;

        Ok(input)
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn set_value<T: Into<String>>(&self, value:T)->Result<()>{
        let value = value.into();
        FieldHelper::set_value_attr(&self.element_wrapper.element, &value)?;
        *self.value.borrow_mut() = value;
        Ok(())
    }

    pub fn mark_invalid(&self, invalid:bool)->Result<()>{
        self.element().class_list().toggle_with_force("invalid", invalid)?;
        Ok(())
    }
    

    pub fn init(&mut self)-> Result<()>{
        let element = self.element();
        {
            let el = element.clone();
            let value = self.value.clone();
            self.element_wrapper.on("changed", move |event| -> Result<()> {

                log_trace!("received changed event: {:?}", event);
                let new_value = el.value();
                log_trace!("new_value: {:?}", new_value);
                let mut value = value.borrow_mut();

                *value = new_value;

                Ok(())

            })?;
        }
        {
            let el = element.clone();
            let value = self.value.clone();
            let listener = Listener::new(move |_event:web_sys::CustomEvent| ->Result<()> {

                //log_trace!("received key event: {:#?}", event);
                let new_value = el.value();
                //log_trace!("new_value: {:?}", new_value);
                let mut value = value.borrow_mut();

                *value = new_value;
                Ok(())
            });
            self.element_wrapper.element.add_event_listener_with_callback("keyup", listener.into_js())?;
            self.element_wrapper.element.add_event_listener_with_callback("keydown", listener.into_js())?;
            self.element_wrapper.push_listener(listener);
        }

        Ok(())
    }
}


impl<'refs> TryFrom<ElementBindingContext<'refs>> for Input {
    type Error = Error;

    fn try_from(ctx : ElementBindingContext<'refs>) -> Result<Self> {
        Ok(Self::create(
            ctx.element.clone(),
            ctx.layout.clone(),
            &ctx.attributes,
            &ctx.docs,
            String::new()
        )?)

    }
}