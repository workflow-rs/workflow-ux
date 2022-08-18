
use std::collections::BTreeMap;
use std::sync::Mutex;
use workflow_ux::prelude::*;
use workflow_html::ElementResult;
use workflow_ux::result::Result;
// use workflow_log::*;


#[derive(Debug, Clone)]
pub struct Image {
    element: HtmlImageElement,
    onclick : Arc<Mutex<Option<Closure::<dyn FnMut(web_sys::MouseEvent)>>>>,
    onerror : Arc<Mutex<Option<Closure::<dyn FnMut(web_sys::ErrorEvent)>>>>,

    // element: Element,
    // url : String,
}

impl Image {

    pub fn element(&self) -> Element {
        self.element.clone().dyn_into::<Element>().expect("Unable to case HtmlImageElement to Element")
    }

     // TODO review: id is not used
    pub fn new() -> Result<Self> {

        let element = HtmlImageElement::new()?; //document().create_element("img")?;
        // li.set_attribute("class", &format!("menu-item skip-drawer-event"))?;

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

    pub fn with_src_and_fallback(self, url: &str, fallback: &str) -> Result<Self> {

        let self_ = self.clone();
        let fallback_ = fallback.to_string();
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(Box::new(move |error: web_sys::ErrorEvent| {
            log_trace!("Image error: {:?}", error);
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

        //  add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        // self.element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        // closure.forget();
        Ok(self)
    }

}



impl workflow_html::Render for Image {
    fn render<W:std::fmt::Write>(&self, _w:&mut W) -> std::fmt::Result {
        Ok(())
    }

    fn render_node(self, parent:&mut Element, _map:&mut BTreeMap<String, Element>)->ElementResult<()>{
        parent.append_child(&self.element)?;
        Ok(())
    }


}