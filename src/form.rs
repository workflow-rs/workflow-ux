use workflow_ux::result::Result;
use async_trait::async_trait;

pub struct Category{
    pub key:String,
    pub text:String
}

impl Category{
    pub fn new<T: Into<String>>(text:T, key:T)->Self{
        Category{text:text.into(), key:key.into()}
    }
}

impl<T> From<(T, T)> for Category
where T:Into<String>{
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

pub struct FormResult{

}

impl FormResult{
    pub fn empty()->Self{
        FormResult{}
    }
}

#[async_trait(?Send)]
pub trait FormHandlers{
    async fn load(&mut self)->Result<()>;
    async fn submit(&mut self)->Result<FormResult>;
}
