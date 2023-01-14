use std::fmt;
use std::sync::Mutex;
use wasm_bindgen::JsCast;
use workflow_ux::error::Error;
use workflow_ux::prelude::*;
use workflow_ux::result::Result;

use crate::attributes::Attributes;
use crate::controls::{form::FormControl, stage_footer};
use crate::docs::Docs;
use crate::markdown::markdown_to_html;

use web_sys::Element;

#[derive(Debug)]
pub enum ElementLayoutStyle {
    Form,
    Stage,
    Section,
    Pane,
    Panel,
    Page,
    Group,
    Html,
}

impl ElementLayoutStyle {
    pub fn get_type(&self) -> &str {
        match self {
            Self::Form => "form",
            Self::Stage => "stage",
            Self::Section => "section",
            Self::Pane => "pane",
            Self::Panel => "panel",
            Self::Page => "page",
            Self::Group => "group",
            Self::Html => "html",
        }
    }
}

impl fmt::Display for ElementLayoutStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElementLayoutStyle::{}", self.get_type())
    }
}

pub trait DefaultFunctions {
    fn init(&self) -> Result<()> {
        Ok(())
    }
    fn validate_stage(&self) -> Result<bool> {
        Ok(true)
    }
    /*
    fn submit(&self) -> Result<bool>{
        log_trace!("DefaultFunctions::submit()");
        Ok(true)
    }
    */
}

pub trait Elemental {
    fn element(&self) -> Element;
}

#[derive(Debug)]
pub struct ElementLayoutInner {
    pub id: Id,
    // pub parent_id: String,
    pub element: Element,
    pub attributes: Attributes,
    pub layout_style: ElementLayoutStyle,
}

// pub struct ElementLayout {
//     inner: Arc<Mutex<ElementLayoutInner>>,
// }
// declare_async_rwlock!(ElementLayout, ElementLayoutInner);
#[derive(Clone)]
pub struct ElementLayout(std::sync::Arc<std::sync::Mutex<ElementLayoutInner>>);

impl ElementLayout {
    pub fn inner(&self) -> Option<std::sync::MutexGuard<ElementLayoutInner>> {
        self.0.lock().ok()
    }
}

impl ElementLayout {
    pub fn element(&self) -> Element {
        // self.element.clone()
        let inner = self
            .inner()
            .expect("ElementLayout::element() failure to lock inner");
        // Ok(
        inner.element.clone()
        // )
        // Ok(inner.element.clone())
    }

    pub fn root_element(&self) -> Result<Element> {
        let inner = self
            .inner()
            .expect("ElementLayout::element() failure to lock inner");
        // let root = get_element_by_id(&inner.parent_id)
        let root = inner
            .element
            .parent_element()
            .ok_or(Error::ParentNotFound(inner.element.clone()))?;
        Ok(root.clone())
    }

    pub fn try_new_for_html() -> Result<ElementLayout> {
        let element = document().create_element("workflow-layout")?;
        let id = Id::new();
        // let id = generate_random_pubkey().to_string();
        element.set_id(&id.to_string());
        element.set_class_name(&format!(
            "{}-container",
            ElementLayoutStyle::Html.get_type()
        ));

        let layout = ElementLayout(Arc::new(Mutex::new(ElementLayoutInner {
            id,
            layout_style: ElementLayoutStyle::Html,
            attributes: Attributes::new(), //attributes.clone(),
            element,
        })));
        Ok(layout)
    }

    // pub fn root(parent_id: &str, layout_style:ElementLayoutStyle) -> Result<ElementLayout> {
    pub fn try_inject(parent: &Element, layout_style: ElementLayoutStyle) -> Result<ElementLayout> {
        // let parent = get_element_by_id(parent_id)
        // .ok_or(error!("ElementLayout::root() - element with parent id `{}` not found", parent_id))?;
        let attributes = Attributes::new();
        // let id = generate_random_pubkey().to_string();
        let id = Id::new();
        let element = document().create_element("layout-root")?;
        element.set_id(&id.to_string());
        parent.append_child(&element)?;

        let layout = ElementLayout(Arc::new(Mutex::new(ElementLayoutInner {
            id: id.into(),
            // parent_id : String::from(parent_id),
            layout_style,
            attributes: attributes.clone(),
            element,
        })));
        Ok(layout)
    }

