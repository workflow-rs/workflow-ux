use crate::app_menu::AppMenu;
use crate::find_el;
use crate::result::Result;
use crate::view::{Container, ContainerStack};
use std::sync::Arc;

pub struct Workspace {
    pub header: Arc<Container>,
    pub menu: Arc<AppMenu>,
    pub status: Arc<Container>,
    pub main: Arc<Container>,
    pub sidebar: Arc<ContainerStack>,
}

impl Workspace {
    pub fn new(
        header_el: &str,
        status_el: &str,
        main_el: &str,
        sidebar_el: &str,
        menu: Arc<AppMenu>, //menu_el: &str,
                            //bottom_menu_el: &str
    ) -> Result<Workspace> {
        //let menu = Arc::new(AppMenu::new(menu_el, bottom_menu_el)?);

        let header_ele = find_el(header_el, "missing workspace header element")?;
        let header = Arc::new(Container::new(header_ele, None));

        let status_ele = find_el(status_el, "missing workspace status element")?;
        let status = Arc::new(Container::new(status_ele, None));

        let main_ele = find_el(main_el, "missing workspace main element")?;
        let main = Arc::new(Container::new(main_ele, Some(menu.clone())));

        let sidebar_ele = find_el(sidebar_el, "missing workspace sidebar element")?;
        let sidebar = Arc::new(ContainerStack::new(sidebar_ele));

        let workspace = Workspace {
            header,
            menu,
            status,
            main,
            sidebar,
        };

        Ok(workspace)
    }

    pub fn header(&self) -> Arc<Container> {
        self.header.clone()
    }

    pub fn menu(&self) -> Arc<AppMenu> {
        self.menu.clone()
    }

    pub fn status(&self) -> Arc<Container> {
        self.status.clone()
    }

    pub fn main(&self) -> Arc<Container> {
        self.main.clone()
    }

    pub fn sidebar(&self) -> Arc<ContainerStack> {
        self.sidebar.clone()
    }
}
