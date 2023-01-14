use wasm_bindgen::prelude::*;
use web_sys::Navigator;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (js_namespace=window, getter, js_name = navigator)]
    pub fn get_navigator() -> Navigator;
}

pub fn get_user_agent() -> std::result::Result<String, JsValue> {
    let user_agent = get_navigator().user_agent()?;
    Ok(user_agent)
}
