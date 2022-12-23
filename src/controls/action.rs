use crate::prelude::*;
use crate::layout::ElementLayout;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct Action {
    pub element_wrapper : ElementWrapper,
    callback : OptionalCallbackFnNoArgs
}

impl Action {

    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn new(
        layout: &ElementLayout,
        attributes: &Attributes,
        _docs : &Docs
    ) -> Result<Action> {
        let element = document()
            .create_element("flow-btn")?;
        element.set_text_content(Some("ACTION BUTTON"));

        for (k,v) in attributes.iter() {
            if k.eq("text"){
                element.set_text_content(Some(v));
            }else if k.eq("html"){
                element.set_inner_html(v);
            }else{
                element.set_attribute(k,v)?;
            }
        }

        let parent = layout.element();
        parent.append_child(&element)?;

        let mut action = Action{
            element_wrapper: ElementWrapper::new( element ),
            callback : Arc::new(Mutex::new(None))
        };

        action.init()?;
        
        Ok(action)
    }

    pub fn with_callback(&self, callback : CallbackFnNoArgs) -> &Self {
        *self.callback.lock().unwrap() = Some(callback);
        self
    }

    pub fn init(&mut self) -> Result<()> {
        let cb_opt = self.callback.clone();
        self.element_wrapper.on_click(move |event| -> Result<()> {
            log_trace!("action button received mouse event: {:#?}", event);
            if let Some(cb) = cb_opt.lock().unwrap().as_mut(){
                cb()?;
            };
            Ok(())
        })?;
        Ok(())
    }
}
