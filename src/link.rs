use core::fmt::Write;
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::sync::Mutex;
use workflow_html::ElementResult;
use workflow_ux::result::Result;
use workflow_log::*;
use workflow_ux::prelude::*;

#[derive(Clone, Debug)]
enum Kind {
    Module,
    External,
}

#[derive(Debug, Clone)]
pub struct Link {
    element : Element,
    kind : Kind,
    text : String,
    href : Option<String>,
    _onclick : Arc<Mutex<Option<Closure::<dyn FnMut(web_sys::MouseEvent)>>>>
}

impl crate::dom::Element for Link {
    fn element(&self) -> Element {
        self.element.clone()
    }

}

impl Link {


    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new_for_callback(text : &str) -> Result<Self> {
    //     let link = Self::new_for_callback_with_cls(text, "")?;
    //     Ok(link)
    // }
    // pub fn new_for_callback_with_cls(text : &str, cls:&str) -> Result<Self> {
        let element = document().create_element("a")?;
        element.set_attribute("href", "javascript:void(0)")?;
        // if cls.len() > 0 {
        //     element.set_attribute("class", cls)?;
        // }
        element.set_inner_html(&text);
        
        Ok(Link {
            kind : Kind::Module,
            text : text.to_string(),
            href : None,
            element,
            _onclick : Arc::new(Mutex::new(None))
        })
    }

    pub fn new_with_url(text : &str, href : &str) -> Result<Self> {

        let element = document().create_element("a")?;
        element.set_attribute("href", href)?;
        element.set_attribute("target", "_blank")?;
        element.set_inner_html(text);

        Ok(Link {
            kind : Kind::External,
            text : text.to_string(),
            href : Some(href.to_string()),
            element,
            _onclick : Arc::new(Mutex::new(None))
        })
    }

    // pub fn create_element(&self) -> Result<Element> {

    //     let element = match self.kind {
    //         Kind::Module => {
    //             let element = document().create_element("a")?;
    //             element.set_attribute("href", "javascript:void(0)")?;
    //             element.set_inner_html(&self.text);
    //             element
    //         },
    //         Kind::External => {
    //             let element = document().create_element("a")?;
    //             if let Some(href) = &self.href {
    //                 element.set_attribute("href", href)?;
    //                 element.set_attribute("target", "_blank")?;
    //                 element.set_inner_html(&self.text);
    //                 element
    //             } else {
    //                 return Err(error!("Link is missing href attribute"));
    //             }
    //         }
    //     };

    //     Ok(element)
    // }

    pub fn with_class(self, class: &str) -> Result<Self> {
        self.element().set_attribute("class", class)?;
        Ok(self)
    }

    pub fn with_callback(self, callback: Box<dyn Fn() -> Result<()>>) -> Result<Self> {

        // crate::dom::register(self);

        let onclick = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(Box::new(move |_event: web_sys::MouseEvent| {
            log_trace!("Link::with_callback called");
            _event.stop_immediate_propagation();
            match callback() {
                Ok(_) => {},
                Err(err) => {
                    log_error!("Error executing MenuItem callback: {:?}", err);
                }
            }
        }));
        // self.element().set_onclick(Some(onclick.as_ref().unchecked_ref()));
        self.element.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())?;
        onclick.forget();
        // *self.onclick.lock().unwrap() = Some(onclick);


        Ok(self)
    }
}

impl workflow_html::Render for Link {
    fn render<W:Write>(&self, w:&mut W) -> core::fmt::Result {
        match self.kind {
            Kind::Module => {
                write!(w, "<a href=\"javascript:void(0)\">{}</a>",self.text)?;
                log_error!("Error: unsupported link to text conversion for internal link: {}",self.text);
            },
            Kind::External => {
                if let Some(href) = &self.href {
                    write!(w, "<a href=\"{}\">{}</a>",href,self.text)?;
                } else {
                    panic!("Link is missing href property: {}",self.text);
                }
            }
        }

        Ok(())
    }

    fn render_node(self, parent:&mut Element, _map:&mut BTreeMap<String, Element>)->ElementResult<()>{
        parent.append_child(&self.element)?;
        Ok(())
    }


}