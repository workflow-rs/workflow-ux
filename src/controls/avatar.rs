use std::collections::BTreeMap;
use std::sync::{LockResult, MutexGuard};

use crate::controls::prelude::*;
use crate::image::Image;
use crate::prelude::*;
use crate::result::Result;
use crate::task::FunctionDebounce;
use hex;
use md5;
use sha2::{Digest, Sha256};
use workflow_core::enums::Describe;

use super::input::FlowInputBase;

#[derive(Clone, Debug)]
pub enum AvatarValue {
    Gravatar(Vec<u8>),
    Libravatar(Vec<u8>),
    Robohash(Vec<u8>),
    Url(String),
}

#[derive(Clone, Describe)]
pub enum AvatarProvider {
    Gravatar,
    Libravatar,
    Robohash,
    #[descr("Custom URL")]
    Custom,
}
pub struct AvatarInner {
    pub provider: AvatarProvider,
    pub params: BTreeMap<&'static str, String>,
    pub value: Option<AvatarValue>,
    pub attributes: Attributes,
    pub docs: Docs,
    pub changeable: bool,
    pub fallback: String,
    pub email_field_handler: Option<FunctionDebounce>,
    pub text_field_handler: Option<FunctionDebounce>,
    pub url_field_handler: Option<FunctionDebounce>,
}

#[derive(Clone)]
pub struct Avatar {
    pub element: Element,
    pub image: Image,
    form_container: Element,
    email_field: Input,
    text_field: Input,
    url_field: Input,
    md5_radio: Element,
    sha256_radio: Element,
    hash_containers: ElementWrapper,
    change_btn: ElementWrapper,
    cancel_btn: ElementWrapper,
    save_btn: ElementWrapper,
    provider_select: Select<AvatarProvider>,
    inner: Arc<Mutex<AvatarInner>>,
}

unsafe impl Send for Avatar {}

impl Avatar {
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn inner(&self) -> LockResult<MutexGuard<AvatarInner>> {
        self.inner.lock()
    }

    pub fn new(pane: &ElementLayout, attr: &Attributes, docs: &Docs) -> Result<Self> {
        let element = create_el("div", vec![("class", "avatar-container")], None)?;
        let img_box = create_el("div", vec![("class", "img-box")], None)?;
        element.append_child(&img_box)?;
        let form = create_el("div", vec![("class", "form-container")], None)?;
        element.append_child(&form)?;
        let action_container = create_el("div", vec![("class", "action-container")], None)?;
        element.append_child(&action_container)?;

        //image
        let mut fallback = "".to_string();
        if let Some(url) = attr.get("fallback") {
            fallback = url.clone();
        }
        let image = Image::new()?.with_src_and_fallback(&fallback, &fallback)?;
        let img_el = image.element();
        img_box.append_child(&img_el)?;

        //form
        let radio_name = format!("avatar-{}", Id::new());
        let mut provider_attr = attr.clone();
        provider_attr.insert("value".to_string(), "Gravatar".to_string());
        provider_attr.insert("label".to_string(), i18n("Provider"));
        let provider_select = Select::<AvatarProvider>::new(pane, &provider_attr, docs)?;
        form.append_child(&provider_select.element())?;

        let email_field =
            Self::create_input_field(pane, attr, docs, &form, "Enter Email address", "email")?;
        let text_field =
            Self::create_input_field(pane, attr, docs, &form, "Enter Robotext", "text")?;
        let url_field = Self::create_input_field(pane, attr, docs, &form, "Enter URL", "url")?;

        let hash_containers = create_el(
            "div",
            vec![("class", "hash-containers"), ("data-for", "hash")],
            None,
        )?;
        let md5_radio = Self::create_hash_field(&hash_containers, &radio_name, "md5")?;
        let sha256_radio = Self::create_hash_field(&hash_containers, &radio_name, "sha256")?;
        form.append_child(&hash_containers)?;

        //buttons
        let change_btn = create_el("flow-btn", vec![("class", "change")], Some(&i18n("Edit")))?;
        action_container.append_child(&change_btn)?;
        let cancel_btn = create_el("flow-btn", vec![("class", "cancel")], Some(&i18n("Cancel")))?;
        action_container.append_child(&cancel_btn)?;
        let save_btn = create_el("flow-btn", vec![("class", "save")], Some(&i18n("Save")))?;
        action_container.append_child(&save_btn)?;

        let control = Avatar {
            element,
            image,
            form_container: form,
            email_field,
            text_field,
            url_field,
            md5_radio,
            sha256_radio,
            hash_containers: ElementWrapper::new(hash_containers),
            provider_select,
            change_btn: ElementWrapper::new(change_btn),
            cancel_btn: ElementWrapper::new(cancel_btn),
            save_btn: ElementWrapper::new(save_btn),

            inner: Arc::new(Mutex::new(AvatarInner {
                attributes: attr.clone(),
                docs: docs.clone(),
                provider: AvatarProvider::Gravatar,
                params: BTreeMap::new(),
                value: None,
                changeable: true,
                fallback,
                email_field_handler: None,
                text_field_handler: None,
                url_field_handler: None,
            })),
        };

        let control = control.init()?;

        Ok(control)
    }

