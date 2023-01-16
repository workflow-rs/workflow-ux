use std::collections::BTreeMap;
use std::sync::LockResult;

use crate::icon::Icon;
use crate::prelude::*;
use crate::result::Result;
use std::sync::MutexGuard;
use workflow_wasm::callback::Callback;
use workflow_wasm::prelude::*;

pub struct ListRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub sub: Option<String>,
    pub value: Option<String>,
    pub left_icon: Option<String>,
    pub right_icon: Option<String>,
    pub right_icon_click_listener: Option<Callback<CallbackClosure<web_sys::MouseEvent>>>,
    pub cls: Option<String>,
    pub editable: bool,
    pub deletable: bool,
    pub orderable: bool,
}

impl Default for ListRow {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            description: None,
            sub: None,
            value: None,
            left_icon: None,
            right_icon: None,
            right_icon_click_listener: None,
            cls: None,
            editable: true,
            deletable: true,
            orderable: true,
        }
    }
}

impl ListRow {
    pub fn render_el(&mut self) -> Result<Element> {
        let info_row_el = create_el("div", vec![("class", "info-row")], None)?;

        let title_el = create_el("div", vec![("class", "title")], Some(&self.title))?;
        let title_box_el = create_el("div", vec![("class", "title-box")], None)?;
        title_box_el.append_child(&title_el)?;

        if let Some(sub_title) = self.sub.as_ref() {
            let el = create_el("div", vec![("class", "sub-title")], Some(sub_title))?;
            title_box_el.append_child(&el)?;
        }

        if let Some(description) = self.description.as_ref() {
            let el = create_el("div", vec![("class", "description")], Some(description))?;
            title_box_el.append_child(&el)?;
        }

        if let Some(icon) = self.left_icon.as_ref() {
            let el = create_el("img", vec![("class", "icon left"), ("src", icon)], None)?;
            info_row_el.append_child(&el)?;
        }

        info_row_el.append_child(&title_box_el)?;

        if let Some(value) = self.value.as_ref() {
            let el = create_el("div", vec![("class", "value")], Some(value))?;
            info_row_el.append_child(&el)?;
        }

        if self.orderable {
            let el = Icon::css("info-row-order-up").element()?;
            el.set_attribute("data-action", "order-up")?;
            info_row_el.append_child(&el)?;
            let el = Icon::css("info-row-order-down").element()?;
            el.set_attribute("data-action", "order-down")?;
            info_row_el.append_child(&el)?;
        }

        if self.editable {
            let el = Icon::css("info-row-edit").element()?;
            el.set_attribute("data-action", "edit")?;
            info_row_el.append_child(&el)?;
        }
        if self.deletable {
            let el = Icon::css("info-row-delete").element()?;
            el.set_attribute("data-action", "delete")?;
            info_row_el.append_child(&el)?;
        }

        if let Some(icon) = self.right_icon.as_ref() {
            let el = create_el("img", vec![("class", "icon right"), ("src", icon)], None)?;
            info_row_el.append_child(&el)?;

            if let Some(listener) = &self.right_icon_click_listener {
                el.add_event_listener_with_callback("click", listener.as_ref())?;
            }
        }

        if let Some(cls) = self.cls.as_ref() {
            info_row_el.class_list().add_1(cls)?;
        }

        Ok(info_row_el)
    }
}

pub trait ListBuilderItem: Clone {
    fn id(&self) -> String;
}

pub trait ListBuilder<I: ListBuilderItem>: Clone {
    fn new() -> Result<Self>;

    fn list(&self, items: &[I], start: usize, limit: usize) -> Result<Vec<ListRow>>;

    fn addable(&self, len: usize) -> Result<bool>;

    fn form_element(&self) -> Result<Element>;

    fn edit_form<B: ListBuilder<I>>(&mut self, builder: &Builder<B, I>, data: I) -> Result<bool>;

    fn add_form<B: ListBuilder<I>>(&mut self, b: &Builder<B, I>) -> Result<bool>;

    fn save<B: ListBuilder<I>>(
        &mut self,
        b: &Builder<B, I>,
        editing_item: Option<I>,
    ) -> Result<bool>;

