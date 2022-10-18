use crate::prelude::*;
use crate::layout::ElementLayout;
use workflow_ux::result::Result;
use workflow_ux::error::Error;
use crate::markdown::markdown_to_html;

#[derive(Clone)]
pub struct Markdown {
    pub layout : ElementLayout,
    pub element : Element,
}

impl Markdown {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(pane : &ElementLayout, _attributes: &Attributes, docs : &Docs) -> Result<Markdown> {  // pane-ctl

        let element = document()
            .create_element("div")?;
        element.set_attribute("docs", "consume")?;

        let content = docs.join("\n\r");
        let html : String = markdown_to_html(&content);
        element.set_inner_html(&html);

        Ok(Markdown { 
            layout : pane.clone(),
            element,
        })
    }

    pub fn set_text(&self, text : &str) -> Result<()> {
        self.element.set_inner_html(text);
        Ok(())
    }

}

impl<'refs> TryFrom<ElementBindingContext<'refs>> for Markdown {
    type Error = Error;

    fn try_from(ctx : ElementBindingContext<'refs>) -> Result<Self> {

        if ctx.docs.len() != 0 {
            let content = ctx.docs.join("\n");
            let html : String = markdown_to_html(&content);
            ctx.element.set_inner_html(&html);
        }

        Ok(Markdown {
            layout : ctx.layout.clone(),
            element : ctx.element.clone(),
        })

    }
}