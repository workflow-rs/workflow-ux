use super::super::section::Section;
use crate::{find_el, prelude::*, result::Result};

#[derive(Debug, Clone)]
pub struct MainMenu {
    element: Element, //<ul>
    pub default: Section,
    pub actions: Section,
    pub settings: Section,
}

impl MainMenu {
    pub fn default(&self) -> Section {
        self.default.clone()
    }
    pub fn actions(&self) -> Section {
        self.actions.clone()
    }
    pub fn settings(&self) -> Section {
        self.settings.clone()
    }

    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn from_el(
        el_selector: &str,
        sub_menu_el_selector: Option<&str>,
        attributes: Option<&Attributes>,
    ) -> Result<Arc<MainMenu>> {
        let element = find_el(el_selector, "MainMenu::from_el()")?;
        let sub_menu_el = if let Some(sub_menu_el_selector) = sub_menu_el_selector {
            let sub_menu_el = find_el(
                sub_menu_el_selector,
                "MainMenu::from_el():sub_menu_el_selector",
            )?;
            sub_menu_el.set_attribute("data-id", "sub_menus")?;
            Some(sub_menu_el)
        } else {
            None
        };
        let menu = Self::create_in(element, sub_menu_el, attributes)?;
        Ok(menu)
    }

    pub fn create_in(
        element: Element,
        sub_menu_el: Option<Element>,
        attributes: Option<&Attributes>,
    ) -> Result<Arc<MainMenu>> {
        element.class_list().add_1("menu-holder")?;
        if let Some(el) = sub_menu_el.as_ref() {
            el.class_list().add_1("menu-holder")?;
        }
        let default = Section::new(&element, "default", sub_menu_el.clone())?;
        let actions = Section::new(&element, "actions", sub_menu_el.clone())?;
        let settings = Section::new(&element, "settings", sub_menu_el.clone())?;
        if let Some(attributes) = attributes {
            for (k, v) in attributes.iter() {
                element.set_attribute(k, v)?;
            }
        }
        Ok(Arc::new(MainMenu {
            element,
            default,
            actions,
            settings,
        }))
    }
}
