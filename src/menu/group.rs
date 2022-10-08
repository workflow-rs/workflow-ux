
use crate::{icon::Icon, result::Result, prelude::*};
use super::{Menu, MenuCaption, section::SectionMenu};

#[derive(Debug, Clone)]
pub struct MenuGroup {
    pub id: String,
    pub item : ElementWrapper,//<li>
    pub sub_li: Element,//<li> wrapper of sub_ul
    pub sub_ul: Element,//<ul></ul> for sub-menus
    pub caption: MenuCaption,
    pub section_menu_id: String,
    pub child_groups: Arc<Mutex<Vec<MenuGroup>>>
}

impl MenuGroup{

    /*
    pub fn select(&self) -> Result<()> {
        select(&self.item.element)?;
        Ok(())
    }
    */

    pub fn is_active(&self)->Result<bool>{
        let active = self.item.element.class_list().contains("active");
        Ok(active)
    }

    pub fn toggle(&self) -> Result<()> {
        if self.is_active()?{
            self.item.element.class_list().remove_1("active")?;
            self.sub_li.class_list().remove_1("active")?;
        }else{
            self.item.element.class_list().add_1("active")?;
            self.sub_li.class_list().add_1("active")?;
        }
        
        Ok(())
    }

    pub fn element(&self) -> Element {
        self.item.element.clone()
    }

    pub fn with_class(self, cls:&str) ->Result<Self>{
        self.item.element.class_list().add_1(cls)?;
        Ok(self)
    }

    // TODO review: id is not used
    pub fn new(section_menu: &SectionMenu, caption: MenuCaption) -> Result<MenuGroup> {
        let doc = document();
        let id = Self::create_id();
        let li = doc.create_element("li")?;
        li.set_attribute("data-id", &format!("menu_group_{}", id))?;
        li.set_attribute("class", &format!("menu-item menu-group skip-drawer-event"))?;

        let text_box_el = doc.create_element("div")?;
        text_box_el.set_attribute("class", "text-box")?;
        text_box_el.set_inner_html(&caption.title);

        let arrow_el = Icon::css("arrow-down-small").element()?;
        arrow_el.class_list().add_1("arrow-icon")?;
        li.append_child(&text_box_el)?;
        li.append_child(&arrow_el)?;

        let sub_li = doc.create_element("li")?;
        sub_li.set_attribute("class", "sub menu-group-items")?;
        let sub_ul = doc.create_element("ul")?;
        sub_li.append_child(&sub_ul)?;
        
        
        let item = section_menu.add_child_group(MenuGroup {
            id,
            section_menu_id: section_menu.id.clone(),
            item : ElementWrapper::new(li.clone()),
            sub_ul,
            sub_li,
            child_groups:Arc::new(Mutex::new(Vec::new())),
            caption
        })?;

        item.toggle()?;

        Ok(item)

    }


    pub fn with_id<M : Into<Menu>>(&mut self, id: M) -> &mut Self {
        let id : Menu = id.into();
        self.item.element.set_id(&id.to_string());
        self
    }

    pub fn with_callback(mut self, callback: Box<dyn Fn(&MenuGroup) -> Result<()>>) -> Result<Self> {
        let self_ = self.clone();
        self.item.on_click(move |_event| -> Result<()> {
            log_trace!("MenuGroup::with_callback called");
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

    fn create_id()->String{
        static mut ID:u8 = 0;
        format!("{}", unsafe{
            ID = ID+1;
            ID
        })
    }

}

