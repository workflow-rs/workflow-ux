use crate::controls::svg::SvgNode;
use crate::{find_el, icon::Icon, prelude::*, result::Result};
use workflow_wasm::prelude::*;

pub fn create_item<T: Into<String>, I: Into<Icon>>(text: T, icon: I) -> Result<BottomMenuItem> {
    BottomMenuItem::new(text.into(), icon)
}
pub fn new_item<T: Into<String>, I: Into<Icon>, F>(text: T, icon: I, t: F) -> Result<BottomMenuItem>
where
    F: FnMut(web_sys::MouseEvent) -> Result<()> + 'static,
{
    let mut item = BottomMenuItem::new(text.into(), icon)?;
    item.on_click(t)?;
    Ok(item)
}

#[derive(Debug, Clone)]
pub struct BottomMenuItem {
    pub id: u8,
    pub text: String,
    pub element: SvgElement,
    pub path_el: SvgElement,
    pub text_el: SvgElement,
    pub circle_el: SvgElement,
    pub icon_el: SvgElement,
    pub click_listener: Option<Callback<dyn FnMut(web_sys::MouseEvent) -> Result<()>>>,
}

impl BottomMenuItem {
    fn new<I: Into<Icon>>(text: String, icon: I) -> Result<Self> {
        let icon_: Icon = icon.into();

        let path_el = SvgElement::try_new("path")
            .expect("BottomMenuItem: Unable to create path")
            .set_cls("slider");
        path_el.set_attribute("d", "M -56 1 l 36 0 c 10 0, 20 0, 20 0 a0 0 0 0 0 0 0 c 0 0 10 0 20 0 l 41 0 l 0 -1 l -117 0 z")?;
        let circle_el = SvgElement::try_new("circle")
            .expect("BottomMenuItem: Unable to create circle")
            .set_radius("30")
            .set_cpos("0", "38");

        let icon_el = icon_
            .svg_element()
            .expect("BottomMenuItem: Unable to create image")
            //.set_href("#svg-icon-work")//&icon_.to_string())
            .set_pos("-15", "17")
            .set_size("30", "30")
            .set_aspect_ratio("xMidYMid meet");

        // let text: String = text.into();
        let text_el = SvgElement::try_new("text")
            .expect("BottomMenuItem: Unable to create text")
            .set_html(&text)
            .set_text_anchor("middle")
            .set_pos("0", "57");

        let element = SvgElement::try_new("g")
            .expect("BottomMenuItem: Unable to create root")
            .set_cls("menu")
            .add_child(&path_el)
            .add_child(&circle_el)
            .add_child(&icon_el)
            .add_child(&text_el);

        Ok(Self {
            id: Self::get_id(),
            text,
            element,
            path_el,
            text_el,
            circle_el,
            icon_el,
            click_listener: None,
        })
    }
    pub fn set_active(&self) -> Result<()> {
        self.path_el.set_attribute("d", "M -56 1 l 0 0 c 10 0, 20 0, 20 34 a36 36 0 0 0 72 0 c 0 -34 10 -34 20 -34 l 5 0 l 0 -1 l -117 0 z")?;
        self.element.set_attribute("class", "menu active")?;
        Ok(())
    }
    pub fn set_position(&self, x: f64, y: f64) -> Result<()> {
        self.element
            .set_attribute("style", &format!("transform: translate({x}px, {y}px);"))?;
        Ok(())
    }
    fn get_id() -> u8 {
        static mut ID: u8 = 0;
        unsafe {
            ID += 1;
            ID
        }
    }
    pub fn on_click<F>(&mut self, t: F) -> Result<()>
    where
        F: FnMut(web_sys::MouseEvent) -> Result<()> + 'static,
    {
        let callback = callback!(t);
        self.element
            .add_event_listener_with_callback("click", callback.as_ref())?;
        self.click_listener = Some(callback);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BottomMenu {
    pub element: Element,
    svg: SvgElement,
    pub items: Vec<BottomMenuItem>,
    pub default_items: Vec<BottomMenuItem>,
    width: f64,
    home_item: BottomMenuItem,
    popup_menu: Option<Arc<PopupMenu>>,
}

impl BottomMenu {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(
        layout: &ElementLayout,
        attributes: &Attributes,
        _docs: &Docs,
    ) -> Result<Arc<Mutex<BottomMenu>>> {
        let pane_inner = layout
            .inner()
            .ok_or_else(|| JsValue::from("unable to mut lock pane inner"))?;
        let menu = Self::create_in(&pane_inner.element, Some(attributes), None)?;
        Ok(menu)
    }

    pub fn from_el(
        el_selector: &str,
        attributes: Option<&Attributes>,
        popup_menu: Option<Arc<PopupMenu>>,
    ) -> Result<Arc<Mutex<BottomMenu>>> {
        let parent = find_el(el_selector, "BottomMenu::from_el()")?;
        let menu = Self::create_in(&parent, attributes, popup_menu)?;
        Ok(menu)
    }

    pub fn create_in(
        parent: &Element,
        attributes: Option<&Attributes>,
        popup_menu: Option<Arc<PopupMenu>>,
    ) -> Result<Arc<Mutex<BottomMenu>>> {
        let doc = document();
        let element = doc.create_element("div")?;
        let (width, height) = {
            let rect_box = parent.get_bounding_client_rect();
            let w = rect_box.width().max(320.0).min(500.0);
            let h = rect_box.height().max(72.0);
            (w, h)
        };
        let width = width + 10.0;
        element.set_attribute("class", "bottom-nav")?;
        element.set_attribute("hide", "true")?;
        let view_box = format!("0,0,{width},{height}");
        let svg = SvgElement::try_new("svg")?
            .set_view_box(&view_box)
            .set_size("100%", &format!("{}", height - 4.0))
            .set_aspect_ratio("xMidYMid meet");
        element.append_child(&svg)?;

        let top_line_el = SvgElement::try_new("line")?
            .set_cls("slider-top-line")
            .set_pos1("-250", "0")
            .set_pos2(&format!("{}", width + 250.0), "0");

        svg.append_child(&top_line_el)?;

        if let Some(attributes) = attributes {
            for (k, v) in attributes.iter() {
                element.set_attribute(k, v)?;
            }
        }

        parent.append_child(&element)?;
        let home_item = create_item("Home", Icon::IconRootSVG("home".to_string()))?;
        home_item.set_active()?;
        let menu = BottomMenu {
            svg,
            element,
            items: Vec::new(),
            default_items: Vec::new(),
            width,
            home_item,
            popup_menu,
        };

        let m = menu.init_event()?;

        Ok(m)
    }

    fn set_circle_size(&mut self) -> Result<()> {
        //self.size = size;
        //self.svg.set_view_box()?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.set_circle_size()?;

        let mut index = 0.0;
        let size = self.width / 5.0;
        let offset = size / 2.0;
        let half_index = self.items.len() as f64 / 2.0;
        let mut add_home_item = self.popup_menu.is_some();
        //log_trace!("BottomMenu: update ========>\n\n");
        for item in &self.items {
            let x = offset + index * size;
            item.set_position(x, 1.0)?;
            //log_trace!("BottomMenu: item.text:{}", item.text);
            self.svg.append_child(&item.element)?;
            index += 1.0;
            if add_home_item && index >= half_index {
                add_home_item = false;
                self.svg.append_child(&self.home_item.element)?;
                let x = offset + index * size;
                self.home_item.set_position(x, 1.0)?;
                index += 1.0;
            }
        }

        Ok(())
    }

    pub fn add_item<T: Into<String>, I: Into<Icon>>(&mut self, text: T, icon: I) -> Result<()> {
        let item = BottomMenuItem::new(text.into(), icon)?;

        //self.svg.append_child(&item.element)?;
        self.items.push(item);
        Ok(())
    }

    pub fn add_default_item<T: Into<String>, I: Into<Icon>>(
        &mut self,
        text: T,
        icon: I,
    ) -> Result<()> {
        let item = BottomMenuItem::new(text.into(), icon)?;

        //self.svg.append_child(&item.element)?;
        self.items.push(item.clone());
        self.default_items.push(item);
        Ok(())
    }

    pub fn add_default_item_with_callback<T: Into<String>, I: Into<Icon>, F>(
        &mut self,
        text: T,
        icon: I,
        t: F,
    ) -> Result<()>
    where
        F: FnMut(web_sys::MouseEvent) -> Result<()> + 'static,
    {
        let mut item = BottomMenuItem::new(text.into(), icon)?;
        item.on_click(t)?;
        self.items.push(item.clone());
        self.default_items.push(item);
        Ok(())
    }

    pub fn show(&mut self) -> Result<()> {
        self.update()?;
        self.element.remove_attribute("hide")?;
        Ok(())
    }

    pub fn on_home_menu_click(&mut self) -> Result<()> {
        if let Some(m) = &self.popup_menu {
            PopupMenu::open_menu(m)?;
        }
        Ok(())
    }

    fn init_event(self) -> Result<Arc<Mutex<Self>>> {
        let this = Arc::new(Mutex::new(self));
        let mut self_ = this
            .lock()
            .expect("Unable to lock BottomMenu for click event");
        {
            let _this = this.clone();
            self_.home_item.on_click(move |_event| -> Result<()> {
                let mut m = _this
                    .lock()
                    .expect("Unable to lock BottomMenu for click event");
                m.on_home_menu_click()?;
                Ok(())
            })?;
        }

        Ok(this.clone())
    }
}
