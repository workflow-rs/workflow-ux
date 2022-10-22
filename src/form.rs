use std::collections::BTreeMap;
use crate::result::Result;
use crate::workflow_async_trait;
use paste::paste;
use std::str;

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
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    //Pubkey(String),
    //Usize(usize)
    List(Vec<String>)
}

macro_rules! num_fields {
    ($($ident:ident)+)=>{
        paste!{
            $(
            pub fn [<add_ $ident:lower>](&mut self, name:&str, value:[<$ident:lower>]){
                self.values.insert(name.to_string(), FormDataValue::$ident(value));
            }
            pub fn [<get_ $ident:lower>](&self, name:&str)->Option<[<$ident:lower>]>{
                if let Some(value) = self.values.get(name){
                    match value{
                        FormDataValue::$ident(value)=>{
                            return Some(value.clone());
                        },
                        _=>{
                            return None;
                        }
                    }
                }
        
                None
            })+
        }
    }
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

    pub fn add(&mut self, name:&str, value:FormDataValue){
        self.values.insert(name.to_string(), value);
    }
    pub fn add_string(&mut self, name:&str, value:String){
        self.values.insert(name.to_string(), FormDataValue::String(value));
    }
    pub fn add_list(&mut self, name:&str, list:Vec<String>){
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
    num_fields!(U8 U16 U32 U64 U128);
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
