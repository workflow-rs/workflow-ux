use crate::prelude::*;
use crate::layout::ElementLayout;
use workflow_ux::result::Result;
use workflow_ux::error::Error;

#[derive(Clone)]
pub struct Text {
    pub layout : ElementLayout,
    pub element : Element,
}

impl Text {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(pane : &ElementLayout, _attributes: &Attributes, docs : &Docs) -> Result<Text> {  // pane-ctl

        let element = document()
            .create_element("div")?;
        element.set_attribute("docs", "consume")?;

        let content = docs.join("\n");
        let html : String = ::markdown::to_html(&content);
        element.set_inner_html(&html);

        Ok(Text { 
            layout : pane.clone(),
            element,
        })
    }

    pub fn set_text(&self, text : &str) -> Result<()> {
        self.element.set_inner_html(text);
        Ok(())
    }

}

impl<'refs> TryFrom<ElementBindingContext<'refs>> for Text {
    type Error = Error;

    fn try_from(ctx : ElementBindingContext<'refs>) -> Result<Self> {
        Ok(Text {
            layout : ctx.layout.clone(),
            element : ctx.element.clone(),
        })

    }
}