use crate::prelude::*;
use crate::result::Result;

static mut DRAWER : Option<Arc<AppDrawer>> = None;

pub fn get_drawer()-> Option<Arc<AppDrawer>>{
    unsafe {DRAWER.clone()}
}
pub fn set_drawer(drawer: Arc<AppDrawer>){
    unsafe {DRAWER = Some(drawer)}
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = BaseElement,  js_name = FlowAppDrawer , typescript_type = "FlowAppDrawer")]
    // "The `AppDrawer` class.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AppDrawer;

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "toggleLeftDrawer")]
    pub fn toggle_left_drawer(this: &AppDrawer);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "closeLeftDrawer")]
    pub fn close_left_drawer(this: &AppDrawer);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "toggleRightDrawer")]
    pub fn toggle_right_drawer(this: &AppDrawer);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "closeRightDrawer")]
    pub fn close_right_drawer(this: &AppDrawer);

}

impl AppDrawer{
    pub fn get(selector:&str)->Result<Self>{
        let drawer_el_opt = document().query_selector(selector)?;
        if drawer_el_opt.is_none(){
            panic!("Unable to find `{}` element for AppDrawer", selector);
        }
        let drawer = drawer_el_opt.unwrap().dyn_into::<AppDrawer>()?;
        set_drawer(Arc::new(drawer.clone()));
        Ok(drawer)
    }
}
