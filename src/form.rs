use std::collections::BTreeMap;

use workflow_ux::result::Result;
use crate::async_trait::async_trait;

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

#[derive(Debug)]
pub enum FormDataValue{
    String(String),
    //Pubkey(String),
    //Usize(usize)
    List(Vec<String>)
}

#[derive(Debug)]
pub struct FormData{
    pub id: Option<String>,
    pub values:BTreeMap<String, FormDataValue>
}

impl FormData{
    pub fn new(id:Option<String>)->Self{
        Self { id, values: BTreeMap::new() }
    }

    pub fn id(&self)->Option<String>{
        self.id.clone()
    }
    pub fn with_id(&mut self, id:Option<String>){
        self.id = id;
    }

    pub fn insert(&mut self, name:&str, value:FormDataValue){
        self.values.insert(name.to_string(), value);
    }
    pub fn insert_string(&mut self, name:&str, value:String){
        self.values.insert(name.to_string(), FormDataValue::String(value));
    }
    pub fn insert_list(&mut self, name:&str, list:Vec<String>){
        self.values.insert(name.to_string(), FormDataValue::List(list));
    }

    pub fn get_string(&self, name:&str)->Option<String>{
        if let Some(value) = self.values.get(name){
            match value{
                FormDataValue::String(s)=>{
                    return Some(s.clone());
                },
                _=>{
                    return None;
                }
            }
        }

        None
    }
    pub fn empty()->Self{
        Self {
            id: None,
            values:BTreeMap::new()
        }
    }
}

#[workflow_async_trait]
pub trait FormHandler{
    async fn load(&self)->Result<()>;
    async fn submit(&self)->Result<()>;
}
