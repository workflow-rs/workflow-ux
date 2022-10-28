use crate::prelude::*;
use crate::result::Result;

#[derive(Clone)]
pub struct HiddenId {
    el:Element,
    value: Arc<Mutex<Option<String>>>
}

unsafe impl Send for HiddenId{}

impl HiddenId {

    pub fn element(&self)->Element{
        self.el.clone()
    }

    pub fn new(_pane : &ElementLayout, _attributes: &Attributes, _docs : &Docs) -> Result<Self> {
        let el = document().create_element("input")?;
        el.set_attribute("type", "hidden")?;

        Ok(Self{
            el,
            value:Arc::new(Mutex::new(None))
        })
    }

    pub fn value(&self) -> Result<Option<String>> {
        let value = self.value.lock()?.clone();
        Ok(value)
    }

    pub fn set_value(&self, value:Option<String>) -> Result<()> {
        *self.value.lock()? = value;
        Ok(())
    }

}

