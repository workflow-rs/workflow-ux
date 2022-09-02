use crate::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct TokenSelect{
    pub element : Element,
    value : Rc<RefCell<String>>,
    on_change_cb: Rc<RefCell<Option<Callback<String>>>>
}



impl TokenSelect{
    
    pub fn element(&self) -> FlowMenuBase {
        self.element.clone().dyn_into::<FlowMenuBase>().expect("Unable to cast TokenSelect to FlowMenuBase")
    }

    pub fn focus(&self) -> Result<()> {
        Ok(self.element().focus_form_control()?)
    }

    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<TokenSelect> {
        let doc = document();
        let element = doc
            .create_element("workflow-token-select")?;
            

        let init_value: String = String::from("");
        for (k,v) in attributes.iter() {
            if k.eq("multiple"){
                log_trace!("Use `MultiSelect` for multiple selection {:?}", attributes);
                continue;
            }
            if k.eq("hide_name"){
                element.set_attribute("hide-name",v)?;
            }else if k.eq("small_badge"){
                element.set_attribute("small-badge",v)?;
            }else{
                element.set_attribute(k,v)?;
            }
        }
        let value = Rc::new(RefCell::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let control = TokenSelect {
            element,
            value,
            on_change_cb:Rc::new(RefCell::new(None))
        };

        control.init_events()?;
        Ok(control)
    }

    fn init_events(&self) -> Result<()>{
        let el = self.element.clone().dyn_into::<FlowMenuBase>()?;//.map_err(|err|error!("Unable to cast TokenSelect to FlowMenuBase {:#?}",err))?;
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {

            log_trace!("Select: {:?}", event);
            let current_value = el.value();
            let mut value = value.borrow_mut();
            *value = current_value.clone();
            if let Some(cb) = &mut*cb_opt.borrow_mut(){
                cb(current_value);
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
