use std::{collections::BTreeMap, f64::consts::PI};

use crate::{prelude::*, icon::Icon};
use workflow_ux::result::Result;
use crate::controls::svg::SvgNode;
use crate::controls::listener::Listener;
use std::sync::{MutexGuard, LockResult};
use crate::menu::MenuCaption;

/*
#[wasm_bindgen]
#[derive(Debug)]
pub struct ElPosition{
    pub id:u8,
    pub x:u16,
    pub y:u16,
}
*/

#[derive(Debug, Clone)]
pub struct PopupMenuItem{
    pub id: u8,
    pub text: String,
    pub element : SvgElement,
    pub items: Arc<Mutex<BTreeMap<u8, PopupMenuItem>>>,
    pub click_listener: Arc<Mutex<Option<Listener<web_sys::MouseEvent>>>>
}

impl PopupMenuItem{
    pub fn new<I : Into<Icon>>(parent: &PopupMenuItem, caption: MenuCaption, icon: I) -> Result<Self> {
        
        let icon_:Icon = icon.into();

        let circle_el = SvgElement::new("circle").expect("PopupMenuItem: Unable to create circle")
            .set_radius("42")
            .set_cpos("0", "0");

        let icon_el = icon_.svg_element().expect("PopupMenuItem: Unable to create image")
            //.set_href(&icon_.to_string())
            .set_pos("-17", "-25")
            .set_size("35", "35")
            .set_aspect_ratio("xMidYMid meet");

        let text:String = caption.title;
        let text_el = SvgElement::new("text").expect("PopupMenuItem: Unable to create text")
            .set_html(&text)
            .set_text_anchor("middle")
            .set_pos("0", "22");

        let element = SvgElement::new("g").expect("PopupMenuItem: Unable to create root")
            .set_cls("menu")
            .add_child(&circle_el)
            .add_child(&icon_el)
            .add_child(&text_el);

        let item = Self{
            id:Self::get_id(),
            text,
            element,
            click_listener: Arc::new(Mutex::new(None)),
            items: Arc::new(Mutex::new(BTreeMap::new()))
        };

        {
            {
                let mut locked = parent.items.lock().expect("Unable to lock popup_menu/parent.items for new child");
                locked.insert(item.id, item.clone());
            }
            if let Some(menu) = get_popup_menu(){
                menu.append_child_element(&item.element)?;
            }
            //log_trace!("locked.len(): {}, parent:{:?}", locked.len(), parent);
        }

        Ok(item)
    }
    pub fn set_position(&self, x:f32, y:f32)->Result<()>{
        self.element.set_attribute("style", &format!("--menu-x: {x}px; --menu-y: {y}px;"))?;
        Ok(())
    }
    fn get_id()->u8{
        static mut ID:u8 = 0;
        unsafe{
            ID = ID+1;
            ID
        }
    }
    pub fn with_callback(self, callback: Box<dyn Fn(&PopupMenuItem) -> Result<()>>) ->Result<Self>{
        let self_ = self.clone();
        let callback = Listener::new(move|event: web_sys::MouseEvent|->Result<()>{
            log_trace!("PopupMenuItem::with_callback called");
            event.stop_immediate_propagation();
            
            match callback(&self_) {
                Ok(_) => {},
                Err(err) => {
                    log_error!("Error executing PopupMenuItem callback: {:?}", err);
                }
            };

            Ok(())
        });
        self.element.add_event_listener_with_callback("click", callback.into_js())?;
        {
            let mut locked = self.click_listener.lock().expect("Unable to lock popu_pmenu.click_listener");
            (*locked) = Some(callback);
        }
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub struct PopupMenuInner {
    //size: i64,
    closed: bool,
    listeners: Vec<Listener<CustomEvent>>
}

static mut POPUP_MENU : Option<Arc<PopupMenu>> = None;

pub fn get_popup_menu()-> Option<Arc<PopupMenu>>{
    unsafe {POPUP_MENU.clone()}
}

#[derive(Debug, Clone)]
pub struct PopupMenu {
    pub element : Element,
    pub root: PopupMenuItem,
    svg: SvgElement,
    circle_el: SvgPathElement,
    circle_proxy_el: SvgElement,
    width: i64,
    large_size: i64,
    inner: Arc<Mutex<PopupMenuInner>>
}

impl PopupMenu {


    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn from_el(el_selector:&str, attributes: Option<&Attributes>) -> Result<Arc<Self>> {
        let element = find_el(el_selector, "PopupMenu::from_el()")?;
        let menu = Self::create_in(&element, attributes)?;
        Ok(menu)
    }

    pub fn create_in(parent:&Element, attributes: Option<&Attributes>)-> Result<Arc<Self>> {
        let doc = document();
        let element = doc.create_element("div")?;
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
        //let large_size = 400;
        let size = -1;
        element.set_attribute("class", "workflow-popup-menu")?;
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

        parent.append_child(&element)?;

        let root = PopupMenuItem{
            id: PopupMenuItem::get_id(),
            text: "root".to_string(),
            element: SvgElement::new("g").expect("PopupMenuItem: Unable to create root"),
            click_listener: Arc::new(Mutex::new(None)),
            items: Arc::new(Mutex::new(BTreeMap::new()))
        };

        let menu = Self {
            svg,
            element,
            circle_el,
            circle_proxy_el,
            root,
            width,
            large_size,
            inner: Arc::new(Mutex::new(PopupMenuInner {
                //size,
                closed: false,
                listeners: Vec::new()
            }))
        };
        let m = menu.init_event()?;

        unsafe { POPUP_MENU = Some(m.clone()); }

        Ok(m)
    }

    fn create_circle_path(width:i64, size:i64)->String{
        let dim = size;
        let dim2 = width-size;
        let size_half = size/2;
        format!("M {}, {} a {},{} 0 1,0 {dim},0 a {},{} 0 1,0 {},0", dim2/2, dim2/2+size_half, dim/2, dim/2, dim/2, dim/2, dim*-1)
    }

    fn set_circle_size(&self, size:i64)->Result<()>{
        //self.size = size;
        self.circle_el.set_attribute("d", &Self::create_circle_path(self.width, size))?;
        Ok(())
    }

    fn get_root(&self)->&PopupMenuItem{
        &self.root
    }

    pub fn update(&self, size:i64)->Result<()>{
        self.set_circle_size(size)?;

        let mut index:f32 = 0.0;
        let items = self.get_root().items.lock().expect("Unable to lock popup menu items");
        let node_count = items.len() as f32;

        let circumference = self.circle_el.get_total_length();
        //log_trace!("circumference: {circumference}, node_count:{node_count}");
        let section_length = circumference/node_count;
        //log_trace!("size: {}, section_length: {}", size, section_length);

        for (_id, item) in items.iter(){
            //if item.element.parent_element().is_none(){
            //    self.svg.append_child(&item.element)?;
            //}
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
        let items = self.get_root().items.lock().expect("Unable to lock popup menu items");
        let size = (98.0 * (items.len() as f64)) / PI;
        size.ceil() as i64
    }

    pub fn create_item<T:Into<MenuCaption>, I: Into<Icon>>(parent:&PopupMenuItem, text:T, icon:I)->Result<PopupMenuItem>{
        let item = PopupMenuItem::new(parent, text.into(), icon)?;
        Ok(item)
    }
    pub fn add_item(&self, item: PopupMenuItem)->Result<()>{
        //self.svg.append_child(&item.element)?;
        let mut items = self.get_root().items.lock().expect("Unable to lock popup menu items");
        items.insert(item.id, item);
        Ok(())
    }
    fn inner(&self)-> LockResult<MutexGuard<'_, PopupMenuInner>> {
        self.inner.lock()
    }
    fn is_closed(&self)->Result<bool>{
        Ok(self.inner()?.closed)
    }
    fn set_closed(&self, closed:bool)->Result<()>{
        self.inner()?.closed = closed;
        Ok(())
    }
    pub fn show(&self)->Result<()>{
        self.set_closed(false)?;
        self.element.remove_attribute("hide")?;
        self.element.set_attribute("closed", "true")?;
        self.element.set_attribute("opening", "true")?;
        Ok(())
    }

    fn append_child_element(&self, element: &Element)->Result<()>{
        self.svg.append_child(element)?;
        self.update(self.large_size)?;
        Ok(())
    }

    pub fn open_menu(menu:&Arc<Self>)->Result<()>{
        //let mut menu = menu.lock().expect("Unable to lock PopupMenu");
        menu.open()?;

        Ok(())
    }

    pub fn open(&self)->Result<()>{
        self.update(self.large_size)?;
        self.show()?;
        self.update(self.calc_size())?;
        Ok(())
    }
    pub fn close(&self)->Result<()>{
        self.set_closed(true)?;
        self.update(self.large_size)?;
        self.element.set_attribute("closing", "true")?;
        Ok(())
    }

    fn on_transition_end(&self)->Result<()>{
        let closed = self.is_closed()?;
        if closed{
            self.element.set_attribute("hide", "true")?;
        }else{
            self.element.remove_attribute("hide")?;
        }
        self.element.remove_attribute("closing")?;
        self.element.remove_attribute("opening")?;
        self.element.remove_attribute("closed")?;
        Ok(())
    }

    fn init_event(self)->Result<Arc<Self>>{
        let this = Arc::new(self);
        let self_ = this.clone();
        {
            let _this = this.clone();
            let listener = Listener::new(move |_event: web_sys::CustomEvent| -> Result<()> {
                //let mut m = _this.lock().expect("Unable to lock PopupMenu for click event");
                //log_trace!("##### transitionend");
                _this.on_transition_end()?;
                Ok(())
            });
            self_.circle_proxy_el.add_event_listener_with_callback("transitionend", listener.into_js())?;
            let mut inner = self_.inner()?;
            inner.listeners.push(listener);
        }
        {
            let _this = this.clone();
            let listener = Listener::new(move |_event: web_sys::CustomEvent| -> Result<()> {
                //let mut m = _this.lock().expect("Unable to lock PopupMenu for click event");
                _this.close()?;
                Ok(())
            });
            self_.circle_el.add_event_listener_with_callback("click", listener.into_js())?;
            let mut inner = self_.inner()?;
            inner.listeners.push(listener);
        }
        
        Ok(this.clone())
    }


}
