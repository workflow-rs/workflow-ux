//use workflow_log::log_trace;

pub use crate::prelude::Element;
use crate::result::Result;

pub use super::MainMenu;
pub use super::{BottomMenu, BottomMenuItem};
pub use super::{PopupMenu, PopupMenuItem};

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppMenu {
    pub main: Arc<MainMenu>,
    pub bottom: Option<Arc<Mutex<BottomMenu>>>,
    pub popup: Option<Arc<PopupMenu>>,
}

impl AppMenu {
    //pub fn element(&self) -> Element {
    //    self.main.element().clone()
    //}

    pub fn new(
        el: &str,
        sub_menu_el: Option<&str>,
        bottom_menu_el: Option<&str>,
        popup_menu_el: Option<&str>,
    ) -> Result<Self> {
        let main = MainMenu::from_el(el, sub_menu_el, None)?;

        let mut popup = None;
        if let Some(popup_menu_el) = popup_menu_el {
            let menu = PopupMenu::from_el(popup_menu_el, None)?;
            popup = Some(menu);
        }

        let mut bottom = None;
        if let Some(bottom_menu_el) = bottom_menu_el {
            let menu = BottomMenu::from_el(bottom_menu_el, None, popup.clone())?;
            bottom = Some(menu);
        }

        Ok(AppMenu {
            main,
            bottom,
            popup,
        })
    }

    pub fn update_bottom_menus(&self, menus: Option<Vec<BottomMenuItem>>) -> Result<()> {
        if let Some(bottom) = self.bottom.as_ref() {
            let m = bottom.clone();
            let mut menu = m.lock().expect("Unable to lock BottomMenu");
            let default_len = menu.default_items.len();
            let mut update_size = 0;
            let mut update_list = Vec::with_capacity(default_len);

            if let Some(items) = menus {
                update_size = items.len().min(default_len);
                for item in items[0..update_size].to_vec() {
                    update_list.push(item);
                }
            }

            for i in update_size..default_len {
                update_list.push(menu.default_items[i].clone());
            }

            //log_trace!("update_bottom_menus: update_list:{:?}", update_list);

            menu.items.clear();
            for item in update_list {
                //log_trace!("BottomMenu: new bottom item: => {:?} : {}", item.text, item.id);
                menu.items.push(item);
            }

            menu.update()?;

            menu.show()?;
        }

        Ok(())
    }
}
