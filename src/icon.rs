//use crate::dom::Element;
use crate::prelude::{Arc, Mutex, Theme};
use crate::theme::current_theme_folder;
use crate::{controls::svg::SvgNode, document, result::Result};
use regex::Regex;
use std::collections::BTreeMap;
use web_sys::{Element, SvgElement};
pub type IconInfoMap = BTreeMap<String, IconInfo>;

pub struct IconInfo {
    pub file_name: String,
    pub is_svg: bool,
}
impl IconInfo {
    fn new(file_name: String) -> Self {
        Self {
            file_name,
            is_svg: false,
        }
    }
    fn new_svg(file_name: String) -> Self {
        Self {
            file_name,
            is_svg: true,
        }
    }
}

static mut ICON_ROOT_URL: Option<String> = None;

static mut ICONS: Option<Arc<Mutex<IconInfoMap>>> = None;

pub fn icon_root() -> String {
    unsafe {
        ICON_ROOT_URL
            .as_ref()
            .expect("Icon root is not initialized")
            .clone()
    }
}
pub fn get_icons() -> Arc<Mutex<IconInfoMap>> {
    match unsafe { ICONS.as_ref() } {
        Some(icons) => icons.clone(),
        None => {
            let icons = Arc::new(Mutex::new(BTreeMap::new()));
            unsafe {
                ICONS = Some(icons.clone());
            }
            icons
        }
    }
}
pub fn track_icon<T: Into<String>>(id: T, icon: IconInfo) {
    let id_str: String = id.into();
    let icons = get_icons();
    {
        let mut locked = icons
            .lock()
            .unwrap_or_else(|_| panic!("unable to lock icons list for tracking `{id_str}`"));
        if let Some(icon) = locked.get_mut(&id_str) {
            if !icon.is_svg {
                // FIXME
                // icon.is_svg = icon.is_svg;
            }
        } else {
            locked.insert(id_str, icon);
        }
    }

    Theme::update_theme_content_icons(icons);
}

// #[wasm_bindgen]
pub fn init_icon_root(icon_root: &str) -> Result<()> {
    let mut icon_root = icon_root.to_string();
    if icon_root.ends_with('/') {
        icon_root.pop();
    }
    unsafe {
        ICON_ROOT_URL = Some(icon_root);
    }
    Ok(())
}

pub fn icon_folder() -> String {
    format!("{}/{}", icon_root(), current_theme_folder())
}

pub enum Icon {
    Url(String),
    IconRootCustom(String),
    IconRootSVG(String),
    Css(String),
}

impl Icon {
    pub fn css<T: Into<String>>(icon: T) -> Icon {
        let icon = Self::Css(icon.into().to_lowercase());
        let (file_name, id) = icon.get_file_name_and_id();
        track_icon(id, IconInfo::new(file_name));

        icon
    }
    pub fn svg<T: Into<String>>(icon: T) -> Icon {
        Self::IconRootSVG(icon.into())
    }
    pub fn url<T: Into<String>>(icon: T) -> Icon {
        Self::Url(icon.into())
    }
    pub fn custom<T: Into<String>>(icon: T) -> Icon {
        Self::IconRootCustom(icon.into())
    }

    pub fn get_file_name_and_id(&self) -> (String, String) {
        match self {
            Icon::Url(url) => (
                url.clone(),
                Regex::new("[^a-z0-9]{1,}")
                    .unwrap()
                    .replace_all(&url.to_lowercase(), "-")
                    .to_string(),
            ),
            Icon::IconRootCustom(name) => (
                name.to_lowercase(),
                Regex::new("[^a-z0-9]{1,}")
                    .unwrap()
                    .replace_all(&name.to_lowercase(), "-")
                    .to_string(),
            ),
            Icon::IconRootSVG(name) | Icon::Css(name) => (
                format!("{}.svg#icon", name.to_lowercase()),
                name.to_lowercase(),
            ),
        }
    }

    pub fn svg_element(&self) -> Result<SvgElement> {
        let el = SvgElement::try_new("use")?;
        let (file_name, id) = self.get_file_name_and_id();
        track_icon(&id, IconInfo::new_svg(file_name));
        Ok(el.set_href(&format!("#svg-icon-{id}")))
    }
    pub fn element(&self) -> Result<Element> {
        let el = match self {
            Icon::Css(name) => {
                let icon_el = document().create_element("div")?;
                icon_el.set_attribute("icon", name)?;
                icon_el.set_attribute("class", "icon")?;
                icon_el
            }
            _ => {
                let icon_el = document().create_element("img")?;
                icon_el.set_attribute("src", &self.to_string())?;
                icon_el.set_attribute("class", "icon")?;
                icon_el
            }
        };

        Ok(el)
    }
}

fn custom(name: &str) -> String {
    format!("{}/{}", icon_folder(), name.to_lowercase())
}

fn svg(name: &str) -> String {
    format!("{}/{}.svg#icon", icon_folder(), name.to_lowercase())
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Icon::Url(url) => write!(f, "{}", url),
            Icon::IconRootCustom(name) => write!(f, "{}", custom(name)),
            Icon::IconRootSVG(name) => write!(f, "{}", svg(name)),
            Icon::Css(name) => write!(f, "{}", name),
        }
    }
}

impl From<Icon> for String {
    fn from(icon: Icon) -> Self {
        icon.to_string()
    }
}

pub fn update_theme() -> Result<()> {
    let icon_root = icon_root();
    let icon_folder = icon_folder();

    let icons = document().get_elements_by_class_name("icon");

    for idx in 0..icons.length() {
        let el = icons.item(idx).expect("Unabel to access icon element");
        let src = el.get_attribute("src");
        if let Some(src) = src {
            if src.starts_with(&icon_root) {
                let src = &src[icon_root.len() + 1..src.len()];
                let idx = src.find('/').expect("Unable to locate theme path ending");
                let src = format!("{}/{}", icon_folder, &src[idx + 1..]);
                el.set_attribute("src", &src)?;
            }
        }
    }

    Ok(())
}
