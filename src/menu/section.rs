use super::{select, Menu, MenuCaption};
use crate::{icon::Icon, prelude::*, result::Result};

#[derive(Debug, Clone)]
pub struct Section {
    pub element: Element,
    pub sub_menu_container: Option<Element>,
}

impl Section {
    pub fn new(parent: &Element, name: &str, sub_menu_container: Option<Element>) -> Result<Self> {
        let element = match parent.query_selector(&format!(".section[section='{}']", name))? {
            Some(el) => el,
            None => {
                let el = create_el("ul", vec![("class", "section"), ("section", name)], None)?;
                parent.append_child(&el)?;
                el
            }
        };
        Ok(Self {
            element,
            sub_menu_container,
        })
    }

    fn add_section_menu(&self, child: SectionMenu) -> Result<SectionMenu> {
        self.element.class_list().add_1("has-child")?;
        let child_el = &child.element_wrapper.element;
        self.element.append_child(child_el)?;
        if let Some(sub_menu_container) = self.sub_menu_container.as_ref() {
            sub_menu_container.append_child(&child.sub_li)?;
        } else {
            child_el.insert_adjacent_element("afterend", &child.sub_li)?;
        }

        //self.child_groups.lock().as_mut().unwrap().push(child.clone());

        Ok(child)
    }
}

#[derive(Debug, Clone)]
pub struct SectionMenu {
    element_wrapper: ElementWrapper, //<li>
    pub id: String,
    pub sub_li: Element, //<li> wrapper of sub_ul
    pub sub_ul: Element, //<ul> for MenuGroup
    pub caption: MenuCaption,
    pub child_groups: Arc<Mutex<Vec<MenuGroup>>>,
    pub sub_menu_container: Option<Element>,
}

impl SectionMenu {
    pub fn select_by_id(id: &str) -> Result<()> {
        match document().query_selector(&format!("[data-id=\"section_menu_{}\"]", id)) {
            Ok(el_opt) => {
                if let Some(el) = el_opt {
                    select(&el)?;
                }
            }
            Err(e) => {
                log_trace!("unable to get section_menu_{}: error:{:?}", id, e);
            }
        }

        match document().query_selector("[data-id=\"sub_menus\"]") {
            Ok(el_opt) => {
                if let Some(sub_menus_container) = el_opt {
                    let sub_menu_id = format!("section_menu_sub_{}", id);
                    let els = sub_menus_container.query_selector_all(".section-menu-sub")?;
                    for idx in 0..els.length() {
                        let sub_menu = els.item(idx).unwrap().dyn_into::<Element>().unwrap();
                        if let Some(id) = sub_menu.get_attribute("data-id") {
                            if id.eq(&sub_menu_id) {
                                sub_menu.class_list().add_1("active")?;
                            } else {
                                sub_menu.class_list().remove_1("active")?;
                            }
                        } else {
                            sub_menu.class_list().remove_1("active")?;
                        }
                    }
                }
            }
            Err(e) => {
                log_trace!("unable to get sub-menus: error:{:?}", e);
            }
        }

        Ok(())
    }

    pub fn select(&self) -> Result<()> {
        select(&self.element_wrapper.element)?;
        if let Some(sub_menu_container) = self.sub_menu_container.as_ref() {
            let els = sub_menu_container.query_selector_all(".section-menu-sub")?;
            for idx in 0..els.length() {
                els.item(idx)
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .class_list()
                    .remove_1("active")?;
            }

            self.sub_li.class_list().add_1("active")?;
        }
        Ok(())
    }

    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn new<I: Into<Icon>>(
        section: &Section,
        caption: MenuCaption,
        icon: I,
    ) -> Result<SectionMenu> {
        let doc = document();
        let id = Self::create_id();
        let li = doc.create_element("li")?;
        li.set_attribute("class", "menu-item skip-drawer-event")?;
        li.set_attribute("data-id", &format!("section_menu_{}", id))?;
        let icon: Icon = icon.into();
        let icon_el = match icon {
            Icon::Css(name) => {
                let icon_el = doc.create_element("div")?;
                icon_el.set_attribute("icon", &name)?;
                icon_el
            }
            _ => {
                let icon_el = doc.create_element("img")?;
                icon_el.set_attribute("src", &icon.to_string())?;
                icon_el
            }
        };
        icon_el.set_attribute("class", "icon skip-drawer-event")?;
        // icon_el.set_attribute("class", "icon")?;

        let icon_box_el = doc.create_element("div")?;
        icon_box_el.set_attribute("class", "icon-box")?;

        //let text_box_el = doc.create_element("div")?;
        //text_box_el.set_attribute("class", "text-box")?;

        let short_title_el = doc.create_element("span")?;
        short_title_el.set_attribute("class", "short-title")?;
        short_title_el.set_inner_html(&caption.title);

        icon_box_el.append_child(&icon_el)?;
        icon_box_el.append_child(&short_title_el)?;
        //text_box_el.set_inner_html(&caption.title);

        li.append_child(&icon_box_el)?;
        //li.append_child(&text_box_el)?;

        let sub_li = doc.create_element("li")?;
        sub_li.set_attribute("class", "sub section-menu-sub")?;
        sub_li.set_attribute("data-id", &format!("section_menu_sub_{}", id))?;
        let sub_ul = doc.create_element("ul")?;
        sub_li.append_child(&sub_ul)?;

        let item = section.add_section_menu(SectionMenu {
            id,
            caption,
            element_wrapper: ElementWrapper::new(li),
            sub_ul,
            sub_li,
            child_groups: Arc::new(Mutex::new(Vec::new())),
            sub_menu_container: section.sub_menu_container.clone(),
        })?;

        Ok(item)
    }

    pub fn add_child_group(&self, child: MenuGroup) -> Result<MenuGroup> {
        self.element_wrapper
            .element
            .class_list()
            .add_1("has-child")?;
        let child_el = &child.item.element;
        self.sub_ul.append_child(child_el)?;
        self.sub_ul.append_child(&child.sub_li)?;

        self.child_groups
            .lock()
            .as_mut()
            .unwrap()
            .push(child.clone());

        Ok(child)
    }

    pub fn with_id<M: Into<Menu>>(&mut self, id: M) -> &mut Self {
        let id: Menu = id.into();
        self.element_wrapper.element.set_id(&id.to_string());
        self
    }

    pub fn with_callback(
        mut self,
        callback: Box<dyn Fn(&SectionMenu) -> Result<()>>,
    ) -> Result<Self> {
        let self_ = self.clone();
        self.element_wrapper.on_click(move |_event| -> Result<()> {
            log_trace!("SectionMenu::with_callback called");
            match callback(&self_) {
                Ok(_) => {}
                Err(err) => {
                    log_error!("Error executing MenuItem callback: {:?}", err);
                }
            };
            Ok(())
        })?;
        Ok(self)
    }

    fn create_id() -> String {
        static mut ID: u8 = 0;
        format!("{}", unsafe {
            ID += 1;
            ID
        })
    }
}
