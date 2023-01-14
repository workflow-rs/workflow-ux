use crate::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct TokenSelect {
    pub element_wrapper: ElementWrapper,
    value: Arc<Mutex<String>>,
    on_change_cb: Arc<Mutex<Option<CallbackFn<String>>>>,
}

impl TokenSelect {
    pub fn element(&self) -> FlowMenuBase {
        self.element_wrapper
            .element
            .clone()
            .dyn_into::<FlowMenuBase>()
            .expect("Unable to cast TokenSelect to FlowMenuBase")
    }

    pub fn focus(&self) -> Result<()> {
        Ok(self.element().focus_form_control()?)
    }

    pub fn new(
        layout: &ElementLayout,
        attributes: &Attributes,
        _docs: &Docs,
    ) -> Result<TokenSelect> {
        let doc = document();
        let element = doc.create_element("workflow-token-select")?;

        let init_value: String = String::from("");
        for (k, v) in attributes.iter() {
            if k.eq("multiple") {
                log_trace!("Use `MultiSelect` for multiple selection {:?}", attributes);
                continue;
            }
            if k.eq("hide_name") {
                element.set_attribute("hide-name", v)?;
            } else if k.eq("small_badge") {
                element.set_attribute("small-badge", v)?;
            } else {
                element.set_attribute(k, v)?;
            }
        }
        let value = Arc::new(Mutex::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut control = TokenSelect {
            element_wrapper: ElementWrapper::new(element),
            value,
            on_change_cb: Arc::new(Mutex::new(None)),
        };

        control.init_events()?;
        Ok(control)
    }

    fn init_events(&mut self) -> Result<()> {
        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper
            .on("select", move |event| -> Result<()> {
                log_trace!("Select: {:?}", event);
                let new_value = el.value();
                let mut value = value.lock().unwrap();
                *value = new_value.clone();
                if let Some(cb) = cb_opt.lock().unwrap().as_mut() {
                    cb(new_value)?;
                }

                Ok(())
            })?;

        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.lock().unwrap().clone()
    }
    pub fn on_change(&self, callback: CallbackFn<String>) {
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }
}
