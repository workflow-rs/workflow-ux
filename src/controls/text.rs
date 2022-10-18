use crate::prelude::*;
use crate::result::Result;
use crate::error::Error;
use crate::markdown::markdown_to_html;

#[derive(Clone)]
pub struct Text {
    pub layout : ElementLayout,
    pub element : Element,
}

unsafe impl Send for Text{}

impl Text {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(pane : &ElementLayout, attributes: &Attributes, docs : &Docs) -> Result<Text> {  // pane-ctl

        let element = document()
            .create_element("div")?;
        element.set_attribute("docs", "consume")?;

        let content = docs.join("\n");
        let html : String = markdown_to_html(&content);//::markdown::to_html(&content);
        element.set_inner_html(&html);

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }

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
