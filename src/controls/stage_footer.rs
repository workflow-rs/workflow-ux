use crate::prelude::*;
use std::convert::Into;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = BaseElement, js_name = StageFooter , typescript_type = "StageFooter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StageFooter` class."]
    pub type StageFooter;

    # [wasm_bindgen (extends = CustomEvent , extends = ::js_sys::Object , js_name = StageFooterBtnEvent , typescript_type = "StageFooterBtnEvent")]
    #[derive(Debug, Clone)]
    pub type StageFooterBtnEvent;

    # [wasm_bindgen (structural , method , js_class = "StageFooter" , js_name = enableButton)]
    pub fn _enable_btn(this: &StageFooter, value: String);

    # [wasm_bindgen (structural , method , js_class = "StageFooter" , js_name = disableButton)]
    pub fn _disable_btn(this: &StageFooter, value: String);

    # [wasm_bindgen (structural , method , js_class = "StageFooter" , js_name = showButton)]
    pub fn _show_btn(this: &StageFooter, value: String);

    # [wasm_bindgen (structural , method , js_class = "StageFooter" , js_name = hideButton)]
    pub fn _hide_btn(this: &StageFooter, value: String);

}

impl StageFooterBtnEvent {
    pub fn btn(&self) -> String {
        let btn_js = js_sys::Reflect::get(&self.detail(), &JsValue::from_str("btn")).unwrap();
        match btn_js.as_string() {
            Some(btn) => btn,
            None => "".to_string(),
        }
    }
}

impl StageFooter {
    pub fn enable_btn<S: Into<String>>(self: &StageFooter, btn: S) {
        self._enable_btn(btn.into());
    }
    pub fn disable_btn<S: Into<String>>(self: &StageFooter, btn: S) {
        self._disable_btn(btn.into());
    }
    pub fn show_btn<S: Into<String>>(self: &StageFooter, btn: S) {
        self._show_btn(btn.into());
    }
    pub fn hide_btn<S: Into<String>>(self: &StageFooter, btn: S) {
        self._hide_btn(btn.into());
    }
}
