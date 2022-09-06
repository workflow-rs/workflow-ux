use crate::error;
use crate::{prelude::*, icon::Icon};
use web_sys::SvgElement;
use workflow_ux::result::Result;
use std::sync::Mutex;
use crate::controls::svg::SvgNode;
use crate::controls::listener::Listener;

static mut BOTTOM_MENU : Option<Arc<Mutex<BottomMenu>>> = None;

pub fn get_bottom_menu()-> Result<Arc<Mutex<BottomMenu>>>{
    let menu_arc = match unsafe {&BOTTOM_MENU}{
        Some(menu)=>{
            menu.clone()
        }
        None=>{
            //let body = document().body().unwrap();
            let parent = match document().query_selector("#workspace-bottom-nav").expect("#workspace-bottom-nav is missing"){
                Some(el)=>el,
                None=>{
                    return Err(error!("#workspace-bottom-nav element is required for bottom nav"))
                }
            };
            let menu_arc = bottom_menu::BottomMenu::create_in(&parent, None)?;
            
            let menu_arc_clone = menu_arc.clone();
            unsafe { BOTTOM_MENU = Some(menu_arc.clone()); }

            let mut menu = menu_arc.lock().expect("Unable to lock BottomMenu");
            log_trace!("creating bottom menu: {:?}", menu.element);

            menu.add_default_item("Settings", Icon::svg("settings"))?;
            menu.add_default_item("Work", Icon::svg("work"))?;
            menu.add_default_item("Ban", Icon::svg("ban"))?;
            menu.add_default_item("Clock", Icon::svg("clock"))?;

            menu_arc_clone
        }
    };
    Ok(menu_arc)
    //let mut menu = menu_arc.lock().expect("Unable to lock BottomMenu");
    //let _ = menu.show();
    //Ok(())
}

pub fn update_menus(menus:Option<Vec<BottomMenuItem>>)->Result<()>{
    let m = get_bottom_menu()?;
    let mut menu = m.lock().expect("Unable to lock BottomMenu");
    let default_len = menu.default_items.len();
    let mut update_size = 0;
    let mut update_list = Vec::with_capacity(default_len);

    
    if let Some(items) = menus{
        update_size = items.len().min(default_len);
        for item in items[0..update_size].to_vec(){
            update_list.push(item);
        }
    }

    for i in update_size..default_len{
        update_list.push(menu.default_items[i].clone());
    }

    menu.items.clear();
    for item in update_list{
        //log_trace!("BottomMenu: new bottom item: => {:?} : {}", item.text, item.id);
        menu.items.push(item);
    }

    menu.update()?;


    Ok(())
}

pub fn create_item<T:Into<String>, I: Into<Icon>>(text:T, icon:I)->Result<BottomMenuItem>{
    Ok(BottomMenuItem::new(text.into(), icon)?)
}
pub fn new_item<T:Into<String>, I: Into<Icon>, F>(text:T, icon:I, t:F)->Result<BottomMenuItem>
where F: FnMut(web_sys::MouseEvent) ->Result<()> + 'static
{
    let mut item = BottomMenuItem::new(text.into(), icon)?;
    item.on_click(t)?;
    Ok(item)
}

#[derive(Clone)]
pub struct BottomMenuItem{
    pub id:u8,
    pub text:String,
    pub element : SvgElement,
    pub text_el : SvgElement,
    pub circle_el : SvgElement,
    pub icon_el: SvgElement,
    pub click_listener:Option<Listener<web_sys::MouseEvent>>
}

impl BottomMenuItem{
    fn new<I: Into<Icon>>(text:String, icon:I)->Result<Self>{
        let icon_:Icon = icon.into();

        let path_el = SvgElement::new("path").expect("BottomMenuItem: Unable to create path")
            .set_cls("slider");
        let circle_el = SvgElement::new("circle").expect("BottomMenuItem: Unable to create circle")
            .set_radius("30")
            .set_cpos("0", "38");

        let icon_el = SvgElement::new("image").expect("BottomMenuItem: Unable to create image")
            .set_href(&icon_.to_string())
            .set_pos("-15", "17")
            .set_size("30", "30")
            .set_aspect_ratio("xMidYMid meet");

        let text:String = text.into();
        let text_el = SvgElement::new("text").expect("BottomMenuItem: Unable to create text")
            .set_html(&text)
            .set_text_anchor("middle")
            .set_pos("0", "57");

        let element = SvgElement::new("g").expect("BottomMenuItem: Unable to create root")
            .set_cls("menu")
            .add_child(&path_el)
            .add_child(&circle_el)
            .add_child(&icon_el)
            .add_child(&text_el);

        Ok(Self{
            id:Self::get_id(),
            text,
            element,
            text_el,
            circle_el,
            icon_el,
            click_listener:None
        })
    }
    pub fn set_active(&self){
        let _r = self.element.set_attribute("class", "menu active");
    }
    pub fn set_position(&self, x:f64, y:f64)->Result<()>{
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
    pub fn on_click<F>(&mut self, t:F) ->Result<()>
    where
        F: FnMut(web_sys::MouseEvent) ->Result<()> + 'static
    {
        let callback = Listener::new(t);
        self.element.add_event_listener_with_callback("click", callback.into_js())?;
        self.click_listener = Some(callback);
        Ok(())
    }
}

#[derive(Clone)]
pub struct BottomMenu {
    pub element : Element,
    svg: SvgElement,
    items: Vec<BottomMenuItem>,
    default_items: Vec<BottomMenuItem>,
    width:f64,
    value : Rc<RefCell<String>>,
    home_item: BottomMenuItem
}

impl BottomMenu {
    
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(
        layout : &ElementLayout,
        attributes: &Attributes,
        _docs : &Docs
    ) -> Result<Arc<Mutex<BottomMenu>>> {
        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        let menu = Self::create_in(&pane_inner.element, Some(attributes))?;
        Ok(menu)
    }

