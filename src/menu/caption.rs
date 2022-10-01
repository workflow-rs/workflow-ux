#[derive(Debug, Clone)]
pub struct MenuCaption {
    pub title: String,
    pub short: String,
    pub subtitle: String,
}

impl From<Vec<&str>> for MenuCaption {
    fn from(v: Vec<&str>) -> Self {
        let title = { if v.len() > 0 { v[0] } else { "" } }.to_string();
        let short = { if v.len() > 1 { v[1] } else { "" } }.to_string();
        let subtitle = { if v.len() > 2 { v[2] } else { "" } }.to_string();
        Self { title, short, subtitle }
    }
}

impl From<(&str,&str,&str)> for MenuCaption {
    fn from((title,short,subtitle): (&str,&str,&str)) -> Self {
        Self { 
            title : title.to_string(),
            short : short.to_string(),
            subtitle : subtitle.to_string(), 
        }
    }
}

impl From<(&str,&str)> for MenuCaption {
    fn from((title,short): (&str,&str)) -> Self {
        (title,short,"").into()
    }
}

impl From<&str> for MenuCaption {
    fn from(title: &str) -> Self {
        (title,title,"").into()
    }
}

impl From<String> for MenuCaption {
    fn from(title: String) -> Self {
        (title.as_str(),title.as_str(),"").into()
    }
}
impl From<(String, String)> for MenuCaption {
    fn from(t: (String, String)) -> Self {
        Self {
            title: t.0,
            subtitle: t.1,
            short: "".to_string()
        }
    }
}
impl From<(String, String, String)> for MenuCaption {
    fn from(t: (String, String, String)) -> Self {
        Self {
            title: t.0,
            subtitle: t.1,
            short: t.2
        }
    }
}