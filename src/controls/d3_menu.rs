use std::{collections::BTreeMap, f64::consts::PI};

use crate::{prelude::*, icon::Icon};
use web_sys::{SvgPathElement, SvgElement};
use workflow_ux::result::Result;
use std::sync::Mutex;
use crate::controls::svg::SvgNode;
use crate::controls::listener::Listener;

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
    pub element : SvgElement,
    pub text_el : SvgElement,
    pub circle_el : SvgElement,
    pub icon_el: SvgElement
}

impl MenuItem{
    fn new<I: Into<Icon>>(text:String, icon:I)->Result<Self>{
        let icon_:Icon = icon.into();

        let circle_el = SvgElement::new("circle").expect("MenuItem: Unable to create circle")
            .set_radius("42")
            .set_cpos("0", "0");

        let icon_el = SvgElement::new("image").expect("MenuItem: Unable to create image")
            .set_href(&icon_.to_string())
            .set_pos("-17", "-25")
            .set_size("35", "35")
            .set_aspect_ratio("xMidYMid meet");

        let text:String = text.into();
        let text_el = SvgElement::new("text").expect("MenuItem: Unable to create text")
            .set_html(&text)
            .set_text_anchor("middle")
            .set_pos("0", "22");

        let element = SvgElement::new("g").expect("MenuItem: Unable to create root")
            .set_cls("menu")
            .add_child(&circle_el)
            .add_child(&icon_el)
            .add_child(&text_el);

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

static mut MENU : Option<Arc<Mutex<D3Menu>>> = None;

pub fn get_menu()->Result<Arc<Mutex<D3Menu>>>{
    let menu_arc = match unsafe {&MENU}{
        Some(menu)=>{
            menu.clone()
        }
        None=>{
            let body = document().body().unwrap();
            let menu_arc = D3Menu::create_in(&body, None)?;
            
            let menu_arc_clone = menu_arc.clone();
            unsafe { MENU = Some(menu_arc.clone()); }

            let mut menu = menu_arc.lock().expect("Unable to lock D3Menu");
            log_trace!("creating menu: {:?}", menu.element);
            
            menu.add_item("Settings", Icon::IconRootSVG("settings".to_string()))?;
            menu.add_item("Work", Icon::IconRootSVG("work".to_string()))?;
            menu.add_item("Ban", Icon::IconRootSVG("ban".to_string()))?;
            
            menu.add_item("Campfire", Icon::IconRootSVG("campfire".to_string()))?;
            menu.add_item("Dao", Icon::IconRootSVG("dao".to_string()))?;
            /*menu.add_item("Classroom", Icon::Classroom)?;
            menu.add_item("CloudUnavailable", Icon::CloudUnavailable)?;
            menu.add_item("Certificate", Icon::Certificate)?;
            menu.add_item("Connected", Icon::Connected)?;
            menu.add_item("Clock", Icon::Clock)?;
            menu.add_item("Funding", Icon::Funding)?;
            */

            menu_arc_clone
        }
    };

    Ok(menu_arc)
}

pub fn show_menu()->Result<()>{
    let m = get_menu()?;
    let mut menu = m.lock().expect("Unable to lock D3Menu");
    let _ = menu.open();
    Ok(())
}

#[derive(Clone)]
pub struct D3Menu {
    pub element : Element,
    svg: SvgElement,
    circle_el:SvgPathElement,
    circle_proxy_el:SvgElement,
    items: BTreeMap<u8, MenuItem>,
    width:i64,
    size:i64,
    large_size:i64,
    value : Rc<RefCell<String>>,
    closed:bool,
    listeners:Vec<Listener<CustomEvent>>
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
        let view_box = format!("0,0,{width},{height}");
        let svg = SvgElement::new("svg")?
            .set_view_box(&view_box)
            .set_size("100%", "100%")
            .set_aspect_ratio("xMidYMid meet");
        element.append_child(&svg)?;

        let circle_el = SvgElement::new("path")?
            .dyn_into::<SvgPathElement>()
            .expect("Unable to cast element to SvgPathElement");
        
        circle_el.set_attribute("d", &Self::create_circle_path(width, size))?;
        circle_el.set_attribute("fill", "transparent")?;
        //circle_el.set_attribute("fill", "#F00")?;
        circle_el.set_attribute("class", "close-btn")?;
        svg.append_child(&circle_el)?;

        let circle_proxy_el = SvgElement::new("circle")?
            .set_cls("proxy")
            .set_cpos("0", "-1000")
            .set_radius("10");
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
            closed:false,
            listeners:Vec::new()
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
        //log_trace!("circumference: {circumference}, node_count:{node_count}");
        let section_length = circumference/node_count;
        //log_trace!("size: {}, section_length: {}", size, section_length);

        for (_id, item) in self.items.iter(){
            let position = section_length*index+section_length/ (2 as f32);
            let p = self.circle_el.get_point_at_length(circumference-position)?;
            //log_trace!("p.y(): {}", p.y());
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
        let mut self_ = this_clone.lock().expect("Unable to lock D3Menu for click event");
        {
            let _this = this.clone();
            let listener = Listener::new(move |_event: web_sys::CustomEvent| -> Result<()> {
                let mut m = _this.lock().expect("Unable to lock D3Menu for click event");
                log_trace!("##### transitionend");
                m.on_transition_end()?;
                Ok(())
            });
            self_.circle_proxy_el.add_event_listener_with_callback("transitionend", listener.into_js())?;
            self_.listeners.push(listener);
        }
        {
            let _this = this.clone();
            let listener = Listener::new(move |_event: web_sys::CustomEvent| -> Result<()> {
                let mut m = _this.lock().expect("Unable to lock D3Menu for click event");
                m.close()?;
                Ok(())
            });
            self_.circle_el.add_event_listener_with_callback("click", listener.into_js())?;
            self_.listeners.push(listener);
        }
        
        Ok(this.clone())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

}