    pub fn new(
        parent_layout: &ElementLayout,
        layout_style: ElementLayoutStyle,
        attributes: &Attributes,
    ) -> Result<ElementLayout> {
        let parent = parent_layout.inner().unwrap();
        // FIXME use <flow-layout> instead  of <div>
        let element = document().create_element("workflow-layout")?;
        // let id = generate_random_pubkey().to_string();
        if let Some(title) = attributes.get("title") {
            let title_el = document().create_element("h2")?;
            title_el.set_attribute("class", "layout-title")?;
            title_el.set_inner_html(title);
            element.append_child(&title_el)?;
        }

        let id = Id::new();
        element.set_id(&id.to_string());
        element.set_class_name(&format!("{}-container", layout_style.get_type()));
        parent.element.append_child(&element)?;
        let layout = ElementLayout(Arc::new(Mutex::new(ElementLayoutInner {
            id: id.into(),
            // parent_id: parent.id.clone(),
            layout_style,
            attributes: attributes.clone(),
            element,
        })));
        Ok(layout)
    }

    pub fn append_child(
        &self,
        child: &Element,
        attributes: &Attributes,
        docs: &Docs,
    ) -> Result<()> {
        let layout = self
            .inner()
            .ok_or("ElementLayout::append_child() - faulure to lock parent layout inner")?;
        let container = match &layout.layout_style {
            ElementLayoutStyle::Form => None,
            ElementLayoutStyle::Stage => None,
            ElementLayoutStyle::Section => {
                let form_control = FormControl::new()?;
                let mut parse_doc = true;
                if let Some(md_doc) = attributes.get("md_doc") {
                    //form_control.set_title(title)?;
                    parse_doc = !md_doc.eq("false");
                }

                if let Some(title) = attributes.get("title") {
                    form_control.set_title(title)?;
                }

                for (k, v) in attributes.iter() {
                    if !k.eq("title") {
                        if k.eq("no_info") {
                            form_control.set_attribute("no-info", v)?;
                        } else if k.eq("no_icon") {
                            form_control.set_attribute("no-icon", v)?;
                        } else {
                            form_control.set_attribute(k, v)?;
                        }
                    }
                }

                let disposition = child.get_attribute("docs");
                if disposition.is_none() || disposition.unwrap() != "consume" {
                    let mut markdown = docs.join("\n");
                    if parse_doc {
                        markdown = markdown_to_html(&markdown);
                    }
                    //log_trace!("parse_doc: {parse_doc}, {markdown}");
                    form_control.set_info(&markdown)?;
                }

                Some(form_control.element.clone())
            }
            ElementLayoutStyle::Pane => None,
            ElementLayoutStyle::Panel => None,
            ElementLayoutStyle::Page => None,
            ElementLayoutStyle::Group => None,
            ElementLayoutStyle::Html => None,
        };

        match container {
            Some(container) => {
                container.append_child(child)?;
                layout.element.append_child(&container)?;
            }
            None => {
                layout.element.append_child(child)?;
            }
        }

        Ok(())
    }

    pub fn init_footer(&self) -> Result<()> {
        let layout = self
            .inner()
            .ok_or("ElementLayout::init_footer() - failure to lock parent layout inner")?;
        match &layout.layout_style {
            ElementLayoutStyle::Stage | ElementLayoutStyle::Form => {
                let footer = document().create_element("workflow-stage-footer").unwrap();
                let parent = layout.element.parent_element()
                .ok_or("ElementLayout::init_footer() parent_element() - failure to lock parent layout inner")?;
                parent.append_child(&footer)?;
            }
            _ => {}
        };

        Ok(())
    }

    pub fn update_footer(&self, _attributes: &Attributes) -> Result<()> {
        let layout = self
            .inner()
            .ok_or("ElementLayout::update_footer() - failure to lock parent layout inner")?;
        match &layout.layout_style {
            ElementLayoutStyle::Stage => {}
            _ => {}
        };

        Ok(())
    }

    pub fn get_stage_footer(&self) -> Result<stage_footer::StageFooter> {
        let layout = self
            .inner()
            .ok_or("ElementLayout::get_stage_footer() - failure to lock parent layout inner")?;
        let footer_node = layout
            .element
            .parent_element()
            .ok_or("ElementLayout::get_stage_footer() - failure to find parent node")?
            .query_selector("workflow-stage-footer")?
            .ok_or("ElementLayout::get_stage_footer() - failure to find <workflow-stage-footer>")?;
        let footer = match footer_node.dyn_into::<stage_footer::StageFooter>() {
            Ok(footer) => footer,
            Err(err) => {
                // return Err(error!("{:#?}", err));
                // return Err(Error::WebElement(err));
                return Err(err.into());
            }
        };
        return Ok(footer);
    }
}
