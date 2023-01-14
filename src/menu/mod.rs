use crate::prelude::*;
use crate::result::Result;
mod types;

pub static CSS: &'static str = include_str!("menu.css");

pub mod caption;
pub mod group;
pub mod item;
pub mod section;

pub use types::bottom as bottom_menu;
pub use types::main as main_menu;
pub use types::popup as popup_menu;

pub use bottom_menu::{BottomMenu, BottomMenuItem};
pub use main_menu::MainMenu;
pub use popup_menu::{PopupMenu, PopupMenuItem};
pub use section::{Section, SectionMenu};

pub use caption::MenuCaption;
pub use group::MenuGroup;
pub use item::MenuItem;

pub mod app_menu;

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

pub fn select(target: &Element) -> Result<()> {
    if let Some(holder) = target.closest(".menu-holder")? {
        let els = holder.query_selector_all(".menu-item.selected")?;

        for idx in 0..els.length() {
            els.item(idx)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap()
                .class_list()
                .remove_1("selected")?;
        }

        target.class_list().add_1("selected")?;
    }
    Ok(())
}
