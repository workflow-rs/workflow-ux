// use std::collections::BTreeMap;

use crate::layout::ElementLayout;
use crate::prelude::*;
use workflow_ux::error::Error;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct Html {
    pub layout: ElementLayout,
    pub element: Element,
}

impl Html {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(pane: &ElementLayout, _attributes: &Attributes, _docs: &Docs) -> Result<Html> {
        // pane-ctl

        let element = document().create_element("div")?;

        // element.set_attribute("docs", "consume")?;
        // let markdown = docs.join("\n");
        // element.set_inner_html(&markdown);

        Ok(Html {
            layout: pane.clone(),
            element,
        })
    }

    pub fn set_html(&self, html: &workflow_html::Html) -> Result<()> {
        //(Vec<Element>, BTreeMap<String, Element>)) -> Result<()> {
        self.element.set_inner_html("");
        html.inject_into(&self.element)?;
        Ok(())
    }
}

impl<'refs> TryFrom<ElementBindingContext<'refs>> for Html {
    type Error = Error;

    fn try_from(ctx: ElementBindingContext<'refs>) -> Result<Self> {
        Ok(Html {
            layout: ctx.layout.clone(),
            element: ctx.element.clone(),
        })
    }
}
