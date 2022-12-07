use crate::prelude::*;
use workflow_ux::result::Result;
use qrcode::{text_to_qr_with_options, Options};

#[derive(Clone)]
pub struct QRCode {
    pub layout : ElementLayout,
    pub element : Element,
    pub options: Options
}

impl QRCode {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(
        pane : &ElementLayout,
        attributes: &Attributes,
        _docs : &Docs
    ) -> Result<Self> {

        let element = document()
            .create_element("div")?;

        let content = "";
        let options = Options::from_attributes(attributes)?;
        
        let html = text_to_qr_with_options(&content, &options)?;
        element.set_inner_html(&html);

        Ok(Self { 
            layout : pane.clone(),
            element,
            options
        })
    }

    pub fn set_text(&self, text : &str) -> Result<()> {
        self.element.set_inner_html(text);
        Ok(())
    }

}
