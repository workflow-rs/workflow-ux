use crate::async_trait_without_send;
use crate::result::Result;
use borsh::de::BorshDeserialize;
use borsh::ser::BorshSerialize;
use paste::paste;
use std::{
    collections::BTreeMap,
    sync::Arc,
    str
};
use web_sys::Element;
use crate::{
    layout::{DefaultFunctions, Elemental, ElementLayout, ElementLayoutStyle},
    attributes::Attributes,
    docs::Docs,
    form_footer::FormFooter,
    view::Layout,
    document,
    module::ModuleInterface
};

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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


pub struct FormStages {
    layout: ElementLayout,
    footer: workflow_ux::form_footer::FormFooter,
    index: u8,
    pub stages: Vec<Arc<dyn FormHandler>>,
    pub data: Vec<FormData>
}

unsafe impl Send for FormStages {}
unsafe impl Sync for FormStages {}

impl FormStages {

    pub async fn try_create_layout_view(
        module: Option<std::sync::Arc<dyn ModuleInterface>>,
    ) -> Result<std::sync::Arc<Layout<Arc<Self>, ()>>> {
        let result = Self::try_create_layout_view_with_data(module, Option::<()>::None).await?;
        Ok(result)
    }

    pub async fn try_create_layout_view_with_data<D: Send + 'static>(
        module: Option<std::sync::Arc<dyn ModuleInterface>>,
        data: Option<D>,
    ) -> Result<std::sync::Arc<Layout<Arc<Self>, D>>> {
        let el = document().create_element("div")?;
        let layout = Arc::new(Self::try_inject(&el)?);
        layout.bind_footer()?;
        //layout.load().await?;
        let view = Layout::try_new(module, layout, data)?;
        
        /*{
            let layout_clone = view.layout();
            let mut locked = layout_clone.lock().expect("Unable to lock FormStages for footer binding.");
            //locked.footer.bind_form_stages_layout(view.clone())?;
            locked.footer.on_submit_click(Box::new(|_value|{

                Ok(())
            }));
        }
        */
        Ok(view)
    }

    pub fn try_new() -> Result<Self> {
        let el = workflow_ux::document().create_element("div")?;
        let layout = Self::try_inject(&el)?;
        Ok(layout)
    }

    pub fn try_inject(parent: &web_sys::Element) -> Result<Self> {
        let root =
            ElementLayout::try_inject(parent, ElementLayoutStyle::Form)?;
        let attributes = Attributes::new();
        let docs = Docs::new();
        let l = Self::new(&root, &attributes, &docs)?;
        Ok(l)
    }

    pub fn new(
        parent_layout: &ElementLayout,
        attributes: &Attributes,
        _docs: &Docs,
    ) -> Result<Self> {
        let attr_list = vec![("title".to_string(), "Form Stages".to_string())];

        let mut attributes = attributes.clone();
        for (k, v) in attr_list.iter() {
            attributes.insert(k.to_string(), v.clone());
        }

        let layout_style = ElementLayoutStyle::Form;
        let layout = ElementLayout::new(parent_layout, layout_style, &attributes)?;

        /*
        let stage1 = {
            let mut ctl_attributes = Attributes::new();
            let ctl_attr_list: Vec<(String, String)> = vec![];
            for (k, v) in ctl_attr_list.iter() {
                ctl_attributes.insert(k.to_string(), v.clone());
            }
            let mut layout_attributes = Attributes::new();
            let layout_attr_list: Vec<(String, String)> = vec![];
            for (k, v) in layout_attr_list.iter() {
                layout_attributes.insert(k.to_string(), v.clone());
            }
            let docs: Vec<&str> = vec![];
            let stage1 = FormStage1::new(&layout, &ctl_attributes, &docs)?;
            let child = stage1.element();
            layout.append_child(&child, &layout_attributes, &docs)?;
            stage1
        };
        */

        let footer = {
            let layout_attributes = Attributes::new();
            let ctl_attributes = Attributes::new();
            let docs: Vec<&str> = vec![];
            let footer = FormFooter::new(&layout, &ctl_attributes, &docs)?;
            let child = footer.element();
            layout.append_child(&child, &layout_attributes, &docs)?;
            footer
        };

        let layout = FormStages {
            layout,
            footer,
            index: 0,
            stages: Vec::new(),
            data: Vec::new()
        };

        layout.init()?;

        Ok(layout)
    }

    pub fn bind_footer(self: &Arc<Self>) -> Result<()>{
        self.footer.on_submit_click(Box::new(|_value|{
            //self.activate_stage(0)?;
            Ok(())
        }))?;
    
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

    pub fn layout(&self) -> ElementLayout {
        self.layout.clone()
    }

    pub fn set_submit_btn_text<T: Into<String>>(&self, text: T) -> Result<()> {
        self.footer.set_submit_btn_text(text)?;
        Ok(())
    }

    pub fn activate_stage(&self, index: u8)->Result<()>{
        self.set_stage_index(index)?;
        Ok(())
    }

    fn set_stage_index(&self, _index: u8)->Result<()>{
        /*
        match index{
            1=>{
                self.stage1.show(true)?;
                self.stage2.show(false)?;
                self.stage3.show(false)?;
                self.set_submit_btn_text(i18n("Next"))?;
                self.stage_index.set_value(1)?;
            }
            2=>{
                self.stage1.show(false)?;
                self.stage2.show(true)?;
                self.stage3.show(false)?;
                self.set_submit_btn_text(i18n("Next"))?;
                self.stage_index.set_value(2)?;
            }
            _=>{
                self.stage1.show(false)?;
                self.stage2.show(false)?;
                self.stage3.show(true)?;
                self.set_submit_btn_text(i18n("Submit"))?;
                self.stage_index.set_value(3)?;
            }
        }
        */
        
        Ok(())
    }

}

impl DefaultFunctions for FormStages {}

impl Elemental for Arc<FormStages> {
    fn element(&self) -> web_sys::Element {
        self.layout.element()
    }
}

impl Clone for FormStages {
    fn clone(&self) -> FormStages {
        FormStages {
            layout: self.layout.clone(),
            index: self.index,
            footer: self.footer.clone(),
            stages: self.stages.clone(),
            data: self.data.clone()
        }
    }
}

impl From<FormStages> for Element {
    fn from(form: FormStages) -> Element {
        form.layout.element()
    }
}

/*

#[async_trait_without_send]
impl FormHandler for FormStages {
    async fn load(&self) -> Result<()> {
        self.activate_stage(0)?;
        Ok(())
    }

    async fn submit(&self) -> Result<()> {
        Ok(())
    }
}
*/
