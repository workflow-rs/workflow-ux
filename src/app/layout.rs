use crate::{prelude::*, error};
use crate::result::Result;
use crate::wasm::load_component;

static mut LAYOUT : Option<Arc<AppLayout>> = None;

pub fn get_layout()-> Option<Arc<AppLayout>>{
    unsafe {LAYOUT.clone()}
}
pub fn set_layout(layout: Arc<AppLayout>){
    unsafe {LAYOUT = Some(layout)}
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = BaseElement,  js_name = WorkflowAppLayout , typescript_type = "WorkflowAppLayout")]
    // "The `AppLayout` class.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AppLayout;

    #[wasm_bindgen (method, js_name = "toggleLeftDrawer")]
    pub fn toggle_left_drawer(this: &AppLayout);

    #[wasm_bindgen (method, js_name = "closeLeftDrawer")]
    pub fn close_left_drawer(this: &AppLayout);

    #[wasm_bindgen (method, js_name = "toggleRightDrawer")]
    pub fn toggle_right_drawer(this: &AppLayout);

    #[wasm_bindgen (method, js_name = "closeRightDrawer")]
    pub fn close_right_drawer(this: &AppLayout);

}

impl AppLayout{

    pub fn load_js(flow_ux_path:&str)->Result<()>{
        let cmp = include_str!("layout.js");
        load_component(flow_ux_path, "app-layout.js", cmp)?;
        Ok(())
    }

    pub fn get(selector:&str)->Result<Self>{
        let layout_el = find_el(selector, "missing workspace AppLayout element")?;
        let layout = match layout_el.dyn_into::<AppLayout>(){
            Ok(el)=>el,
            Err(el)=>{
                return Err(error!("Unable to cast '{selector}' to AppLayout, JsValue:{:?}", el));
            }
        };
        set_layout(Arc::new(layout.clone()));
        Ok(layout)
    }
}

