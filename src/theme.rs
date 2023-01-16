// use wasm_bindgen::prelude::*;
use crate::controls::svg::SvgNode;
use crate::icon::{icon_root, IconInfoMap};
use crate::prelude::log_trace;
use crate::prelude::{Arc, Mutex, SvgElement};
use crate::style::ControlStyle;
use crate::{document, error::Error, result::Result};

use convert_case::{Case, Casing};
use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
pub struct CustomTheme {
    pub name: String,
    pub contents: ThemeContents,
}

#[derive(Debug, Clone)]
pub enum Theme {
    Light,
    Dark,
    Custom(CustomTheme),
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}
#[derive(Debug, Clone)]
pub struct ThemeContents {
    css: String,
    svg: String,
}

static mut DARK_THEME: Option<ThemeContents> = None;
static mut LIGHT_THEME: Option<ThemeContents> = None;

impl Theme {
    pub fn update_theme_content_icons(icons: Arc<Mutex<IconInfoMap>>) {
        unsafe { DARK_THEME = Some(build_theme_content("dark", icons.clone())) }
        unsafe { LIGHT_THEME = Some(build_theme_content("light", icons)) }

        match refresh_theme() {
            Ok(_) => {}
            Err(e) => {
                log_trace!("unable to flush theme content: {:?}", e);
            }
        }
    }
    fn name(&self) -> String {
        match self {
            Theme::Custom(theme) => theme.name.clone(),
            Theme::Dark => "dark".to_string(),
            Theme::Light => "light".to_string(),
        }
    }
    fn to_class_name(&self) -> String {
        format!("flow-theme-{}", self.to_folder_name())
    }
    fn to_folder_name(&self) -> String {
        self.name().from_case(Case::Camel).to_case(Case::Kebab)
    }

    fn content(&self) -> ThemeContents {
        match self {
            Theme::Custom(theme) => theme.contents.clone(),
            Theme::Dark => get_theme_content("dark"),
            Theme::Light => get_theme_content("light"),
        }
    }
}

static mut CURRENT_THEME: Option<Theme> = None;

pub fn set_logo(_logo: &str) -> Result<()> {
    // TODO set application logo image
    Ok(())
}

pub fn init_theme(theme: Theme) -> Result<()> {
    set_theme(theme)
}
/*
fn build_theme_file_path(theme: &Theme)->String{
    format!("/resources/themes/{}.css",
            theme.as_str()
                .from_case(Case::Camel)
                .to_case(Case::Kebab)
        ).to_string()
}
*/

fn build_theme_content(theme: &str, icons: Arc<Mutex<IconInfoMap>>) -> ThemeContents {
    let mut icons_list = Vec::new();
    let mut var_list = Vec::new();
    let mut svg_list = Vec::new();
    let locked = icons
        .lock()
        .expect("Unable to lock icons for builing theme contents");
    let root = icon_root();
    for (id, icon) in locked.iter() {
        var_list.push(format!(
            "--svg-icon-{}:url(\"{}/{}/{}\");",
            id, root, theme, icon.file_name
        ));
        icons_list.push(format!(
            ".icon[icon=\"{}\"]{{background-image:var(--svg-icon-{})}}",
            id, id
        ));
        svg_list.push(format!(
            "<symbol id=\"svg-icon-{}\" viewBox=\"0 0 50 50\"><image href=\"{}/{}/{}\"></image></symbol>",
            id, root, theme, icon.file_name
        ));
    }

    ThemeContents {
        css: format!(
            "body{{\n/****** variables ******/\n{}}}\n\n/****** icons ******/\n{}",
            var_list.join(""),
            icons_list.join("")
        ),
        svg: svg_list.join(""),
    }
}

fn get_theme_content(theme: &str) -> ThemeContents {
    let theme_opt = match theme {
        "dark" => unsafe { DARK_THEME.as_ref() },
        "light" => unsafe { LIGHT_THEME.as_ref() },
        _ => None,
    };

    match theme_opt {
        Some(content) => content.clone(),
        None => ThemeContents {
            css: "".to_string(),
            svg: "".to_string(),
        },
    }
}

pub fn set_theme(theme: Theme) -> Result<()> {
    let doc = document();
    let el = match doc.body() {
        Some(el) => el,
        None => {
            return Err(Error::UnableToGetBody);
        }
    };

    let theme_el = match doc.query_selector("head style[app-theme]")? {
        Some(el) => el,
        None => {
            if let Some(head) = doc.query_selector("head")? {
                let el = doc.create_element("style")?;
                //el.set_attribute("app-theme", &theme.to_folder_name())?;
                //el.set_attribute("rel", "stylesheet")?;
                el.set_attribute("type", "text/css")?;
                head.append_child(&el)?;
                el
            } else {
                panic!("unable to get head element for theme");
            }
        }
    };
    let theme_svg_el = match doc.query_selector("body svg[app-theme]")? {
        Some(el) => el.dyn_into::<SvgElement>()?,
        None => {
            if let Some(body) = doc.query_selector("body")? {
                let svg = SvgElement::try_new("svg")?;
                svg.set_attribute("display", "none")?;
                body.append_child(&svg)?;
                svg
            } else {
                panic!("unable to get body element for theme svg");
            }
        }
    };

    let content = theme.content();
    let name = format!(
        "{}-{}-{}",
        theme.to_folder_name(),
        content.css.len(),
        content.svg.len()
    );
    let mut update = true;
    if let Some(p) = theme_el.get_attribute("app-theme") {
        if p.eq(&name) {
            update = false;
        }
    }
    if update {
        let sep = "\n/**************************************************/\n";
        let msg1 = format!("{sep}/*{: ^48}*/{sep}", "WorkflowUX : Controls");
        let msg2 = format!("{sep}/*{: ^48}*/{sep}", "WorkflowUX : Theme");
        theme_el.set_inner_html(&format!(
            "{msg1}{}\n\n{msg2}{}",
            ControlStyle::get_str(),
            content.css
        ));
        theme_svg_el.set_inner_html(&content.svg);
        theme_el.set_attribute("app-theme", &name)?;
        theme_svg_el.set_attribute("app-theme", &name)?;
    }

    let list = el.class_list();
    for idx in 0..list.length() {
        let cls = list.item(idx).unwrap();
        if cls.starts_with("flow-theme") {
            list.replace(&cls, &theme.to_class_name())?;
            update_current_theme(theme)?;
            return Ok(());
        }
    }

    // theme element not found, inject
    list.add_1(&theme.to_class_name())?;
    update_current_theme(theme)?;

    Ok(())
}

fn update_current_theme(theme: Theme) -> Result<()> {
    unsafe {
        CURRENT_THEME = Some(theme);
    }
    // update_dom_elements();
    // TODO - iterate over dom, replace all themable elements
    workflow_ux::icon::update_theme()?;

    Ok(())
}

fn refresh_theme() -> Result<()> {
    set_theme(current_theme())?;
    Ok(())
}

pub fn current_theme() -> Theme {
    unsafe {
        CURRENT_THEME
            .as_ref()
            .expect("Application theme is not initialized")
            .clone()
    }
}

pub fn current_theme_folder() -> String {
    let theme = current_theme(); //unsafe { (&CURRENT_THEME).as_ref().expect("Application theme is not initialized") };
    theme.to_folder_name()
}
