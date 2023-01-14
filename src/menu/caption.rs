#[derive(Debug, Clone)]
pub struct MenuCaption {
    pub title: String,
    pub subtitle: String,
    pub tooltip: String,
}
/*
impl MenuCaption{

    pub fn get_tooltip(&self)->String{
        if self.tooltip.len() > 0{
            self.tooltip.clone()
        }else{
            self.title.clone()
        }
    }

}
*/

impl From<Vec<&str>> for MenuCaption {
    fn from(v: Vec<&str>) -> Self {
        let title = {
            if v.len() > 0 {
                v[0]
            } else {
                ""
            }
        }
        .to_string();
        let mut subtitle = "".to_string();
        let mut tooltip = "".to_string();

        if v.len() > 2 {
            subtitle = v[1].to_string();
            tooltip = v[2].to_string();
        } else if v.len() > 1 {
            subtitle = v[1].to_string();
        }

        Self {
            title,
            subtitle,
            tooltip,
        }
    }
}

impl From<(&str, &str, &str)> for MenuCaption {
    fn from((title, subtitle, tooltip): (&str, &str, &str)) -> Self {
        Self {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            tooltip: tooltip.to_string(),
        }
    }
}

impl From<(&str, &str)> for MenuCaption {
    fn from((title, subtitle): (&str, &str)) -> Self {
        (title, subtitle, "").into()
    }
}

impl From<&str> for MenuCaption {
    fn from(title: &str) -> Self {
        (title, title, "").into()
    }
}

impl From<String> for MenuCaption {
    fn from(title: String) -> Self {
        (title.as_str(), title.as_str(), "").into()
    }
}
impl From<(String, String)> for MenuCaption {
    fn from(t: (String, String)) -> Self {
        Self {
            title: t.0,
            subtitle: t.1,
            tooltip: "".to_string(),
        }
    }
}
impl From<(String, String, String)> for MenuCaption {
    fn from(t: (String, String, String)) -> Self {
        Self {
            title: t.0,
            subtitle: t.1,
            tooltip: t.2,
        }
    }
}
