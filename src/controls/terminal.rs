use crate::layout::ElementLayout;
use crate::prelude::*;
use std::convert::Into;

// use workflow_ux::error::Error;
// #[cfg(target_arch = "wasm32")]
use workflow_wasm::utils;
// #[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
use workflow_ux::result::Result;

#[wasm_bindgen]
extern "C" {
    // The `WorkflowTerminal` class.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    // *This API requires the following crate features to be activated: `Element`*
    #[wasm_bindgen (extends = BaseElement, js_name = WorkflowTerminal , typescript_type = "WorkflowTerminal")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WorkflowTerminal;
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)
    // *This API requires the following crate features to be activated: `Element`*
    #[wasm_bindgen (structural, method, js_class = "WorkflowTerminal" , js_name = write)]
    pub fn write(this: &WorkflowTerminal, text: JsValue);

    #[wasm_bindgen (structural, method, js_class = "WorkflowTerminal", js_name = prompt)]
    pub fn prompt(this: &WorkflowTerminal);
}

#[derive(Clone)]
pub struct Terminal {
    pub element_wrapper: ElementWrapper,
    value: Arc<Mutex<String>>,
}

impl Terminal {
    pub fn element(&self) -> WorkflowTerminal {
        self.element_wrapper
            .element
            .clone()
            .dyn_into::<WorkflowTerminal>()
            .expect("Unable to cast to WorkflowTerminal")
    }

    pub fn new(layout: &ElementLayout, attributes: &Attributes, _docs: &Docs) -> Result<Terminal> {
        let element = document().create_element("workflow-terminal")?;
        let init_value: String = String::from("");

        for (k, v) in attributes.iter() {
            element.set_attribute(k, v)?;
        }

        let value = Arc::new(Mutex::new(init_value));
        let pane_inner = layout
            .inner()
            .ok_or_else(|| JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;
        let mut terminal = Terminal {
            element_wrapper: ElementWrapper::new(element),
            value,
        };
        terminal.init_event()?;
        Ok(terminal)
    }

    fn init_event(&mut self) -> Result<()> {
        let this = self.clone();
        self.element_wrapper.on("cmd", move |event| -> Result<()> {
            log_trace!("received terminal event: {:#?}", event);
            let detail = event.detail();

            let cmd = utils::try_get_string(&detail, "cmd")?;
            log_trace!("cmd: {:#?}", cmd);
            let _this = this.clone();
            let pr = future_to_promise(async move { _this.sink(cmd).await });
            utils::apply_with_args1(&detail, "resolve", JsValue::from(pr))?;

            Ok(())
        })?;

        Ok(())
    }

    pub async fn sink(&self, cmd: String) -> std::result::Result<JsValue, JsValue> {
        if cmd.eq("hello") {
            Ok(JsValue::from_str(&format!("success:{cmd}")))
        } else {
            Err(JsValue::from_str(&format!("error:{cmd}")))
        }
    }

    pub fn value(&self) -> String {
        self.value.lock().unwrap().clone()
    }

    pub fn write<T: Into<String>>(&self, str: T) -> Result<()> {
        self.element().write(JsValue::from_str(&str.into()));
        Ok(())
    }
    pub fn prompt(&self) -> Result<()> {
        self.element().prompt();
        Ok(())
    }
}
