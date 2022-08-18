use std::{collections::BTreeMap, f64::consts::PI};

use crate::{prelude::*, icon::Icon};
use web_sys::{SvgPathElement, SvgElement};
use workflow_ux::result::Result;
use std::sync::Mutex;

#[wasm_bindgen]
#[derive(Debug)]
pub struct ElPosition{
    pub id:u8,
    pub x:u16,
    pub y:u16,
}

#[derive(Clone)]
pub struct MenuItem{
    pub id:u8,
    pub text:String,
    pub element : Element,
    pub text_el : Element,
    pub circle_el : Element,
    pub icon_el: Element
}

impl MenuItem{
    fn new<I: Into<Icon>>(text:String, icon:I)->Result<Self>{
        let icon_:Icon = icon.into();
        let circle_el = create_svg_element("circle")?;
        circle_el.set_attribute("r", "42")?;
        circle_el.set_attribute("cx", "0")?;
        circle_el.set_attribute("cy", "0")?;

        let icon_el = create_svg_element("image")?;
        icon_el.set_attribute("href", &icon_.to_string())?;
        icon_el.set_attribute("x", "-17")?;
        icon_el.set_attribute("y", "-25")?;
        icon_el.set_attribute("width", "35")?;
        icon_el.set_attribute("height", "35")?;
        icon_el.set_attribute("preserveAspectRatio", "xMidYMid meet")?;

        let text_el = create_svg_element("text")?;
        let text:String = text.into();
        text_el.set_inner_html(&text);
        text_el.set_attribute("text-anchor", "middle")?;
        text_el.set_attribute("x", "0")?;
        text_el.set_attribute("y", "22")?;

        let element = create_svg_element("g")?;
        element.set_attribute("class", "menu")?;
        element.append_child(&circle_el)?;
        element.append_child(&icon_el)?;
        element.append_child(&text_el)?;

        Ok(Self{
            id:Self::get_id(),
            text,
            element,
            text_el,
            circle_el,
            icon_el
        })
    }
    pub fn set_position(&self, x:f32, y:f32)->Result<()>{
        /*
        let x_ = format!("{}", x);
        let y_ = format!("{}", y);
        self.circle_el.set_attribute("cx", &x_)?;
        self.circle_el.set_attribute("cy", &y_)?;
        self.text_el.set_attribute("x", &x_)?;
        self.text_el.set_attribute("y", &y_)?;
        */
        self.element.set_attribute("style", &format!("transform: translate({x}px, {y}px);"))?;
        Ok(())
    }
    fn get_id()->u8{
        static mut ID:u8 = 0;
        unsafe{
            ID = ID+1;
            ID
        }
    }
}

#[derive(Clone)]
pub struct D3Menu {
    pub element : Element,
    svg: Element,
    circle_el:SvgPathElement,
    circle_proxy_el:SvgElement,
    items: BTreeMap<u8, MenuItem>,
    width:i64,
    size:i64,
    large_size:i64,
    value : Rc<RefCell<String>>,
    closed:bool
}

pub fn create_svg_element(name:&str)->std::result::Result<Element, JsValue>{
    document().create_element_ns(Some("http://www.w3.org/2000/svg"), name)
}


impl D3Menu {
    
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(
        layout : &ElementLayout,
        attributes: &Attributes,
        _docs : &Docs
    ) -> Result<Arc<Mutex<D3Menu>>> {
        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        let menu = Self::create_in(&pane_inner.element, Some(attributes))?;
        Ok(menu)
    }

    pub fn create_in(parent:&Element, attributes: Option<&Attributes>)-> Result<Arc<Mutex<D3Menu>>> {
        let doc = document();
        let element = doc
            .create_element("div")?;
        let mut large_size = 1000;
        let (width, height) = match doc.query_selector("body")?{
            Some(body)=>{
                let body_box = body.get_bounding_client_rect();
                let w = body_box.width() as i64;
                let h = body_box.height() as i64;
                if w < h{
                    large_size = h + 300;
                    (w, w)
                }else{
                    large_size = w + 300;
                    (h, h)
                }
            }
            None=>{
                (500, 500)
            }
        };
        let size = -1;
        element.set_attribute("class", "d3-menu")?;
        element.set_attribute("hide", "true")?;
        let svg = create_svg_element("svg")?;
        let view_box = format!("0,0,{width},{height}");
        svg.set_attribute("viewBox", &view_box)?;
        svg.set_attribute("width", "100%")?;
        svg.set_attribute("height", "100%")?;
        svg.set_attribute("preserveAspectRatio", "xMidYMid meet")?;
        element.append_child(&svg)?;

        let circle_el = create_svg_element("path")?
            .dyn_into::<SvgPathElement>()
            .expect("Unable to cast element to SvgPathElement");
        
        circle_el.set_attribute("d", &Self::create_circle_path(width, size))?;
        circle_el.set_attribute("fill", "transparent")?;
        //circle_el.set_attribute("fill", "#F00")?;
        circle_el.set_attribute("class", "close-btn")?;
        svg.append_child(&circle_el)?;

        let circle_proxy_el = create_svg_element("circle")?
            .dyn_into::<SvgElement>()
            .expect("Unable to cast element to SvgElement");
        circle_proxy_el.set_attribute("class", "proxy")?;
        circle_proxy_el.set_attribute("cx", "0")?;
        circle_proxy_el.set_attribute("cy", "-1000")?;
        circle_proxy_el.set_attribute("r", "10")?;
        svg.append_child(&circle_proxy_el)?;

        if let Some(attributes) = attributes{
            for (k,v) in attributes.iter() {
                element.set_attribute(k,v)?;
            }
        }

        let init_value: String = String::from("");
        let value = Rc::new(RefCell::new(init_value));
        parent.append_child(&element)?;
        let menu = D3Menu {
            svg,
            element,
            value,
            circle_el,
            circle_proxy_el,
            items:BTreeMap::new(),
            width,
            size,
            large_size,
            closed:false
        };
        let m = menu.init_event()?;

        Ok(m)
    }

