use crate::prelude::*;
use crate::result::Result;
use workflow_html::Html;

pub trait ItemBuilder{
    fn list()->Result<Html>;
}


#[derive(Clone)]
pub struct Builder<B> {
    pub layout : ElementLayout,
    pub element : Element,
    pub list_container: Element,
    b: PhantomData<B>
}

unsafe impl<B> Send for Builder<B> where B:ItemBuilder{}

impl<B> Builder<B> 
where B:ItemBuilder{
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn new(pane : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Self> {

        let element = document()
            .create_element("div")?;

        let list_container = document()
            .create_element("div")?;
        list_container.class_list().add_1("list-container")?;
        element.append_child(&list_container)?;
         
        //element.set_inner_html("<h1>builder</h1>");
        

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }

        let mut builder = Self { 
            layout : pane.clone(),
            element,
            list_container,
            b:PhantomData
        };

        builder = builder.init()?;

        Ok(builder)
    }

    fn init(self)->Result<Self>{
        let list = self.create_list()?;

        list.inject_into(&self.list_container)?;

        Ok(self)
    }

    pub fn create_list(&self)->Result<Html>{
        let tree = B::list()?;
        Ok(tree)
    }

}