    //fn delete(&mut self, id:String)->Result<bool>;
    //fn order(&mut self, id:String, order_up:bool)->Result<bool>;

    fn delete(&mut self, _id: String) -> Result<bool> {
        //self.items.retain(|item|  !item.id.eq(&id) );
        Ok(false)
    }
    fn order(&mut self, _id: String, _order_up: bool) -> Result<bool> {
        /*let index = match self.items.iter().position(|item| item.id.eq(&id)){
            Some(index)=>index,
            None=>return Ok(false)
        };
        let last_index = self.items.len()-1;
        if (index == 0 && order_up) || (index == last_index && !order_up){
            return Ok(false);
        }

        let replace_index = if order_up {index-1}else{index+1};
        let item = self.items[index].clone();
        let other_item = self.items[replace_index].clone();
        self.items[index] = other_item;
        self.items[replace_index] = item;*/

        Ok(false)
    }
}

pub struct BuilderInner<I: ListBuilderItem> {
    pub layout: ElementLayout,
    pub attributes: Attributes,
    pub docs: Docs,
    pub list_items: Vec<I>,
    pub items: Arc<BTreeMap<String, ListRow>>,
    pub list_start: usize,
    pub list_limit: usize,
    pub editing_item: Option<I>,

    pub action_container: Element,
    pub list_container: ElementWrapper,
    pub form_container: ElementWrapper,
    pub save_btn: ElementWrapper,
    pub add_btn: ElementWrapper,
    pub cancel_btn: ElementWrapper,
}

#[derive(Clone)]
pub struct Builder<B: ListBuilder<I> + 'static, I: ListBuilderItem> {
    pub element: Element,
    inner: Arc<Mutex<BuilderInner<I>>>,
    //b: PhantomData<I>,
    pub imp: Arc<Mutex<B>>,
}

unsafe impl<B, I: ListBuilderItem> Send for Builder<B, I> where B: ListBuilder<I> {}

