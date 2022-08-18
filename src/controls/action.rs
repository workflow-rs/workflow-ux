use std::sync::Mutex;

use workflow_ux::result::Result;
use crate::prelude::*;
use crate::layout::ElementLayout;
use web_sys::Element;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ActionInner {
    pub element : Element,
    #[derivative(Debug="ignore")]
    callback : Option<Box<dyn FnMut() -> Result<()>>>
}

// declare_async_rwlock!(Action, ActionInner);
#[derive(Clone)]
pub struct Action(Arc<Mutex<ActionInner>>);

impl Action {

    pub fn inner(&self) -> Option<std::sync::MutexGuard<ActionInner>> {
        self.0.lock().ok()
    }


    pub fn element(&self) -> Element {
        self.inner().unwrap().element.clone()
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

        let action = Action( Arc::new(Mutex::new(ActionInner {
            element: element.clone(),
            callback : None
        })));

        action.init()?;
        let parent = layout.element();
        parent.append_child(&element)?;
        Ok(action)
    }

    pub fn with_callback(&self, callback : Box<dyn FnMut() -> Result<()>>) -> &Self {
        self.inner().unwrap().callback = Some(callback);
        self
    }

    pub fn init(&self) -> Result<()> {
        let this = self.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| -> Result<()> {
            log_trace!("action button received mouse event: {:#?}", event);
            this.action()?;
            // TODO: handle error logging
            Ok(())
        }) as Box<dyn FnMut(_) -> Result<()>>);
        self.inner().unwrap().element
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }

    pub fn action(&self) -> Result<()> {
        log_trace!("$$$ RUNNING ACTION $$$");
        let callback = &mut self.inner().unwrap().callback;
        if let Some(callback) = callback {
            callback()?;
            // TODO: handle error logging
        }
        Ok(())
    }
}
