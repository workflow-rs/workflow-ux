use js_sys::Array;
use workflow_ux::prelude::*;
use workflow_ux::icon::Icon;
use workflow_ux::result::Result;
use workflow_ux::error::Error;

pub type MenuHandlerFn = Box<dyn Fn() -> Result<()>>;

pub struct MenuCaption {
    title: String,
    short: String,
    subtitle: String,
}

impl From<Vec<&str>> for MenuCaption {
    fn from(v: Vec<&str>) -> Self {
        let title = { if v.len() > 0 { v[0] } else { "" } }.to_string();
        let short = { if v.len() > 1 { v[1] } else { "" } }.to_string();
        let subtitle = { if v.len() > 2 { v[2] } else { "" } }.to_string();
        Self { title, short, subtitle }
    }
}

impl From<(&str,&str,&str)> for MenuCaption {
    fn from((title,short,subtitle): (&str,&str,&str)) -> Self {
        Self { 
            title : title.to_string(),
            short : short.to_string(),
            subtitle : subtitle.to_string(), 
        }
    }
}

impl From<(&str,&str)> for MenuCaption {
    fn from((title,short): (&str,&str)) -> Self {
        (title,short,"").into()
    }
}

impl From<&str> for MenuCaption {
    fn from(title: &str) -> Self {
        (title,title,"").into()
    }
}

impl From<String> for MenuCaption {
    fn from(title: String) -> Self {
        (title.as_str(),title.as_str(),"").into()
    }
}
impl From<(String, String)> for MenuCaption {
    fn from(t: (String, String)) -> Self {
        Self {
            title: t.0,
            subtitle: t.1,
            short: "".to_string()
        }
    }
}
impl From<(String, String, String)> for MenuCaption {
    fn from(t: (String, String, String)) -> Self {
        Self {
            title: t.0,
            subtitle: t.1,
            short: t.2
        }
    }
}

pub enum Menu {
    WithId(String),
}

impl ToString for Menu {
    fn to_string(&self) -> String {
        match self {
            Menu::WithId(id) => id.clone(),
        }
    }
}

// impl Menu {
//     pub fn select(&self) -> Result<()> {

//         // TODO - remove selection from all, select only this
//         log_trace!("* * * SELECTING MENU: {}", self.to_string());

//         Ok(())
//     }
// }

pub fn select(target : &Element) -> Result<()> {
    let els = document().query_selector_all(".app-menu .menu-item")?;
    let toggle = Array::new_with_length(1);
    toggle.set(0, JsValue::from("selected"));

    for idx in 0..els.length() {
        els.item(idx).unwrap().dyn_into::<Element>().unwrap().class_list().remove(&toggle)?;
    }

    // if let Some(item) = &self.item {
        target.class_list().add(&toggle)?;
    // }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct MenuGroup {
    element_wrapper: ElementWrapper,
    item : Option<ElementWrapper>,
}

impl MenuGroup {

    pub fn select(&self) -> Result<()> {
        if let Some(item) = &self.item {
            select(&item.element)?;
        }
        Ok(())
    }

    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn from_id(id: &str) -> Result<MenuGroup> {
        let element = document().get_element_by_id(&id)
            .ok_or(Error::MissingElement("WorkspaceMenuGroup::from_id()".into(),id.into()))?;
        let item_opt = element.get_elements_by_tag_name("li").item(0);
        let mut item = None;
        if let Some(el) = item_opt{
            item = Some(ElementWrapper::new(el));
        }
        Ok(MenuGroup {
            element_wrapper: ElementWrapper::new(element),
            item
        })
    }

    // TODO review: id is not used
    pub fn new<I : Into<Icon>>(parent: &MenuGroup, caption: MenuCaption, icon: I) -> Result<MenuGroup> {

        let li = document().create_element("li")?;
        li.set_attribute("class", &format!("menu-item skip-drawer-event"))?;

        let icon_el = document().create_element("img")?;
        icon_el.set_attribute("class", "icon skip-drawer-event")?;
        // icon_el.set_attribute("class", "icon")?;
        let icon : Icon = icon.into();
        icon_el.set_attribute("src", &icon.to_string())?;
    
        let icon_box_el = document().create_element("div")?;
        icon_box_el.set_attribute("class", "icon-box")?;

        let text_box_el = document().create_element("div")?;
        text_box_el.set_attribute("class", "text-box")?;

        let short_title_el = document().create_element("span")?;
        short_title_el.set_attribute("class", "short-title")?;
        if caption.short.len() > 0{
            short_title_el.set_inner_html(&caption.short);
        }else{
            short_title_el.set_inner_html(&caption.title);
        }

        icon_box_el.append_child(&icon_el)?;
        icon_box_el.append_child(&short_title_el)?;
        text_box_el.set_inner_html(&caption.title);

        li.append_child(&icon_box_el)?;
        li.append_child(&text_box_el)?;

        let sub_li = document().create_element("li")?;
        sub_li.set_attribute("class", "sub")?;
        let element = document().create_element("ul")?;
        sub_li.append_child(&element)?;
        parent.element_wrapper.element.append_child(&li)?;
        li.insert_adjacent_element("afterend", &sub_li)?;
        
        Ok(MenuGroup {
            element_wrapper: ElementWrapper::new(element),
            item : Some(ElementWrapper::new(li))
        })

    }