impl<B, I> Builder<B, I>
where
    B: ListBuilder<I> + 'static,
    I: ListBuilderItem + 'static,
{
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn list_builder(&self) -> Arc<Mutex<B>> {
        self.imp.clone()
    }

    pub fn inner(&self) -> LockResult<MutexGuard<BuilderInner<I>>> {
        self.inner.lock()
    }

    pub fn get_item(&self, id: String) -> Result<Option<I>> {
        for item in self.inner()?.list_items.iter() {
            if item.id().eq(&id) {
                return Ok(Some(item.clone()));
            }
        }

        Ok(None)
    }

    pub fn push_item(&self, item: I) -> Result<()> {
        self.inner()?.list_items.push(item);
        Ok(())
    }
    pub fn find_index(&self, id: &str) -> Result<Option<usize>> {
        let index = self
            .inner()?
            .list_items
            .iter()
            .position(|item| item.id().eq(id));
        Ok(index)
    }
    pub fn update_item(&self, item: I) -> Result<()> {
        let index = match self.find_index(&item.id())? {
            Some(index) => index,
            None => return Ok(()),
        };
        self.inner()?.list_items[index] = item;
        Ok(())
    }

    pub fn value(&self) -> Result<Vec<I>> {
        let items = self.inner()?.list_items.clone();
        Ok(items)
    }
    pub fn set_value(&self, items: Vec<I>) -> Result<()> {
        {
            self.inner()?.list_items = items;
            self.show_add_btn(false)?;
        }
        self.clone().update_list()?;
        Ok(())
    }

    pub fn items_len(&self) -> Result<usize> {
        Ok(self.inner()?.list_items.len())
    }

    pub fn new(pane: &ElementLayout, attributes: &Attributes, docs: &Docs) -> Result<Self> {
        let element = create_el("div", vec![("class", "list-builder")], None)?;

        let list_container = create_el("div", vec![("class", "list-container")], None)?;
        element.append_child(&list_container)?;

        let form_container = create_el("div", vec![("class", "form-container")], None)?;
        element.append_child(&form_container)?;

        let add_btn = create_el("flow-btn", vec![("class", "add")], None)?;
        add_btn.set_inner_html(&i18n("Add"));
        let save_btn = create_el("flow-btn", vec![("class", "save")], None)?;
        save_btn.set_inner_html(&i18n("Save"));
        let cancel_btn = create_el("flow-btn", vec![("class", "cancel")], None)?;
        cancel_btn.set_inner_html(&i18n("Cancel"));

        let action_container = create_el("div", vec![("class", "action-container")], None)?;
        action_container.append_child(&add_btn)?;
        action_container.append_child(&save_btn)?;
        action_container.append_child(&cancel_btn)?;
        element.append_child(&action_container)?;
        //element.set_inner_html("<h1>builder</h1>");

        for (k, v) in attributes.iter() {
            element.set_attribute(k, v)?;
        }

        let mut builder = Self {
            inner: Arc::new(Mutex::new(BuilderInner {
                layout: pane.clone(),
                attributes: attributes.clone(),
                docs: docs.clone(),
                list_items: Vec::new(),
                items: Arc::new(BTreeMap::new()),
                list_start: 0,
                list_limit: 50,
                editing_item: None,
                list_container: ElementWrapper::new(list_container),
                form_container: ElementWrapper::new(form_container),
                save_btn: ElementWrapper::new(save_btn),
                add_btn: ElementWrapper::new(add_btn),
                cancel_btn: ElementWrapper::new(cancel_btn),
                action_container,
            })),
            element,
            //b:PhantomData,
            imp: Arc::new(Mutex::new(B::new()?)),
        };

        builder = builder.init()?;

        Ok(builder)
    }

    fn init(mut self) -> Result<Self> {
        let mut this = self.clone();
        {
            let mut locked = self.inner()?;
            locked.list_container.on_click(move |e| -> Result<()> {
                if let Some(et) = e.target() {
                    match et.dyn_into::<Element>() {
                        Ok(el) => {
                            this.on_row_click(el)?;
                        }
                        Err(e) => {
                            log_error!("Builder: Could not cast EventTarget to Element: {:?}", e);
                        }
                    }
                }
                Ok(())
            })?;

            let mut this = self.clone();
            locked.add_btn.on_click(move |_| -> Result<()> {
                this.on_add_click()?;
                Ok(())
            })?;
            let mut this = self.clone();
            locked.save_btn.on_click(move |_| -> Result<()> {
                this.on_save_click()?;
                Ok(())
            })?;
            let mut this = self.clone();
            locked.cancel_btn.on_click(move |_| -> Result<()> {
                this.on_cancel_click()?;
                Ok(())
            })?;
        }
        self.show_save_btn(false)?;
        self.update_list()?;

        Ok(self)
    }

    fn on_add_click(&mut self) -> Result<()> {
        let show = self.imp.lock()?.add_form(self)?;
        if show {
            self.set_form(&self.imp.lock()?.form_element()?)?;
        }
        self.set_save_btn_text(&i18n("Add"))?;
        self.inner()?.editing_item = None;
        Ok(())
    }

    fn on_save_click(&mut self) -> Result<()> {
        let editing_item = { self.inner()?.editing_item.clone() };
        let valid = self.imp.lock()?.save(self, editing_item)?;
        log_trace!("on_save_click:valid {}", valid);
        if valid {
            self.show_save_btn(false)?;
            self.update_list()?;
        }
        Ok(())
    }

    fn on_cancel_click(&mut self) -> Result<()> {
        self.show_save_btn(false)?;
        let len = { self.inner()?.list_items.len() };
        self.show_add_btn(self.imp.lock()?.addable(len)?)?;
        Ok(())
    }

    fn on_row_click(&mut self, target: Element) -> Result<()> {
        if let Some(row) = target.closest(".info-row")? {
            let id = match row.get_attribute("data-uid") {
                Some(id) => id,
                None => return Ok(()),
            };
            if let Some(btn) = target.closest("[data-action]")? {
                let action = btn.get_attribute("data-action").ok_or(String::new())?;
                log_trace!("on-row-click: action:{:?}", action);
                if action.eq("edit") {
                    self.on_row_edit_click(id)?;
                } else if action.eq("delete") {
                    self.on_row_delete_click(id)?;
                } else if action.eq("order-up") {
                    self.on_row_order_click(id, true)?;
                } else if action.eq("order-down") {
                    self.on_row_order_click(id, false)?;
                }
            }
        }
        Ok(())
    }

    fn delete(&self, id: String) -> Result<bool> {
        self.inner()?.list_items.retain(|item| !item.id().eq(&id));
        Ok(true)
    }
    fn order(&self, id: String, order_up: bool) -> Result<bool> {
        let items: &mut Vec<I> = &mut self.inner()?.list_items;
        let index = match items.iter().position(|item| item.id().eq(&id)) {
            Some(index) => index,
            None => return Ok(false),
        };
        let last_index = items.len() - 1;
        if (index == 0 && order_up) || (index == last_index && !order_up) {
            return Ok(false);
        }

        let replace_index = if order_up { index - 1 } else { index + 1 };
        let item = items[index].clone();
        let other_item = items[replace_index].clone();
        items[index] = other_item;
        items[replace_index] = item;

        Ok(true)
    }

    fn on_row_delete_click(&mut self, id: String) -> Result<()> {
        let done = self.imp.lock()?.delete(id.clone())?;
        if done || self.delete(id)? {
            self.update_list()?;
        }
        Ok(())
    }
    fn on_row_order_click(&mut self, id: String, order_up: bool) -> Result<()> {
        let done = self.imp.lock()?.order(id.clone(), order_up)?;
        if done || self.order(id, order_up)? {
            self.update_list()?;
        }
        Ok(())
    }

    fn on_row_edit_click(&mut self, id: String) -> Result<()> {
        let data = match self.get_item(id)? {
            Some(data) => data,
            None => return Ok(()),
        };
        self.inner()?.editing_item = Some(data.clone());
        self.set_save_btn_text(&i18n("Update"))?;
        let show = self.imp.lock()?.edit_form(self, data)?;
        if show {
            self.set_form(&self.imp.lock()?.form_element()?)?;
        }
        Ok(())
    }

    fn set_save_btn_text(&self, text: &str) -> Result<()> {
        self.inner()?.save_btn.element.set_inner_html(text);
        Ok(())
    }

    fn show_add_btn(&self, show: bool) -> Result<()> {
        let btn = &self.inner()?.add_btn.element;
        if show {
            btn.remove_attribute("hidden")?;
        } else {
            btn.set_attribute("hidden", "true")?;
        }
        Ok(())
    }
    fn show_save_btn(&self, show: bool) -> Result<()> {
        let locked = self.inner()?;
        if show {
            locked.save_btn.element.remove_attribute("hidden")?;
            locked.cancel_btn.element.remove_attribute("hidden")?;
        } else {
            locked.save_btn.element.set_attribute("hidden", "true")?;
            locked.cancel_btn.element.set_attribute("hidden", "true")?;
            //locked.form_container.element.set_inner_html("");
            locked
                .form_container
                .element
                .class_list()
                .remove_1("open")?;
        }
        Ok(())
    }

    pub fn update_list(&mut self) -> Result<()> {
        let locked = self.imp.lock()?;
        {
            let mut items: BTreeMap<String, ListRow> = BTreeMap::new();
            let mut this = self.inner()?;
            let list_el = &this.list_container.element;
            let list = locked.list(&this.list_items, this.list_start, this.list_limit)?;

            list_el.set_inner_html("");
            for mut item in list {
                //let id = Id::new().to_string();
                let el = item.render_el()?;
                el.set_attribute("data-uid", &item.id)?;
                list_el.append_child(&el)?;
                items.insert(item.id.clone(), item);
            }

            this.items = Arc::new(items);
        }
        let len = self.items_len()?;
        let add_allowed = locked.addable(len)?;
        self.show_add_btn(add_allowed)?;

        Ok(())
    }

    pub fn set_form(&self, form: &Element) -> Result<()> {
        {
            let form_el = &self.inner()?.form_container.element;
            form_el.class_list().remove_1("open")?;
            form_el.set_inner_html("");
            form_el.append_child(form)?;
            form_el.class_list().add_1("open")?;
        }
        self.show_add_btn(false)?;
        self.show_save_btn(true)?;
        Ok(())
    }
}
