
use crate::{document, result::Result};
use web_sys::{Node, SvgElement};
use wasm_bindgen::JsCast;

pub trait SvgNode{
    fn new(name:&str)->Result<SvgElement>;
    fn set_svg_attribute(&self, name:&str, value:&str)->Result<()>;
    fn set_svg_html(&self, html:&str)->Result<()>;
    fn append_svg_child(&self, child:&Node)->Result<()>;

    fn set_x(self, x:&str) ->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("x", x);
        self
    }
    fn set_y(self, x:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("x", x);
        self
    }

    fn set_cx(self, cx:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("cx", cx);
        self
    }

    fn set_cy(self, cy:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("cy", cy);
        self
    }
    fn set_pos(self, x:&str, y:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("x", x);
        let _ = self.set_svg_attribute("y", y);
        self
    }
    fn set_pos1(self, x1:&str, y1:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("x1", x1);
        let _ = self.set_svg_attribute("y1", y1);
        self
    }
    fn set_pos2(self, x2:&str, y2:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("x2", x2);
        let _ = self.set_svg_attribute("y2", y2);
        self
    }
    fn set_cpos(self, cx:&str, cy:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("cx", cx);
        let _ = self.set_svg_attribute("cy", cy);
        self
    }
    fn set_width(self, width:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("width", width);
        self
    }
    fn set_height(self, height:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("height", height);
        self
    }
    fn set_view_box(self, view_box:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("viewBox", view_box);
        self
    }
    fn set_size(self, width:&str, height:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("width", width);
        let _ = self.set_svg_attribute("height", height);
        self
    }
    fn set_cls(self, cls:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("class", cls);
        self
    }
    fn set_text_anchor(self, text_anchor:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("text-anchor", text_anchor);
        self
    }
    fn set_href(self, href:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("href", href);
        self
    }
    fn set_radius(self, r:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("r", r);
        self
    }
    fn set_aspect_ratio(self, aspect_ratio:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_attribute("preserveAspectRatio", aspect_ratio);
        self
    }
    
    fn set_html(self, html:&str)->Self
    where Self: Sized
    {
        let _ = self.set_svg_html(html);
        self
    }
    fn add_child(self, html:&Node)->Self
    where Self: Sized
    {
        let _ = self.append_svg_child(html);
        self
    }
}


impl SvgNode for SvgElement{
    fn new(name:&str)->Result<SvgElement>{
        let el = document()
        .create_element_ns(Some("http://www.w3.org/2000/svg"), name)?
        .dyn_into::<SvgElement>().expect(&format!("SvgElement::new(): unable to create {name}"));
       Ok(el)
    }
    fn set_svg_attribute(&self, name:&str, value:&str)->Result<()>{
        self.set_attribute(name, value)?;
        Ok(())
    }
    fn set_svg_html(&self, html:&str)->Result<()>{
        self.set_inner_html(html);
        Ok(())
    }
    fn append_svg_child(&self, child:&Node)->Result<()>{
        self.append_child(child)?;
        Ok(())
    }
}