    pub fn with_id<M : Into<Menu>>(&mut self, id: M) -> &mut Self {
        let id : Menu = id.into();
        self.element_wrapper.element.set_id(&id.to_string());
        self
    }

    pub fn with_callback(mut self, callback: Box<dyn Fn(&MenuGroup) -> Result<()>>) -> Result<Self> {
        let self_ = self.clone();
        if let Some(element_wrapper) = &mut self.item{
            element_wrapper.on_click(move |_event| -> Result<()> {
                log_trace!("MenuGroup::with_callback called");
                match callback(&self_) {
                    Ok(_) => {},
                    Err(err) => {
                        log_error!("Error executing MenuItem callback: {:?}", err);
                    }
                };
                Ok(())
            })?;
        }else{
            panic!("MenuGroup::with_callback() unable to bind to menu group without an item");
        }
        Ok(self)
    }

}


#[derive(Debug, Clone)]
pub struct MenuItem {
    element_wrapper: ElementWrapper,
    badge:Option<Element>
}

impl MenuItem {

    pub fn select(&self) -> Result<()> {
        select(&self.element_wrapper.element)?;
        Ok(())
    }


    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn from_id(id: &str) -> Result<MenuItem> {
        let element = document().get_element_by_id(&id)
            .ok_or(Error::MissingElement("WorkspaceMenuItem::from_id()".into(),id.into()))?;
        Ok(MenuItem {
            element_wrapper: ElementWrapper::new(element),
            badge: None
        })
    }

    pub fn new<I : Into<Icon>>(parent: &MenuGroup, caption: MenuCaption, icon: I) -> Result<Self> {
        Self::new_with_parent_element(parent.element(),caption,icon)
    }

    pub fn new_with_parent_id<I : Into<Icon>>(parent_id : &str, caption : MenuCaption, icon: I) -> Result<Self> {
        let element = document().get_element_by_id(&parent_id)
            .ok_or(Error::MissingParent("WorkspaceMenuItem::new_with_id()".into(),parent_id.into()))?;
        Self::new_with_parent_element(element,caption,icon)
    }

    pub fn new_with_parent_element<I : Into<Icon>>(parent: Element, caption : MenuCaption, icon: I) -> Result<Self> {

        let element = document().create_element("li")?;

        let text_box_el = document().create_element("div")?;
        text_box_el.set_attribute("class", "text-box")?;
        text_box_el.set_inner_html(&caption.title);

        let subtitle_el = document().create_element("div")?;
        subtitle_el.set_attribute("class", "sub-title")?;
        if caption.subtitle.len() > 0 {
            subtitle_el.set_inner_html(&caption.subtitle);
        }else{
            subtitle_el.set_inner_html("Default Subtitle");
        }
        text_box_el.append_child(&subtitle_el)?;

        let short_title_el = document().create_element("span")?;
        short_title_el.set_attribute("class", "short-title")?;
        short_title_el.set_inner_html(&caption.short);

        element.set_attribute("class", "menu-item")?; // &format!("menu-item {}", cls))?;

        let icon_el = document().create_element("img")?;
        icon_el.set_attribute("class", "icon")?;
        // let icon_url : String = icon.to_string(); 
        let icon : Icon = icon.into();
        icon_el.set_attribute("src", &icon.to_string())?;

        let icon_box_el = document().create_element("div")?;
        icon_box_el.set_attribute("class", "icon-box")?;
        icon_box_el.append_child(&icon_el)?;
        icon_box_el.append_child(&short_title_el)?;

        element.append_child(&icon_box_el)?;
        element.append_child(&text_box_el)?;
        parent.append_child(&element)?;

        Ok(MenuItem {
            element_wrapper: ElementWrapper::new(element),
            badge: None
        })
    }

    pub fn with_id<M : Into<Menu>>(self, id: M) -> Self {
        let id : Menu = id.into();
        self.element_wrapper.element.set_id(&id.to_string());
        self
    }
    pub fn set_badge(&mut self, num:u64)->Result<()>{
        let badge = match &self.badge {
            Some(badge)=>{
                badge
            },
            None=>{
                let badge = document().create_element("span")?;
                badge.set_attribute("class", "menu-badge")?;
                let icon_box_el_opt = self.element_wrapper.element.query_selector(".icon-box")?;
                if let Some(icon_box_el) = icon_box_el_opt{
                    icon_box_el.append_child(&badge)?;
                    self.badge = Some(badge);
                    self.badge.as_ref().unwrap()
                }else{
                    return Err(Error::MissingIconBox);
                }
            }
        };

        badge.set_inner_html(&format!("{}", num));
        badge.set_attribute("data-badge", &format!("{}", num))?;

        Ok(())
    }

    pub fn with_callback(mut self, callback: Box<dyn Fn(&MenuItem) -> Result<()>>) -> Result<Self> {
        let self_ = self.clone();
        self.element_wrapper.on_click(move |event| ->Result<()> {
            log_trace!("MenuItem::with_callback called");
            event.stop_immediate_propagation();
            
            match callback(&self_) {
                Ok(_) => {},
                Err(err) => {
                    log_error!("Error executing MenuItem callback: {:?}", err);
                }
            };

            Ok(())
        })?;
        Ok(self)
    }
    
}