    fn create_circle_path(width:i64, size:i64)->String{
        let dim = size;
        let dim2 = width-size;
        let size_half = size/2;
        format!("M {}, {} a {},{} 0 1,0 {dim},0 a {},{} 0 1,0 {},0", dim2/2, dim2/2+size_half, dim/2, dim/2, dim/2, dim/2, dim*-1)
    }

    fn set_circle_size(&mut self, size:i64)->Result<()>{
        self.size = size;
        self.circle_el.set_attribute("d", &Self::create_circle_path(self.width, size))?;
        Ok(())
    }

    pub fn update(&mut self, size:i64)->Result<()>{
        self.set_circle_size(size)?;

        let mut index:f32 = 0.0;
        let node_count = self.items.len() as f32;

        let circumference = self.circle_el.get_total_length();
        //trace!("circumference: {circumference}, node_count:{node_count}");
        let section_length = circumference/node_count;
        //trace!("size: {}, section_length: {}", size, section_length);

        for (_id, item) in self.items.iter(){
            let position = section_length*index+section_length/ (2 as f32);
            let p = self.circle_el.get_point_at_length(circumference-position)?;
            //trace!("p.y(): {}", p.y());
            item.set_position(p.x(), p.y())?;
            index = index+(1 as f32);
        }
        let cx = match self.circle_proxy_el.get_attribute("cx"){
            Some(d) => if d.eq("400"){"100"}else{"400"},
            None=>"400"
        };
        self.circle_proxy_el.set_attribute("cx", cx)?;

        Ok(())
    }

    /*
    pub fn circle_coord(circle:&SvgPathElement, index:f32, node_count:f32)->Result<SvgPoint>{
        let circumference = circle.get_total_length();
        let section_length = circumference/node_count;
        let position = section_length*index+section_length/ (2 as f32);
        let point = circle.get_point_at_length(circumference-position)?;
        Ok(point)
    }
    */

    fn calc_size(&self)->i64{
        //Math.ceil(98 * 8 / Math.PI)
        //const box_size:f64 = 98.0;
        let size = (98.0 * (self.items.len() as f64)) / PI;
        size.ceil() as i64
    }

    pub fn add_item<T:Into<String>, I: Into<Icon>>(&mut self, text:T, icon:I)->Result<()>{
        let item = MenuItem::new(text.into(), icon)?;

        self.svg.append_child(&item.element)?;
        self.items.insert(item.id, item);
        Ok(())
    }
    pub fn show(&mut self)->Result<()>{
        self.closed = false;
        self.element.remove_attribute("hide")?;
        self.element.set_attribute("closed", "true")?;
        self.element.set_attribute("opening", "true")?;
        Ok(())
    }

    pub fn open(&mut self)->Result<()>{
        self.update(self.large_size)?;
        self.show()?;
        self.update(self.calc_size())?;
        Ok(())
    }
    pub fn close(&mut self)->Result<()>{
        self.closed = true;
        self.update(self.large_size)?;
        self.element.set_attribute("closing", "true")?;
        Ok(())
    }

    fn on_transition_end(&mut self)->Result<()>{
        if self.closed{
            self.element.set_attribute("hide", "true")?;
        }else{
            self.element.remove_attribute("hide")?;
        }
        self.element.remove_attribute("closing")?;
        self.element.remove_attribute("opening")?;
        self.element.remove_attribute("closed")?;
        Ok(())
    }

    fn init_event(self)->Result<Arc<Mutex<Self>>>{
        let this = Arc::new(Mutex::new(self));
        let this_clone = this.clone();
        let self_ = this_clone.lock().expect("Unable to lock D3Menu for click event");
        {
            let _this = this.clone();
            let closure = Closure::wrap(
                Box::new(move |_event: web_sys::TransitionEvent| {
                let mut m = _this.lock().expect("Unable to lock D3Menu for click event");
                let _r = m.on_transition_end();
                log_trace!("##### transitionend");

            }) as Box<dyn FnMut(web_sys::TransitionEvent)>);
            self_.circle_proxy_el.add_event_listener_with_callback("transitionend", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let _this = this.clone();
            let closure = Closure::wrap(
                Box::new(move |_event: web_sys::MouseEvent| {
                let mut m = _this.lock().expect("Unable to lock D3Menu for click event");
                let _r = m.close();

            }) as Box<dyn FnMut(web_sys::MouseEvent)>);
            self_.circle_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        
        Ok(this.clone())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

}
