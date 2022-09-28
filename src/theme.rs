// use wasm_bindgen::prelude::*;
use workflow_ux::document;
use workflow_ux::result::Result;
use workflow_ux::error::Error;
use workflow_core::enums::*;
// use workflow_core_macros::*;//describe_enum;
// use workflow_macros::describe_enum;
use convert_case::{Case, Casing};

#[describe_enum]
#[derive(Debug, Clone)]
pub enum Theme {
    Light,
    Dark,
    // DarkBlue,

}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

impl Theme {
    fn to_class_name(&self) -> String {
        format!("flow-theme-{}", 
            self.as_str()
                .from_case(Case::Camel)
                .to_case(Case::Kebab)
        ).to_string()
    }
    fn to_folder_name(&self) -> String {
        self.as_str()
            .from_case(Case::Camel)
            .to_case(Case::Kebab)
    }
}

static mut CURRENT_THEME : Option<Theme> = None;

pub fn set_logo(_logo : &str) -> Result<()> {
    // TODO set application logo image 
    Ok(())
}

pub fn init_theme(theme: Theme) -> Result<()> {
    Ok(set_theme(theme)?)
}
fn build_theme_file_path(theme: &Theme)->String{
    format!("/resources/themes/{}.css", 
            theme.as_str()
                .from_case(Case::Camel)
                .to_case(Case::Kebab)
        ).to_string()
}

pub fn set_theme(theme: Theme) -> Result<()> {
    let doc = document();
    let el = match doc.body(){
        Some(el)=>el,
        None=>{
            return Err(Error::UnableToGetBody);
        }
    };

    let theme_link = match doc.query_selector("head link[app-theme]")?{
        Some(el)=>el,
        None=>{
            if let Some(head) = doc.query_selector("head")?{
                let link = doc.create_element("link")?;
                link.set_attribute("app-theme", &theme.to_folder_name())?;
                link.set_attribute("rel", "stylesheet")?;
                link.set_attribute("type", "text/css")?;
                head.append_child(&link)?;
                link
            }else{
                panic!("unable to get head element for theme");
            }
            
        }
    };
    let theme_file_path =  build_theme_file_path(&theme);
    if let Some(p) = theme_link.get_attribute("href"){
        if !p.eq(&theme_file_path){
            theme_link.set_attribute("href", &theme_file_path)?;
            theme_link.set_attribute("app-theme", &theme.to_folder_name())?;
        }
    }else{
        theme_link.set_attribute("href", &theme_file_path)?;
        theme_link.set_attribute("app-theme", &theme.to_folder_name())?;
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

fn update_current_theme(theme : Theme) -> Result<()> {
    unsafe { CURRENT_THEME = Some(theme); }
    // update_dom_elements();
    // TODO - iterate over dom, replace all themable elements
    workflow_ux::icon::update_theme()?;

    Ok(())
}

pub fn current_theme() -> Theme {
    unsafe { (&CURRENT_THEME).as_ref().expect("Application theme is not initialized").clone() }
}

pub fn current_theme_folder() -> String {
    let theme = current_theme(); //unsafe { (&CURRENT_THEME).as_ref().expect("Application theme is not initialized") };
    theme.to_folder_name()
}

