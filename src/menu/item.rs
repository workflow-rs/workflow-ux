use crate::{icon::Icon, result::Result, prelude::*, error::Error};
use super::{select, Menu, MenuCaption};

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
        Self::new_with_parent_element(parent.sub_ul.clone(),caption,icon)
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

        let icon : Icon = icon.into();
        let icon_el = match icon {
            Icon::Css(name)=>{
                let icon_el = document().create_element("div")?;
                icon_el.set_attribute("icon", &name)?;
                icon_el
            }
            _=>{
                let icon_el = document().create_element("img")?;
                icon_el.set_attribute("src", &icon.to_string())?;
                icon_el
            }
        };
        icon_el.set_attribute("class", "icon")?;

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

