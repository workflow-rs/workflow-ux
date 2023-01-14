use crate::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct Checkbox {
    pub layout: ElementLayout,
    pub element_wrapper: ElementWrapper,
    value: Arc<Mutex<bool>>,
    on_change_cb: Arc<Mutex<Option<CallbackFnNoArgs>>>,
}

impl Checkbox {
    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn new(pane: &ElementLayout, attributes: &Attributes, _docs: &Docs) -> Result<Checkbox> {
        let element = document().create_element("flow-checkbox")?;
        for (k, v) in attributes.iter() {
            if k.eq("title") || k.eq("html") || k.eq("label") {
                element.set_inner_html(v);
            } else {
                element.set_attribute(k, v)?;
            }
        }
        let value = Arc::new(Mutex::new(false));

        let mut control = Checkbox {
            layout: pane.clone(),
            element_wrapper: ElementWrapper::new(element),
            value,
            on_change_cb: Arc::new(Mutex::new(None)),
        };

        control.init()?;

        Ok(control)
    }

    fn init(&mut self) -> Result<()> {
        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper
            .on("changed", move |_event| -> Result<()> {
                let new_value = el.get_attribute("checked").is_some();
                log_trace!("new value: {:?}", new_value);

                *value.lock().unwrap() = new_value;

                if let Some(cb) = cb_opt.lock().unwrap().as_mut() {
                    return Ok(cb()?);
                }

                Ok(())
            })?;

        Ok(())
    }

    pub fn value(&self) -> bool {
        *self.value.lock().unwrap()
    }

    pub fn on_change(&self, callback: CallbackFnNoArgs) {
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }
}
