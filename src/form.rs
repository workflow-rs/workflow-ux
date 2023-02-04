use borsh::{
    de::BorshDeserialize as BorshDeserializeTrait, ser::BorshSerialize as BorshSerializeTrait,
    BorshDeserialize, BorshSerialize,
};
use downcast::{downcast_sync, AnySync};
use paste::paste;
//use workflow_log::log_trace;
use crate::{
    async_trait_without_send,
    attributes::Attributes,
    docs::Docs,
    error,
    form_footer::FormFooter,
    layout::{ElementLayout, ElementLayoutStyle, Elemental},
    prelude::{i18n, CallbackFn},
    result::Result,
    error::Error,
    //view::Layout,
    //document,
    //module::ModuleInterface
};
use std::{
    collections::BTreeMap,
    str,
    sync::{Arc, Mutex},
};
use web_sys::Element;

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

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
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

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
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

    pub fn add_object(&mut self, name: &str, obj: impl BorshSerializeTrait) -> Result<()> {
        let mut data = Vec::new();
        obj.serialize(&mut data)?;
        self.values
            .insert(name.to_string(), FormDataValue::Object(data));
        Ok(())
    }

    pub fn get_object<D: BorshDeserializeTrait>(&self, name: &str) -> Result<Option<D>> {
        if let Some(FormDataValue::Object(list)) = self.values.get(name) {
            let data = &mut &list.clone()[0..];
            let obj = D::deserialize(data)?;
            return Ok(Some(obj));
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

#[async_trait_without_send]
pub trait FormStage: Elemental + AnySync {
    async fn serialize(&self) -> Result<FormData>;
    async fn activate(&self) -> Result<()>;
    async fn deactivate(&self) -> Result<()>;
}

downcast_sync!(dyn FormStage);

#[derive(Clone)]
pub struct FormStages {
    layout: ElementLayout,
    index: Arc<Mutex<u8>>,
    pub stages: Arc<Mutex<Vec<Arc<dyn FormStage>>>>,
    pub data: Arc<Mutex<FormData>>,
    pub title: Arc<Mutex<String>>,
    error_cb: Arc<Mutex<Option<CallbackFn<Error>>>>,
}

unsafe impl Send for FormStages {}
unsafe impl Sync for FormStages {}

impl FormStages {
    pub fn new(
        parent_layout: &ElementLayout,
        attributes: &Attributes,
        _docs: &Docs,
    ) -> Result<Self> {
        let layout_style = ElementLayoutStyle::Form;
        let layout = ElementLayout::new(parent_layout, layout_style, attributes)?;
        let title = attributes
            .get("title")
            .unwrap_or(&"Step [INDEX]/[STEPS]".to_string())
            .clone();
        let layout = FormStages {
            layout,
            title: Arc::new(Mutex::new(title)),
            index: Arc::new(Mutex::new(0)),
            stages: Arc::new(Mutex::new(Vec::new())),
            data: Arc::new(Mutex::new(FormData::new(None))),
            error_cb: Arc::new(Mutex::new(None)),
        };

        Ok(layout)
    }

    pub fn add_stage(&self, stage: Arc<dyn FormStage>) -> Result<()> {
        self.stages.lock()?.push(stage);
        Ok(())
    }

    pub fn show(&self, show: bool) -> Result<()> {
        let el = self.layout.element();
        if show {
            Ok(el.remove_attribute("hidden")?)
        } else {
            Ok(el.set_attribute("hidden", "true")?)
        }
    }

    pub fn load(&self, _data: FormData) -> Result<()> {
        Ok(())
    }

    pub fn layout(&self) -> ElementLayout {
        self.layout.clone()
    }

    pub fn is_finished(&self) -> Result<bool> {
        let index = (self.index()? + 1) as usize;
        let length = self.len()?;
        Ok(index >= length)
    }

    pub fn is_first(&self) -> Result<bool> {
        Ok(self.index()? == 0)
    }

    pub fn is_last(&self) -> Result<bool> {
        let index = (self.index()? + 1) as usize;
        Ok(self.stages()?.len() == index)
    }

    pub fn len(&self) -> Result<usize> {
        Ok(self.stages()?.len())
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.stages()?.is_empty())
    }

    pub fn stages(&self) -> Result<Vec<Arc<dyn FormStage>>> {
        Ok(self.stages.lock()?.clone())
    }

    pub fn data(&self) -> Result<FormData> {
        Ok(self.data.lock()?.clone())
    }

    pub fn index(&self) -> Result<u8> {
        Ok(*self.index.lock()?)
    }
    pub fn title(&self) -> Result<String> {
        Ok(self.title.lock()?.clone())
    }

    pub async fn serialize_stage(&self) -> Result<FormData> {
        let stage = self.stage()?;
        let data = stage.serialize().await?;
        self.set_stage_data(self.index()?, data.clone())?;
        Ok(data)
    }

    fn set_stage_data(&self, index: u8, data: FormData) -> Result<()> {
        let mut complete_data = self.data.lock()?;
        complete_data.add_object(&format!("stage_{index}"), data)?;
        Ok(())
    }

    pub fn stage(&self) -> Result<Arc<dyn FormStage>> {
        let index = self.index()?;

        if let Some(stage) = self.stages.lock()?.get(index as usize) {
            Ok(stage.clone())
        } else {
            Err(error!("Invalid stage index"))
        }
    }

    pub fn stage_downcast_arc<T>(&self) -> Result<Arc<T>>
    where
        T: AnySync,
    {
        let stage = self.stage()?;
        let stage = stage.downcast_arc::<T>()?;

        Ok(stage)
    }

    pub async fn activate_stage(&self, index: u8, footer: Option<&FormFooter>) -> Result<()> {
        self.set_index(index, footer).await?;
        Ok(())
    }

    pub async fn next(&self, footer: Option<&FormFooter>) -> Result<bool> {
        if self.is_finished()? {
            return Ok(false);
        }

        self.set_index(self.index()? + 1, footer).await?;

        Ok(true)
    }

    async fn set_index(&self, mut index: u8, footer: Option<&FormFooter>) -> Result<()> {
        let stages = self.stages()?;
        for (i, stage) in stages.iter().enumerate() {
            if i == index as usize {
                self.element().append_child(&stage.element())?;
                stage.activate().await?;
            } else {
                stage.deactivate().await?;
            }
        }

        if let Some(footer) = footer {
            let last = stages.len() - 1;
            if (index as usize) < last {
                footer.set_submit_btn_text(i18n("Next"))?;
            } else {
                footer.set_submit_btn_text(i18n("Submit"))?;
                index = last as u8;
            }
        }

        *self.index.lock()? = index;
        self.update_title()?;

        Ok(())
    }

    pub fn update_title(&self) -> Result<()> {
        self.render_title(self.title()?)?;
        Ok(())
    }

    pub fn render_title<T: AsRef<str>>(&self, title: T) -> Result<()> {
        if let Some(el) = self.layout.element().query_selector(".layout-title")? {
            let title = title
                .as_ref()
                .replace("[INDEX]", &format!("{}", self.index()? + 1))
                .replace("[STEPS]", &format!("{}", self.len()?));
            el.set_inner_html(&title)
        }
        Ok(())
    }

    pub fn on_error(&self, callback: CallbackFn<Error>) {
        *self.error_cb.lock().unwrap() = Some(callback);
    }

    pub fn show_error(&self, error: Error)-> Result<()> {
        if let Some(cb) = self.error_cb.lock()?.as_mut() {
            cb(error)?;
        }

        Ok(())
    }
}

impl Elemental for FormStages {
    fn element(&self) -> web_sys::Element {
        self.layout.element()
    }
}

impl From<FormStages> for Element {
    fn from(form: FormStages) -> Element {
        form.layout.element()
    }
}
