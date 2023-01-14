#[allow(unused_imports)]
use std::sync::Mutex;
use workflow_html::{ElementResult, Hooks, Renderables};
use workflow_log::*;
use workflow_ux::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone, Debug)]
pub enum Kind {
    Module,
    External,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub element: Element,
    pub kind: Kind,
    pub text: String,
    pub href: Option<String>,
    pub _onclick: Arc<Mutex<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>>,
}

impl std::default::Default for Link {
    fn default() -> Self {
        Self {
            element: document()
                .create_element("a")
                .expect("Could not create Link Element"),
            kind: Kind::Module,
            text: "Click Me".to_string(),
            href: None,
            _onclick: Arc::new(Mutex::new(None)),
        }
    }
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

    pub fn new_for_callback(text: &str) -> Result<Self> {
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
            kind: Kind::Module,
            text: text.to_string(),
            href: None,
            element,
            _onclick: Arc::new(Mutex::new(None)),
        })
    }

    pub fn new_with_url(text: &str, href: &str) -> Result<Self> {
        let element = document().create_element("a")?;
        element.set_attribute("href", href)?;
        element.set_attribute("target", "_blank")?;
        element.set_inner_html(text);

        Ok(Link {
            kind: Kind::External,
            text: text.to_string(),
            href: Some(href.to_string()),
            element,
            _onclick: Arc::new(Mutex::new(None)),
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

        let onclick = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(Box::new(
            move |_event: web_sys::MouseEvent| {
                log_trace!("Link::with_callback called");
                _event.stop_immediate_propagation();
                match callback() {
                    Ok(_) => {}
                    Err(err) => {
                        log_error!("Error executing MenuItem callback: {:?}", err);
                    }
                }
            },
        ));
        // self.element().set_onclick(Some(onclick.as_ref().unchecked_ref()));
        self.element
            .add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())?;
        *self._onclick.lock().unwrap() = Some(onclick);

        Ok(self)
    }
}

impl workflow_html::Render for Link {
    fn render(&self, w: &mut Vec<String>) -> workflow_html::ElementResult<()> {
        match self.kind {
            Kind::Module => {
                w.push(format!("<a href=\"javascript:void(0)\">{}</a>", self.text));
                log_error!(
                    "Error: unsupported link to text conversion for internal link: {}",
                    self.text
                );
            }
            Kind::External => {
                if let Some(href) = &self.href {
                    w.push(format!("<a href=\"{}\">{}</a>", href, self.text));
                } else {
                    panic!("Link is missing href property: {}", self.text);
                }
            }
        }

        Ok(())
    }

    fn render_node(
        self,
        parent: &mut Element,
        _map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        if let Some(href) = &self.href {
            self.element.set_attribute("href", href)?;
        }
        self.element.set_inner_html(&self.text);
        parent.append_child(&self.element)?;
        renderables.push(Arc::new(self));
        Ok(())
    }
}
