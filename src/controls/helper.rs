use web_sys::Element;
use workflow_ux::result::Result;

/*
pub trait FieldHelpers{

    fn set_value_attr(el: &Element, value: &str) -> Result<String>{
        Ok(FieldHelper::set_value_attr(el, value)?)
    }
}
*/

pub struct FieldHelper {}

impl FieldHelper {
    pub async fn get_categories() -> Result<Vec<(String, String)>> {
        // let mut list = Vec::new();
        // list.push(("Category 1".to_string(), "cat-1".to_string()));
        // list.push(("Category 2".to_string(), "cat-2".to_string()));
        // list.push(("Category 3".to_string(), "cat-3".to_string()));
        Ok(vec![
            ("Category 1".to_string(), "cat-1".to_string()),
            ("Category 2".to_string(), "cat-2".to_string()),
            ("Category 3".to_string(), "cat-3".to_string()),
        ])
    }
    pub async fn get_subcategories<T: Into<String>>(parent: T) -> Result<Vec<(String, String)>> {
        let mut list = Vec::new();
        let p: String = parent.into();
        if p.eq("cat-1") {
            list.push(("Category 1 - A".to_string(), "cat-1-a".to_string()));
            list.push(("Category 1 - B".to_string(), "cat-1-b".to_string()));
        } else if p.eq("cat-2") {
            list.push(("Category 2 - A".to_string(), "cat-2-a".to_string()));
            list.push(("Category 2 - B".to_string(), "cat-2-b".to_string()));
        }
        Ok(list)
    }

    pub fn set_value_attr(el: &Element, value: &str) -> Result<String> {
        Self::set_attr(el, "value", value)
    }

    pub fn set_attr(el: &Element, name: &str, value: &str) -> Result<String> {
        let v = value.replace('\"', "&quot;");
        el.set_attribute(name, &v)?;
        Ok(v)
    }
    pub fn clean_value_for_attr(value: &str) -> Result<String> {
        Ok(value.replace(['\"','\''], "&quot;"))
    }
}
