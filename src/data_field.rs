use crate::prelude::*;
use crate::result::Result;

#[derive(Clone)]
pub struct DataField<T: Clone> {
    data: Arc<Mutex<Option<T>>>,
}

impl<T: Clone> DataField<T> {
    pub fn new(_: &ElementLayout, _: &Attributes, _: &Docs) -> Result<Self> {
        Ok(Self {
            data: Arc::new(Mutex::new(None)),
        })
    }
    pub fn value(&self) -> Result<Option<T>> {
        Ok(self.data.lock()?.clone())
    }

    pub fn set_value(&self, data: Option<T>) -> Result<()> {
        *self.data.lock()? = data;
        Ok(())
    }
}
