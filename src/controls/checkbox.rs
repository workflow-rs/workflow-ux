use crate::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct Checkbox {
    pub layout : ElementLayout,
    pub element_wrapper : ElementWrapper,
    value : Rc<RefCell<bool>>,
    on_change_cb:Rc<RefCell<Option<CallbackNoArgs>>>,
}

impl Checkbox {
    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn new(pane : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Checkbox> {
        let element = document()
            .create_element("flow-checkbox")?;
        for (k,v) in attributes.iter() {
            if k.eq("title") || k.eq("html") || k.eq("label"){
                element.set_inner_html(v);
            }else{
                element.set_attribute(k,v)?;
            }
        }
        let value = Rc::new(RefCell::new(false));

        let mut control = Checkbox { 
            layout : pane.clone(),
            element_wrapper: ElementWrapper::new(element),
            value,
            on_change_cb:Rc::new(RefCell::new(None))
        };

        control.init()?;

        Ok(control)
    }

    fn init(&mut self)->Result<()>{

        let el = self.element();
        let value = self.value.clone();
        let cb_opt = self.on_change_cb.clone();
        self.element_wrapper.on("changed", move |_event| ->Result<()> {
            let new_value = el.get_attribute("checked").is_some();
            log_trace!("new value: {:?}", new_value);

            *value.borrow_mut() = new_value;

            if let Some(cb) =  &mut*cb_opt.borrow_mut(){
                return Ok(cb()?);
            }

            Ok(())

        })?;

        Ok(())
    }

    pub fn value(&self) -> bool {
        *self.value.borrow()
    }

    pub fn on_change(&self, callback:CallbackNoArgs){
        *self.on_change_cb.borrow_mut() = Some(callback);
    }
}