    pub fn create_in(parent:&Element, attributes: Option<&Attributes>)-> Result<Arc<Mutex<BottomMenu>>> {
        let doc = document();
        let element = doc.create_element("div")?;
        let (width, height) = {
            let rect_box = parent.get_bounding_client_rect();
            let w = rect_box.width().max(320.0);
            let h = rect_box.height().max(72.0);
            (w, h)
        };
        let width = width+10.0;
        element.set_attribute("class", "bottom-nav")?;
        element.set_attribute("hide", "true")?;
        let view_box = format!("0,0,{width},{height}");
        let svg = SvgElement::new("svg")?
            .set_view_box(&view_box)
            .set_size("100%", &format!("{}", height - 4.0))
            .set_aspect_ratio("xMidYMid meet");
        element.append_child(&svg)?;

        let top_line_el = SvgElement::new("line")?
            .set_cls("slider-top-line")
            .set_pos1("-250", "0")
            .set_pos2(&format!("{}", width+250.0), "0");
        
        svg.append_child(&top_line_el)?;

        if let Some(attributes) = attributes{
            for (k,v) in attributes.iter() {
                element.set_attribute(k,v)?;
            }
        }

        let init_value: String = String::from("");
        let value = Rc::new(RefCell::new(init_value));
        parent.append_child(&element)?;
        let home_item = create_item("Home", Icon::IconRootSVG("home".to_string()))?;
        home_item.set_active();
        let menu = BottomMenu {
            svg,
            element,
            value,
            items: Vec::new(),
            default_items: Vec::new(),
            width,
            home_item
        };

        let m = menu.init_event()?;

        Ok(m)
    }


    fn set_circle_size(&mut self)->Result<()>{
        //self.size = size;
        //self.svg.set_view_box()?;
        Ok(())
    }

    pub fn update(&mut self)->Result<()>{
        self.set_circle_size()?;

        let mut index = 0.0;
        let size = self.width / 5.0;
        let offset = size/2.0;
        let half_index = self.items.len() as f64 / 2.0;
        let mut home_item_added = false;
        //log_trace!("BottomMenu: update ========>\n\n");
        for item in &self.items{
            let x = offset + index * size;
            item.set_position(x, 1.0)?;
            //log_trace!("BottomMenu: item.text:{}", item.text);
            self.svg.append_child(&item.element)?;
            index = index+1.0;
            if !home_item_added && index >= half_index{
                home_item_added = true;
                self.svg.append_child(&self.home_item.element)?;
                let x = offset + index * size;
                self.home_item.set_position(x, 1.0)?;
                index = index+1.0;
            }
        }

        Ok(())
    }

    pub fn add_item<T:Into<String>, I: Into<Icon>>(&mut self, text:T, icon:I)->Result<()>{
        let item = BottomMenuItem::new(text.into(), icon)?;

        //self.svg.append_child(&item.element)?;
        self.items.push(item);
        Ok(())
    }

    fn add_default_item<T:Into<String>, I: Into<Icon>>(&mut self, text:T, icon:I)->Result<()>{
        let item = BottomMenuItem::new(text.into(), icon)?;

        //self.svg.append_child(&item.element)?;
        self.items.push(item.clone());
        self.default_items.push(item);
        Ok(())
    }

    pub fn show(&mut self)->Result<()>{
        self.update()?;
        Ok(())
    }

    pub fn on_home_menu_click(&mut self)->Result<()>{
        //let m = d3_menu::get_menu()?;
        //let mut menu = m.lock().expect("Unable to lock D3Menu");
        //let _ = menu.show();
        d3_menu::show_menu()?;
        Ok(())
    }

    fn init_event(self)->Result<Arc<Mutex<Self>>>{
        let this = Arc::new(Mutex::new(self));
        let mut self_ = this.lock().expect("Unable to lock BottomMenu for click event");
        {
            let _this = this.clone();
            self_.home_item.on_click(move |_event| ->Result<()>{
                let mut m = _this.lock().expect("Unable to lock BottomMenu for click event");
                m.on_home_menu_click()?;
                Ok(())
            })?;
        }
        
        Ok(this.clone())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

}