    fn create_input_field(
        pane: &ElementLayout,
        attr: &Attributes,
        docs: &Docs,
        parent: &Element,
        label: &str,
        input_type: &str,
    ) -> Result<Input> {
        let mut field_attr = attr.clone();
        field_attr.insert("label".to_string(), i18n(label));
        field_attr.insert("type".to_string(), input_type.to_string());
        let field = Input::new(pane, &field_attr, docs)?;
        parent.append_child(&field.element())?;

        Ok(field)
    }

    fn create_hash_field(parent: &Element, radio_name: &str, hash_type: &str) -> Result<Element> {
        let container = create_el(
            "div",
            vec![("class", "hash-container"), ("data-hash-type", hash_type)],
            None,
        )?;
        let radio = create_el(
            "flow-radio",
            vec![("name", radio_name), ("data-set-hash-type", hash_type)],
            None,
        )?;
        let field = create_el(
            "flow-input",
            vec![("readonly", "true"), ("label", &hash_type.to_uppercase())],
            None,
        )?;
        container.append_child(&radio)?;
        container.append_child(&field)?;
        parent.append_child(&container)?;

        Ok(radio)
    }

    fn init(mut self) -> Result<Self> {
        //self.set_value("Robohash|hello".to_string())?;
        let this = self.clone();
        self.provider_select
            .on_change(Box::new(move |provider| -> Result<()> {
                if let Some(provider) = AvatarProvider::from_str(&provider) {
                    this.set_provider(provider)?;
                }
                Ok(())
            }));

        let this = self.clone();
        self.change_btn.on_click(move |_| -> Result<()> {
            this.on_change_click()?;
            Ok(())
        })?;
        let this = self.clone();
        self.save_btn.on_click(move |_| -> Result<()> {
            this.on_save_click()?;
            Ok(())
        })?;
        let this = self.clone();
        self.cancel_btn.on_click(move |_| -> Result<()> {
            this.on_cancel_click()?;
            Ok(())
        })?;
        let this = self.clone();
        self.hash_containers.on_click(move |e| -> Result<()> {
            if let Some(et) = e.target() {
                let el = et.dyn_into::<Element>().unwrap_or_else(|_| panic!(
                    "Avatar: Could not cast EventTarget to Element: {:?}",
                    e
                ));
                if let Some(el) = el.closest("[data-set-hash-type]")? {
                    let hash_type = el.get_attribute("data-set-hash-type").unwrap();
                    this.on_hash_type_change(hash_type)?;
                }
            }
            Ok(())
        })?;

        let inner = self.inner.clone();
        self.email_field.on_change(Box::new(move |text| {
            inner
                .lock()?
                .email_field_handler
                .as_ref()
                .unwrap()
                .execute_with_str(text)?;
            Ok(())
        }));
        let inner = self.inner.clone();
        self.text_field.on_change(Box::new(move |text| {
            inner
                .lock()?
                .text_field_handler
                .as_ref()
                .unwrap()
                .execute_with_str(text)?;
            Ok(())
        }));
        let inner = self.inner.clone();
        self.url_field.on_change(Box::new(move |text| {
            inner
                .lock()?
                .url_field_handler
                .as_ref()
                .unwrap()
                .execute_with_str(text)?;
            Ok(())
        }));

        {
            let mut locked = self.inner()?;
            let this = self.clone();
            locked.email_field_handler = Some(FunctionDebounce::new_with_str(
                500,
                Box::new(move |email: String| {
                    //log_trace!("update_hashes: {:?}", email);
                    this.update_hashes(email)?;
                    Ok(())
                }),
            ));
            let this = self.clone();
            locked.text_field_handler = Some(FunctionDebounce::new_with_str(
                500,
                Box::new(move |text: String| {
                    //log_trace!("set_robotext: {:?}", text);
                    this.set_robotext(Self::build_sha256_hash(text)?, None)?;
                    Ok(())
                }),
            ));
            let this = self.clone();
            locked.url_field_handler = Some(FunctionDebounce::new_with_str(
                500,
                Box::new(move |url: String| {
                    //log_trace!("set_custom_url: {:?}", url);
                    this.set_custom_url(url)?;
                    Ok(())
                }),
            ));
        }

        self.show_save_btn(false)?;
        self.update_image()?;
        let changeable = { self.inner()?.changeable };
        self.show_change_btn(changeable)?;
        self.set_hash_type("md5")?;

        Ok(self)
    }

