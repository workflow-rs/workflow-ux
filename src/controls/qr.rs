use crate::prelude::*;
use workflow_ux::result::Result;
pub use qrcode::{text_to_qr_with_options, Options};
use workflow_html::{Render, html, ElementResult, Renderables, Hooks};

#[derive(Clone)]
pub struct QRCode {
    //pub layout : ElementLayout,
    pub element : Element,
    pub code_el : Element,
    //pub text_el : Element,
    pub options: Options
}

impl QRCode {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(
        _pane : &ElementLayout,
        attributes: &Attributes,
        _docs : &Docs
    ) -> Result<Self> {
        let content = "".to_string();
        let text = attributes.get("qr_text").unwrap_or(&content);
        let options = Options::from_attributes(attributes)?;
        let control = Self::create(text, options)?;
        Ok(control)
    }

    pub fn create(text:&str, options: Options)->Result<Self> {
        let tree = html!{
            <div class="workflow-qrcode" @qr_el>
                <div class="qr-code" @qr_code_el></div>
            </div>
        }?;
        
        let svg = text_to_qr_with_options(text, &options)?;
        //options.logo = None;
        //let svg2 = text_to_qr_with_options(&content, &options)?;
        
        let hooks = tree.hooks();
        let element = hooks.get("qr_el").unwrap().clone();
        let code_el = hooks.get("qr_code_el").unwrap().clone();
        //let text_el = hooks.get("qr_text_el").unwrap().clone();

        //text_el.set_inner_html(&content);
        code_el.set_inner_html(&svg);


        Ok(Self { 
            //layout : pane.clone(),
            element,
            code_el,
            //text_el,
            options
        })
    }

    pub fn set_text(&self, text : &str) -> Result<()> {
        self.element.set_inner_html(text);
        Ok(())
    }

}


impl Render for QRCode {
    fn render(&self, _w:&mut Vec<String>) -> workflow_html::ElementResult<()> {
        Ok(())
    }

    fn render_node(
        self,
        parent:&mut Element,
        _map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>{
        parent.append_child(&self.element)?;
        renderables.push(Arc::new(self));
        Ok(())
    }
}