use crate::prelude::*;
use crate::result::Result;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = BaseElement,  js_name = FlowAppDrawer , typescript_type = "FlowAppDrawer")]
    // "The `AppDrawer` class.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AppDrawer;

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "toggleLeftDrawer")]
    pub fn toggle_left_drawer(this: &AppDrawer);

    #[wasm_bindgen (structural , method , js_class = "AppDrawer" , js_name = "toggleRightDrawer")]
    pub fn toggle_right_drawer(this: &AppDrawer);

}

impl AppDrawer{
    pub fn get(selector:&str)->Result<Self>{
        let drawer_el_opt = document().query_selector(selector)?;
        if drawer_el_opt.is_none(){
            panic!("Unable to find `{}` element for AppDrawer", selector);
        }
        let drawer = drawer_el_opt.unwrap().dyn_into::<AppDrawer>()?;
        Ok(drawer)
    }
}