    fn set_hash_type(&self, hash_type: &str) -> Result<()> {
        //set default hash type
        if hash_type.eq("md5") {
            self.md5_radio.set_attribute("checked", "true")?;
            self.on_hash_type_change("md5".to_string())?;
        } else if hash_type.eq("sha256") {
            self.sha256_radio.set_attribute("checked", "true")?;
            self.on_hash_type_change("sha256".to_string())?;
        }

        Ok(())
    }

    pub fn set_editable(&self, editable: bool) -> Result<()> {
        {
            self.inner()?.changeable = editable;
        }
        self.show_change_btn(editable)?;
        Ok(())
    }

    fn on_change_click(&self) -> Result<()> {
        let updating = { self.inner()?.value.is_some() };
        self.open_form(updating)?;
        Ok(())
    }

    fn open_form(&self, updating: bool) -> Result<()> {
        if updating {
            self.save_btn.element.set_inner_html(&i18n("Update"));
        } else {
            self.save_btn.element.set_inner_html(&i18n("Save"));
        }
        self.show_save_btn(true)?;
        self.form_container.class_list().add_1("open")?;
        Ok(())
    }
    fn close_form(&self) -> Result<()> {
        self.show_save_btn(false)?;
        self.show_change_btn(self.inner()?.changeable)?;
        self.form_container.class_list().remove_1("open")?;
        Ok(())
    }
    fn on_save_click(&self) -> Result<()> {
        let valid = self.save()?;
        if valid {
            self.close_form()?;
        }
        Ok(())
    }
    fn set_inner_value(&self, value: Option<AvatarValue>) -> Result<()> {
        let is_some = value.is_some();
        self.inner()?.value = value;

        if is_some {
            self.change_btn.element.set_inner_html(&i18n("Change"));
        } else {
            self.change_btn.element.set_inner_html(&i18n("Set"));
        }
        Ok(())
    }
    fn save(&self) -> Result<bool> {
        if let Some(value) = self.serialize_value()? {
            log_trace!("[Avatar]: value: {:?}", value);
            self.set_inner_value(Some(value))?;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn value(&self) -> Result<Option<AvatarValue>> {
        Ok(self.inner()?.value.clone())
    }
    pub fn set_value(&self, value: Option<AvatarValue>) -> Result<()> {
        if let Some(clean) = self.deserialize_value(value)? {
            self.set_inner_value(Some(clean))?;
        }

        Ok(())
    }

    fn deserialize_value(&self, value: Option<AvatarValue>) -> Result<Option<AvatarValue>> {
        let set_provider = |provider: AvatarProvider| -> Result<()> {
            self.provider_select.set_value(provider.as_str())?;
            self.set_provider(provider)?;
            Ok(())
        };

        let set_hash = |hash: Vec<u8>| -> Result<()> {
            let text_value = hex::encode(hash);
            self.text_field.set_value("".to_string())?;
            self.set_hash_input_value("md5", "".to_string())?;
            self.set_hash_input_value("sha256", "".to_string())?;
            let hash_type = if text_value.len() == 32 || text_value.is_empty() {
                "md5"
            } else {
                "sha256"
            };
            self.set_hash_input_value(hash_type, text_value)?;
            self.set_hash_type(hash_type)?;
            Ok(())
        };

        let value = value.unwrap_or(AvatarValue::Gravatar(Vec::new()));

        match value {
            AvatarValue::Gravatar(hash) => {
                set_provider(AvatarProvider::Gravatar)?;
                set_hash(hash)?;
            }
            AvatarValue::Libravatar(hash) => {
                set_provider(AvatarProvider::Libravatar)?;
                set_hash(hash)?;
            }
            AvatarValue::Robohash(hash) => {
                set_provider(AvatarProvider::Robohash)?;
                self.text_field.set_value("".to_string())?;
                let text = hex::encode(hash);
                let mut parts = text.split('|');
                let text_value = parts.next().unwrap_or("").to_string();

                let set = parts.next().unwrap_or("set2");
                self.set_robotext(text_value, Some(set.to_string()))?;
            }
            AvatarValue::Url(url) => {
                set_provider(AvatarProvider::Custom)?;
                self.url_field.set_value(url.clone())?;
                self.set_custom_url(url)?;
            }
        }

        self.serialize_value()
    }

    fn serialize_value(&self) -> Result<Option<AvatarValue>> {
        let locked = self.inner()?;
        let params = &locked.params;
        let hash = if let Some(hash) = params.get("hash") {
            hash.clone()
        } else {
            "".to_string()
        };
        //let hash_type = if hash.len()==32{"0"}else{"1"};
        let value = match locked.provider {
            AvatarProvider::Gravatar => {
                if hash.is_empty() {
                    return Ok(None);
                }
                //format!("Gravatar|{hash}")
                AvatarValue::Gravatar(hex::decode(hash)?)
            }
            AvatarProvider::Libravatar => {
                if hash.is_empty() {
                    return Ok(None);
                }
                //format!("Robohash|{hash}")
                AvatarValue::Libravatar(hex::decode(hash)?)
            }
            AvatarProvider::Robohash => {
                let set = match params.get("robo-set") {
                    Some(s) => Self::clean_str(s)?,
                    None => "set2".to_string(),
                };
                let text = match params.get("text") {
                    Some(s) => Self::clean_str(s)?.replace('|', ""),
                    None => return Ok(None),
                };
                //format!("Robohash|{text}|{set}")
                let hash = hex::encode(format!("{text}|{set}"));
                AvatarValue::Robohash(hex::decode(hash)?)
            }
            AvatarProvider::Custom => {
                let url = match params.get("url") {
                    Some(s) => Self::clean_str(s)?,
                    None => return Ok(None),
                };
                if url.len() > 200 {
                    return Ok(None);
                }
                //format!("Custom|{}", Self::clean_str(url)?)
                AvatarValue::Url(url)
            }
        };
        Ok(Some(value))
    }

    fn set_provider(&self, provider: AvatarProvider) -> Result<()> {
        let changed = {
            let mut locked = self.inner()?;
            let old = locked.provider.clone();
            locked.provider = provider.clone();
            old.as_str() != provider.as_str()
        };
        match provider {
            AvatarProvider::Gravatar | AvatarProvider::Libravatar => {
                self.email_field.show()?;
                self.text_field.hide()?;
                self.url_field.hide()?;
                self.hash_containers.element.remove_attribute("hidden")?;
                if changed {
                    self.update_hashes(self.text_field.value())?;
                }
            }
            AvatarProvider::Robohash => {
                self.email_field.hide()?;
                self.text_field.show()?;
                self.url_field.hide()?;
                self.hash_containers
                    .element
                    .set_attribute("hidden", "true")?;
            }
            AvatarProvider::Custom => {
                self.email_field.hide()?;
                self.text_field.hide()?;
                self.url_field.show()?;
                self.hash_containers
                    .element
                    .set_attribute("hidden", "true")?;
            }
        }
        self.update_image()?;
        Ok(())
    }

    fn get_input_field(&self, hash_type: &str) -> Result<Option<FlowInputBase>> {
        let search_el = self
            .hash_containers
            .element
            .query_selector(&format!("[data-hash-type=\"{}\"] flow-input", hash_type))?;
        if let Some(el) = search_el {
            match el.dyn_into::<FlowInputBase>() {
                Ok(input) => return Ok(Some(input)),
                Err(_e) => {
                    //None
                }
            }
        }

        Ok(None)
    }

    fn on_hash_type_change(&self, hash_type: String) -> Result<()> {
        //log_trace!("on_hash_type_change: {}", hash_type);

        if let Some(input) = self.get_input_field(&hash_type)? {
            let hash = input.value();
            {
                let parems = &mut self.inner()?.params;
                parems.insert("hash", hash);
                parems.insert("hash-type", hash_type);
            }
            self.update_image()?;
        }
        Ok(())
    }
    fn on_cancel_click(&self) -> Result<()> {
        self.set_value(self.value()?)?;
        self.close_form()?;
        Ok(())
    }

    fn show_change_btn(&self, show: bool) -> Result<()> {
        let btn = &self.change_btn.element;
        if show {
            btn.remove_attribute("hidden")?;
        } else {
            btn.set_attribute("hidden", "true")?;
        }
        Ok(())
    }
    fn show_save_btn(&self, show: bool) -> Result<()> {
        if show {
            self.show_change_btn(false)?;
            self.save_btn.element.remove_attribute("hidden")?;
            self.cancel_btn.element.remove_attribute("hidden")?;
        } else {
            self.save_btn.element.set_attribute("hidden", "true")?;
            self.cancel_btn.element.set_attribute("hidden", "true")?;
            self.form_container.class_list().remove_1("open")?;
        }
        Ok(())
    }

    fn build_md5_hash(content: String) -> Result<String> {
        if content.is_empty() {
            return Ok("".to_string());
        }
        Ok(format!("{:x}", md5::compute(content)))
    }

    fn build_sha256_hash(content: String) -> Result<String> {
        if content.is_empty() {
            return Ok("".to_string());
        }
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    fn update_hashes(&self, email: String) -> Result<()> {
        let md5 = Self::build_md5_hash(email.clone())?;
        let sha256 = Self::build_sha256_hash(email)?;
        self.set_hash_input_value("md5", md5.clone())?;
        self.set_hash_input_value("sha256", sha256.clone())?;

        let changed = {
            let parems = &mut self.inner()?.params;
            if let Some(hash_type) = parems.get("hash-type") {
                if hash_type.eq("md5") {
                    parems.insert("hash", md5);
                    true
                } else if hash_type.eq("sha256") {
                    parems.insert("hash", sha256);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if changed {
            self.update_image()?;
        }
        Ok(())
    }

    fn set_robotext(&self, text: String, set: Option<String>) -> Result<()> {
        {
            let params = &mut self.inner()?.params;
            params.insert("text", text);
            if let Some(set) = set {
                params.insert("robo-set", set);
            }
        }
        self.update_image()?;
        Ok(())
    }

    fn set_custom_url(&self, url: String) -> Result<()> {
        {
            self.inner()?.params.insert("url", url);
        }
        self.update_image()?;
        Ok(())
    }

    fn set_hash_input_value(&self, hash_type: &str, value: String) -> Result<()> {
        if let Some(input) = self.get_input_field(hash_type)? {
            //log_trace!("set_hash_input_value value: {} ", value);
            FieldHelper::set_value_attr(&input, &value)?;
        }
        Ok(())
    }

    pub fn update_image(&self) -> Result<()> {
        self.image
            .set_src_and_fallback(&self.build_url(300)?, &self.inner()?.fallback)?;
        Ok(())
    }

    pub fn build_url(&self, size: u16) -> Result<String> {
        let locked = self.inner()?;
        let params = &locked.params;
        let hash = if let Some(hash) = params.get("hash") {
            hash.clone()
        } else {
            "".to_string()
        };
        let url = match locked.provider {
            AvatarProvider::Gravatar => {
                if hash.is_empty() {
                    return Ok(locked.fallback.clone());
                }
                format!("https://s.gravatar.com/avatar/{hash}?s={size}&d=404")
            }
            AvatarProvider::Libravatar => {
                if hash.is_empty() {
                    return Ok(locked.fallback.clone());
                }
                format!("https://libravatar.org/avatar/{hash}?s={size}&d=404")
            }
            AvatarProvider::Robohash => {
                let set = match params.get("robo-set") {
                    Some(s) => Self::clean_str(s)?,
                    None => "set2".to_string(),
                };
                let text = match params.get("text") {
                    Some(s) => Self::clean_str(s)?,
                    None => return Ok(locked.fallback.clone()),
                };
                format!(
                    "https://robohash.org/{text}.jpg?ignoreext=false&size={size}x{size}&set={set}"
                )
            }
            AvatarProvider::Custom => {
                let url = match params.get("url") {
                    Some(s) => Self::clean_str(s)?,
                    None => return Ok(locked.fallback.clone()),
                };
                Self::clean_str(url)?
            }
        };
        Ok(url)
    }

    fn clean_str<T: Into<String>>(str: T) -> Result<String> {
        let text: String = str.into();
        FieldHelper::clean_value_for_attr(&text)
    }
}
