use crate::prelude::*;
use crate::controls::Callback;
use std::{convert::Into, marker::PhantomData};
use js_sys::Array;
use workflow_ux::result::Result;


#[wasm_bindgen]
extern "C" {
    pub type FlowMenu;
    #[wasm_bindgen (extends = BaseElement, js_name = FlowMenu , typescript_type = "FlowMenu")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    // The `FlowMenuBase` class.
    pub type FlowMenuBase;

    // Getter for the `value` field of this object.
    #[wasm_bindgen (structural , method , getter , js_class = "FlowMenuBase" , js_name = value)]
    pub fn value(this: &FlowMenuBase) -> String;

    #[wasm_bindgen (js_namespace=FlowMenu, extends = ::js_sys::Object)]
    pub type SelectOption;

    // Create option Object for select
    #[wasm_bindgen(static_method_of=FlowMenu, js_name = createOption)]
    pub fn _create_option(text:String, value:String)->SelectOption;

    // Create option Object for select
    #[wasm_bindgen(static_method_of=FlowMenu, js_name = createOption)]
    pub fn _create_option_with_cls(text:String, value:String, cls:String)->SelectOption;

    #[wasm_bindgen (structural , method , js_class = "FlowMenuBase" , js_name = selectOne)]
    pub fn _select(this: &FlowMenuBase, value:String);

    // Remove old/current options and set new option
    #[wasm_bindgen (structural , method , js_class = "FlowMenuBase" , js_name = changeOptions)]
    pub fn change_options(this: &FlowMenuBase, options:Array);
}


impl FlowMenuBase{
    pub fn select<S: Into<String>>(self: &FlowMenuBase, selection:S){
        self._select(selection.into());
    }
}

#[derive(Clone)]
pub struct Select<E> {
    pub element : Element,
    value : Rc<RefCell<String>>,
    on_change_cb: Rc<RefCell<Option<Callback<String>>>>,
    p:PhantomData<E>
}

impl Select<()>{
    pub fn create_option<S: Into<String>>(text:S, value:S)->SelectOption{
        FlowMenu::_create_option(text.into(), value.into())
    }

    pub fn create_option_with_cls<S: Into<String>>(text:S, value:S, cls:S)->SelectOption{
        FlowMenu::_create_option_with_cls(text.into(), value.into(), cls.into())
    }
}



impl<E> Select<E>
where E: EnumTrait<E>
{
    
    // pub fn element(&self) -> Element {
    //     self.element.clone()
    //     // Ok(self.element.clone().dyn_into::<FlowMenuBase>()?)
    // }
    pub fn element(&self) -> FlowMenuBase {
        self.element.clone().dyn_into::<FlowMenuBase>().expect("Unable to cast Select as FlowMenuBase")
    }
    // pub fn focus(&self) -> Result<()> {
    pub fn focus(&self) -> Result<()> {
        self.element().focus_form_control()?;
        // self.element().focus_form_control()
        Ok(())
    }

    pub fn bind(element : &Element) -> Result<Select<String>> {

        let value = match element.get_attribute("value") {
            Some(value) => { value },
            None => { "".to_string() },
        };

        let control = Select {
            element : element.clone(),
            value : Rc::new(RefCell::new(value)),
            on_change_cb:Rc::new(RefCell::new(None)),
            p:PhantomData
        };

        Ok(control)
    }


    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Select<E>> {
        let doc = document();
        let element = doc
            .create_element("flow-select")?;
            

        let init_value: String = String::from("");
        let items = E::list();
        for item in items.iter() {
            let menu_item = doc.create_element("flow-menu-item")?;
            menu_item.set_attribute("value", item.as_str())?;
            menu_item.set_inner_html(item.descr());
            element.append_child(&menu_item)?;
        }

        for (k,v) in attributes.iter() {
            if k.eq("multiple"){
                log_trace!("Use `MultiSelect` for multiple selection {:?}", attributes);
                continue;
            }
            element.set_attribute(k,v)?;
        }
        let value = Rc::new(RefCell::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let control = Select {
            element,
            value,
            on_change_cb:Rc::new(RefCell::new(None)),
            p:PhantomData
        };

        control.init_events()?;
        Ok(control)
    }

    fn init_events(&self) -> Result<()>{
        let el = self.element();
        // let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {

            log_trace!("Select: {:?}", event);
            let current_value = el.value();
            let mut value = value.borrow_mut();
            *value = current_value.clone();
            match &mut*cb_opt.borrow_mut(){
                Some(cb)=>{
                    cb(current_value)
                },
                None=>{}
            };

        }) as Box<dyn FnMut(_)>);
        self.element.add_event_listener_with_callback("select", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }
    pub fn on_change(&self, callback:Callback<String>){
        *self.on_change_cb.borrow_mut() = Some(callback);
    }
}
