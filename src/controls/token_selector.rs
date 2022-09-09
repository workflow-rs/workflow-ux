use crate::prelude::*;
use workflow_ux::result::Result;
use workflow_html::{html, Render};

#[derive(Clone)]
pub struct TokenSelector{
    pub layout: ElementLayout,
    pub element_wrapper : ElementWrapper,
    value : Rc<RefCell<String>>,
    on_change_cb: Rc<RefCell<Option<Callback<String>>>>
}



impl TokenSelector{
    
    pub fn element(&self) -> FlowMenuBase {
        self.element_wrapper.element.clone().dyn_into::<FlowMenuBase>().expect("Unable to cast TokenSelector Element to FlowMenuBase")
    }

    pub fn focus(&self) -> Result<()> {
        Ok(self.element().focus_form_control()?)
    }

    pub fn new(layout : &ElementLayout, _attributes: &Attributes, _docs : &Docs) -> Result<TokenSelector> {
        let doc = document();
        let element = doc
            .create_element("flow-menu")?;

        let amount_title = "Rate";
        let token_title = "Token";
        let btn_text = "Add";
        let selected_label = "Selected";
        
        let tree = html!{
            <div class="h-box align-center">
                <flow-input label={amount_title}></flow-input>
                <flow-select label={token_title}></flow-select>
                <flow-btn>{btn_text}</flow-btn>
            </div>
            <flow-input label={selected_label}></flow-input>
        }?;

        tree.inject_into(&element)?;
            

        let init_value: String = String::from("");
        let value = Rc::new(RefCell::new(init_value));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;

        let mut control = TokenSelector {
            layout:layout.clone(),
            element_wrapper: ElementWrapper::new(element),
            value,
            on_change_cb: Rc::new(RefCell::new(None))
        };

        control.init_events()?;
        Ok(control)
    }

    fn init_events(&mut self) -> Result<()>{
        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper.on("select", move |event| ->Result<()> {

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
    pub fn on_change(&self, callback:Callback<String>){
        *self.on_change_cb.borrow_mut() = Some(callback);
    }
}
