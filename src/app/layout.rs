use crate::prelude::*;
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
    # [wasm_bindgen (extends = BaseElement,  js_name = FlowAppDrawer , typescript_type = "FlowAppDrawer")]
    // "The `AppLayout` class.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AppLayout;

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "toggleLeftDrawer")]
    pub fn toggle_left_drawer(this: &AppLayout);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "closeLeftDrawer")]
    pub fn close_left_drawer(this: &AppLayout);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "toggleRightDrawer")]
    pub fn toggle_right_drawer(this: &AppLayout);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "closeRightDrawer")]
    pub fn close_right_drawer(this: &AppLayout);

}

impl AppLayout{

    pub fn load_js(flow_ux_path:&str)->Result<()>{
        let cmp = include_str!("layout.js");
        load_component(flow_ux_path, "app-layout.js", cmp)?;
        Ok(())
    }

    pub fn get(selector:&str)->Result<Self>{
        let drawer_el_opt = document().query_selector(selector)?;
        if drawer_el_opt.is_none(){
            panic!("Unable to find `{}` element for AppLayout", selector);
        }
        let drawer = drawer_el_opt.unwrap().dyn_into::<AppLayout>()?;
        set_layout(Arc::new(drawer.clone()));
        Ok(drawer)
    }
}

