use crate::async_trait_without_send;
use crate::result::Result;
use borsh::de::BorshDeserialize;
use borsh::ser::BorshSerialize;
use paste::paste;
use std::collections::BTreeMap;
use std::str;

pub struct Category {
    pub key: String,
    pub text: String,
}

impl Category {
    pub fn new<T: Into<String>>(text: T, key: T) -> Self {
        Category {
            text: text.into(),
            key: key.into(),
        }
    }
}

impl<T> From<(T, T)> for Category
where
    T: Into<String>,
{
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

#[derive(Debug)]
pub enum FormDataValue {
    String(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),

    //Pubkey(String),
    //Usize(usize)
    List(Vec<String>),
    Object(Vec<u8>),
}

macro_rules! define_fields {
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
pub struct FormData {
    pub id: Option<String>,
    pub values: BTreeMap<String, FormDataValue>,
}

impl FormData {
    pub fn new(id: Option<String>) -> Self {
        Self {
            id,
            values: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }
    pub fn with_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    pub fn add(&mut self, name: &str, value: FormDataValue) {
        self.values.insert(name.to_string(), value);
    }

    pub fn add_list(&mut self, name: &str, list: Vec<String>) {
        self.values
            .insert(name.to_string(), FormDataValue::List(list));
    }

    pub fn add_object(&mut self, name: &str, obj: impl BorshSerialize) -> Result<()> {
        let mut data = Vec::new();
        obj.serialize(&mut data)?;
        self.values
            .insert(name.to_string(), FormDataValue::Object(data));
        Ok(())
    }

    pub fn get_object<D: BorshDeserialize>(&self, name: &str) -> Result<Option<D>> {
        if let Some(value) = self.values.get(name) {
            match value {
                FormDataValue::Object(list) => {
                    let data = &mut &list.clone()[0..];
                    let obj = D::deserialize(data)?;
                    return Ok(Some(obj));
                }
                _ => {}
            }
        }

        Ok(None)
    }
    pub fn add_string(&mut self, name: &str, value: String) {
        self.values
            .insert(name.to_string(), FormDataValue::String(value));
    }
    pub fn get_string(&self, name: &str) -> Option<String> {
        if let Some(value) = self.values.get(name) {
            match value {
                FormDataValue::String(value) => {
                    return Some(value.clone());
                }
                _ => {
                    return None;
                }
            }
        }
        None
    }

    define_fields!(U8 U16 U32 U64 U128 F32 F64 Bool);

    pub fn empty() -> Self {
        Self {
            id: None,
            values: BTreeMap::new(),
        }
    }
}

#[async_trait_without_send]
pub trait FormHandler {
    async fn load(&self) -> Result<()>;
    async fn submit(&self) -> Result<()>;
}
