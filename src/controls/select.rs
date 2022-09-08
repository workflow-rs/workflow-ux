use crate::prelude::*;
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
    pub element_wrapper : ElementWrapper,
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
    
    pub fn element(&self) -> FlowMenuBase {
        self.element_wrapper.element.clone().dyn_into::<FlowMenuBase>().expect("Unable to cast Select as FlowMenuBase")
    }
    pub fn focus(&self) -> Result<()> {
        self.element().focus_form_control()?;
        Ok(())
    }

    pub fn bind(element : &Element) -> Result<Select<String>> {

        let value = match element.get_attribute("value") {
            Some(value) => { value },
            None => { "".to_string() },
        };

        let control = Select {
            element_wrapper : ElementWrapper::new(element.clone()),
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
            

        let mut init_value: String = String::new();
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
            if k.eq("value"){
                element.set_attribute("selected",v)?;
                init_value = v.to_string();
            }else{
                element.set_attribute(k,v)?;
            }
        }
        let value = Rc::new(RefCell::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut control = Select {
            element_wrapper:ElementWrapper::new(element),
            value,
            on_change_cb:Rc::new(RefCell::new(None)),
            p:PhantomData
        };

        control.init_events()?;
        Ok(control)
    }

    fn init_events(&mut self) -> Result<()>{
        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper.on("select", move |event| -> Result<()> {

            log_trace!("Select: {:?}", event);
            let new_value = el.value();
            let mut value = value.borrow_mut();
            *value = new_value.clone();
            if let Some(cb) = &mut*cb_opt.borrow_mut(){
                cb(new_value)?;
            }

            Ok(())

        })?;

        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn set_value<T: Into<String>>(&self, value:T)->Result<()>{
        let value = value.into();
        FieldHelper::set_attr(&self.element_wrapper.element, "selected", &value)?;
        *self.value.borrow_mut() = value;
        Ok(())
    }

    pub fn on_change(&self, callback:Callback<String>){
        *self.on_change_cb.borrow_mut() = Some(callback);
    }

    pub fn change_options<T:Into<String>>(&self, options:Vec<(T, T)>)->Result<()>{
        let items = Array::new_with_length(options.len() as u32);
        for (text, value) in options{
            let opt = Select::create_option(text, value);
            items.push(&JsValue::from(opt));
        }

        self.element().change_options(items);
        Ok(())
    }
}
