// use web_sys::Element;
use workflow_ux::prelude::*;

pub trait Control {
    fn element(&self) -> Element;
}

pub struct ElementBindingContext<'refs> {
    pub layout : &'refs ElementLayout,
    pub element : &'refs Element,
    pub attributes : &'refs Attributes,
    pub docs : &'refs Docs,
}

impl<'refs> ElementBindingContext<'refs> {
    pub fn new(layout : &'refs ElementLayout, element: &'refs Element, attributes: &'refs Attributes, docs: &'refs Docs) -> ElementBindingContext<'refs> {
        ElementBindingContext { 
            layout,
            element,
            attributes,
            docs
        }
    }
}