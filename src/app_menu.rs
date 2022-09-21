
//use workflow_log::log_trace;

use crate::result::Result;
use crate::menu::MenuGroup;
use crate::bottom_menu::{BottomMenu, BottomMenuItem};
use crate::prelude::Element;
use crate::popup_menu::PopupMenu;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppMenu {
    pub main : MenuGroup,
    pub bottom : Arc<Mutex<BottomMenu>>,
    pub popup: Option<Arc<PopupMenu>>
}

impl AppMenu {

    pub fn element(&self) -> Element {
        self.main.element().clone()
    }

    pub fn new(el: &str, bottom_menu_el: &str, popup_menu_el:Option<&str>) -> Result<Self> {

        let main = MenuGroup::from_el(el)?;
        
        let mut popup = None;
        if let Some(popup_menu_el) = popup_menu_el{
            let menu = PopupMenu::from_el(popup_menu_el, None)?;
            popup = Some(menu);
        }

        let bottom = BottomMenu::from_el(bottom_menu_el, None, popup.clone())?;
        Ok(AppMenu {
            main,
            bottom,
            popup
        })
    }

    pub fn update_bottom_menus(&self, menus:Option<Vec<BottomMenuItem>>)->Result<()>{
        let m = self.bottom.clone();
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

        //log_trace!("update_bottom_menus: update_list:{:?}", update_list);
    
        menu.items.clear();
        for item in update_list{
            //log_trace!("BottomMenu: new bottom item: => {:?} : {}", item.text, item.id);
            menu.items.push(item);
        }
    
        menu.update()?;

        menu.show()?;
    
    
        Ok(())
    }

}
