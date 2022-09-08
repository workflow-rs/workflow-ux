use workflow_ux::result::Result;
use web_sys::Element;

/*
pub trait FieldHelpers{

    fn set_value_attr(el: &Element, value: &str) -> Result<String>{
        Ok(FieldHelper::set_value_attr(el, value)?)
    }
}
*/

pub struct FieldHelper{

}

impl FieldHelper{
    pub fn set_value_attr(el: &Element, value: &str) -> Result<String>{
        Ok(Self::set_attr(el, "value", value)?)
    }

    pub fn set_attr(el: &Element, name:&str, value: &str)-> Result<String>{
        let v = value.replace("\"", "&quot;");
        el.set_attribute(name, &v)?;
        Ok(v)
    }
}