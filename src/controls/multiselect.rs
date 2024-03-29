use crate::prelude::*;
use crate::result::Result;
use js_sys::Array;
use std::{convert::Into, marker::PhantomData};

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
    pub fn _select(this: &FlowMultiMenuBase, values: Array);

    // Remove old/current options and set new option
    # [wasm_bindgen (structural , method , js_class = "FlowMultiMenuBase" , js_name = changeOptions)]
    pub fn change_options(this: &FlowMultiMenuBase, options: Array);
}

impl FlowMultiMenuBase {
    pub fn select<S: ToString>(self: &FlowMultiMenuBase, selection: Vec<S>) {
        let select: Vec<String> = selection.iter().map(|a| a.to_string()).collect();
        let list = Array::new_with_length(select.len() as u32);
        for str in select {
            list.push(&JsValue::from_str(&str));
        }
        self._select(list);
    }
}

#[derive(Clone)]
pub struct MultiSelect<E> {
    pub element_wrapper: ElementWrapper,
    values: Arc<Mutex<Vec<String>>>,
    on_change_cb: Arc<Mutex<Option<CallbackFn<Vec<String>>>>>,
    p: PhantomData<E>,
}

impl<E> MultiSelect<E>
where
    E: EnumTrait<E>,
{
    pub fn element(&self) -> FlowMultiMenuBase {
        self.element_wrapper
            .element
            .clone()
            .dyn_into::<FlowMultiMenuBase>()
            .expect("Unable to case to FlowMultiMenuBase")
    }

    pub fn focus(&self) -> Result<()> {
        self.element().focus_form_control()?;
        Ok(())
    }

    pub fn new(
        layout: &ElementLayout,
        attributes: &Attributes,
        _docs: &Docs,
    ) -> Result<MultiSelect<E>> {
        let doc = document();
        let element = doc.create_element("flow-select")?;

        let mut init_value: String = String::from("");
        let items = E::list();
        for item in items.iter() {
            let menu_item = doc.create_element("flow-menu-item")?;
            menu_item.set_attribute("value", item.as_str())?;
            menu_item.set_inner_html(item.descr());
            element.append_child(&menu_item)?;
            if init_value.eq("") {
                init_value = String::from(item.as_str())
            }
        }

        element.set_attribute("multiple", "true")?;
        for (k, v) in attributes.iter() {
            element.set_attribute(k, v)?;
        }
        let values: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

        let pane_inner = layout
            .inner()
            .ok_or_else(|| JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut control = MultiSelect {
            element_wrapper: ElementWrapper::new(element),
            values,
            on_change_cb: Arc::new(Mutex::new(None)),
            p: PhantomData,
        };
        control.init_events()?;
        Ok(control)
    }

    fn init_events(&mut self) -> Result<()> {
        let el = self.element();
        let values = self.values.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper
            .on("select", move |event| -> Result<()> {
                log_trace!("MultiSelect: select event: {:?}", event);
                let items: Vec<String> = serde_wasm_bindgen::from_value(el.value())?;
                log_trace!("MultiSelect: current_values: {:?}", items);
                let mut values = values.lock().unwrap();
                *values = items.clone();

                if let Some(cb) = cb_opt.lock().unwrap().as_mut() {
                    cb(items)?;
                };

                Ok(())
            })?;

        Ok(())
    }

    pub fn values(&self) -> Vec<String> {
        self.values.lock().unwrap().clone()
    }

    pub fn on_change(&self, callback: CallbackFn<Vec<String>>) {
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }
}
