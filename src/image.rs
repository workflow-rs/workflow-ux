
use std::sync::Mutex;
use crate::prelude::*;
use crate::result::Result;


#[derive(Debug, Clone)]
pub struct Image {
    element: HtmlImageElement,
    onclick : Arc<Mutex<Option<Closure::<dyn FnMut(web_sys::MouseEvent)>>>>,
    onerror : Arc<Mutex<Option<Closure::<dyn FnMut(web_sys::ErrorEvent)>>>>,
}

impl Image {

    pub fn element(&self) -> HtmlImageElement {
        self.element.clone()
    }

    pub fn new() -> Result<Self> {
        let element = HtmlImageElement::new()?;
        Ok(Self {
            element,
            onerror : Arc::new(Mutex::new(None)),
            onclick : Arc::new(Mutex::new(None)),
        })
    }

    pub fn with_class(self, class: &str) -> Result<Self> {
        self.element().set_attribute("class", class)?;
        Ok(self)
    }

    pub fn set_src_and_fallback(&self, url: &str, fallback: &str) -> Result<()> {

        let self_ = self.clone();
        let fallback_ = fallback.to_string();
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(Box::new(move |error: web_sys::ErrorEvent| {
            log_trace!("Image error: {:?}, fallback_:{}", error, fallback_);
            self_.element.set_onerror(None);
            self_.element.set_src(&fallback_);
        }));
        
        self.element.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        *self.onerror.lock().unwrap() = Some(onerror);
        self.element.set_src(url);

        Ok(())
    }

    pub fn with_src_and_fallback(self, url: &str, fallback: &str) -> Result<Self> {

        let self_ = self.clone();
        let fallback_ = fallback.to_string();
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(Box::new(move |error: web_sys::ErrorEvent| {
            log_trace!("Image error: {:?}, fallback_:{}", error, fallback_);
            self_.element.set_onerror(None);
            self_.element.set_src(&fallback_);
        }));
        
        self.element.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        *self.onerror.lock().unwrap() = Some(onerror);
        self.element.set_src(url);

        Ok(self)
    }

    pub fn with_callback(self, callback: Box<dyn Fn() -> Result<()>>) -> Result<Self> {
        let onclick = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(Box::new(move |event: web_sys::MouseEvent| {
            log_trace!("Link::with_callback called");
            event.stop_immediate_propagation();
            match callback() {
                Ok(_) => {},
                Err(err) => {
                    log_error!("Error executing MenuItem callback: {:?}", err);
                }
            }
        }));
        self.element.set_onclick(Some(onclick.as_ref().unchecked_ref()));
        *self.onclick.lock().unwrap() = Some(onclick);

        Ok(self)
    }

}

/*
 
impl workflow_html::Render for Image {
    fn render<W:std::fmt::Write>(&self, _w:&mut W) -> std::fmt::Result {
        Ok(())
    }

    fn render_node(&self, parent:&mut Element, _map:&mut BTreeMap<String, Element>)->ElementResult<()>{
        parent.append_child(&self.element)?;
        Ok(())
    }
}
*/