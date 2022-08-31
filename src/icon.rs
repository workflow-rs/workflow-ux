use workflow_ux::result::Result;
use workflow_ux::document;
use workflow_ux::theme::current_theme_folder;

static mut ICON_ROOT_URL : Option<String> = None;

pub fn icon_root() -> String {
    unsafe { (&ICON_ROOT_URL).as_ref().expect("Icon root is not initialized").clone() }
}

// #[wasm_bindgen] 
pub fn init_icon_root(icon_root: &str) -> Result<()> {
    let icon_root = icon_root.to_string();
    if icon_root.ends_with("/") {
        icon_root.to_string().pop();//.push('/');
    }
    unsafe { ICON_ROOT_URL = Some(icon_root.to_string()); }
    Ok(())
}

pub fn icon_folder() -> String {
    format!("{}/{}",icon_root(),current_theme_folder()).to_string()
}

pub enum Icon {
    Url(String),
    IconRootCustom(String),
    IconRootSVG(String),
}

impl Icon{
    pub fn svg<T:Into<String>>(icon:T)->Icon{
        Self::IconRootSVG(icon.into())
    }
    pub fn url<T:Into<String>>(icon:T)->Icon{
        Self::Url(icon.into())
    }
    pub fn custom<T:Into<String>>(icon:T)->Icon{
        Self::IconRootCustom(icon.into())
    }
}

fn custom(name:&str) -> String {
    format!("{}/{}",icon_folder(),name).to_string()
}

fn svg(name:&str) -> String {
    format!("{}/{}.svg",icon_folder(),name).to_string()
}

impl ToString for Icon {
    fn to_string(&self) -> String {
        match self {
            Icon::Url(url) => url.clone(),
            Icon::IconRootCustom(name) => custom(&name),
            Icon::IconRootSVG(name) => svg(&name),
        }
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
                let src = &src[icon_root.len()+1..src.len()];
                let idx = src.find("/").expect("Unable to locate theme path ending");
                let src = format!("{}/{}",icon_folder,&src[idx+1..]);
                el.set_attribute("src", &src)?;
            }
        }
    }

    Ok(())
}