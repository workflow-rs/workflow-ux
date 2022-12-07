use crate::prelude::*;
use workflow_ux::result::Result;
use qrcode::{text_to_qr_with_options, Options};
use workflow_html::{Render, html};

#[derive(Clone)]
pub struct QRCode {
    pub layout : ElementLayout,
    pub element : Element,
    pub code_el : Element,
    pub logo_el : Element,
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

        let tree = html!{
            <div class="workflow-qrcode" @qr_el>
                <div class="qr-code" @qr_code_el></div>
                <div class="qr-logo" @qr_logo_el></div>
            </div>
        }?;

        let content = "".to_string();
        let content = attributes.get("qr_text").unwrap_or(&content);
        let options = Options::from_attributes(attributes)?;
        
        let html = text_to_qr_with_options(&content, &options)?;

        let hooks = tree.hooks();
        let element = hooks.get("qr_el").unwrap().clone();
        let code_el = hooks.get("qr_code_el").unwrap().clone();
        let logo_el = hooks.get("qr_logo_el").unwrap().clone();

        code_el.set_inner_html(&html);
        if let Some(img) = attributes.get("logo"){
            if options.has_logo(){
                logo_el.set_attribute("style", &format!("background-image:url({img})"))?;
            }
        }


        Ok(Self { 
            layout : pane.clone(),
            element,
            code_el,
            logo_el,
            options
        })
    }

    pub fn set_text(&self, text : &str) -> Result<()> {
        self.element.set_inner_html(text);
        Ok(())
    }

}
