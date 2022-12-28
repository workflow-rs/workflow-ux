use wasm_bindgen::prelude::*;
use crate::result::Result;
use workflow_dom::inject::{Content, inject_blob_nowait};
use workflow_wasm::init::init_workflow;
pub use workflow_wasm::init::{global, workflow};
use crate::location;

pub fn init_ux(workflow: &JsValue, modules: &JsValue) -> Result<()> {
    init_workflow(workflow, modules)?;
    Ok(())
}

#[wasm_bindgen(js_name="loadComponents")]
pub fn load_components(flow_ux_path:&str)->Result<()>{
    println!("flow_ux_path:{:?}", flow_ux_path);

    crate::app::layout::AppLayout::load_js(flow_ux_path)?;
    Ok(())
}

pub fn load_component(flow_ux_path:&str, _name:&str, cmp:&str)->Result<()>{
    let loc = location();
    let origin = loc.origin()?;
    let js = cmp.replace("[FLOW-UX-PATH]", flow_ux_path)
                        .replace("[HOST-ORIGIN]", &origin);
    inject_blob_nowait(Content::Module(js.as_bytes()))?;
    Ok(())
}
