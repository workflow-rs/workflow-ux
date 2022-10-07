use std::collections::BTreeMap;
use std::sync::LockResult;

use crate::prelude::*;
use crate::result::Result;
use crate::icon::Icon;
use crate::controls::listener::Listener;
use std::sync::MutexGuard;
//use workflow_core::id::Id;
//use workflow_html::Html;

pub struct ListRow{
    pub id: String,
    pub title: String,
    pub sub: Option<String>,
    pub value: Option<String>,
    pub left_icon: Option<String>,
    pub right_icon: Option<String>,
    pub right_icon_click_listener: Option<Listener<web_sys::MouseEvent>>,
    pub cls: Option<String>,
    pub editable: bool
}

impl Default for ListRow {
    fn default() -> Self {
        Self{
            id:String::new(),
            title:String::new(),
            sub:None,
            value:None,
            left_icon:None,
            right_icon:None,
            right_icon_click_listener:None,
            cls:None,
            editable: true
        }
    }
}

impl ListRow{
    pub fn render_el(&mut self)->Result<Element>{
        let info_row_el = create_el("div", vec![("class", "info-row")], None)?;

        let title_el = create_el("div", vec![("class", "title")], Some(&self.title))?;
        let title_box_el = create_el("div", vec![("class", "title-box")], None)?;
        title_box_el.append_child(&title_el)?;

        if let Some(sub_title) = self.sub.as_ref(){
            let el = create_el("div", vec![("class", "sub-title")], Some(sub_title))?;
            title_box_el.append_child(&el)?;
        }
        
        if let Some(icon) = self.left_icon.as_ref(){
            let el = create_el("img", vec![("class", "icon left"), ("src", icon)], None)?;
            info_row_el.append_child(&el)?;
        }

        info_row_el.append_child(&title_box_el)?;

        if let Some(value) = self.value.as_ref(){
            let el = create_el("div", vec![("class", "value")], Some(value))?;
            info_row_el.append_child(&el)?;
        }

        if self.editable{
            let el = Icon::css("info-row-edit").element()?;
            el.set_attribute("data-action", "edit")?;
            info_row_el.append_child(&el)?;
        }

        if let Some(icon) = self.right_icon.as_ref(){
            let el = create_el("img", vec![("class", "icon right"), ("src", icon)], None)?;
            info_row_el.append_child(&el)?;

            if let Some(listener) = &self.right_icon_click_listener{
                el.add_event_listener_with_callback("click", listener.into_js())?;
            }
        }

        if let Some(cls) = self.cls.as_ref(){
            info_row_el.class_list().add_1(cls)?;
        }

        Ok(info_row_el)
    }

}


pub trait ListBuilder:Clone{
    fn new()->Result<Self>;

    fn list(&self, start:usize, limit:usize)->Result<Vec<ListRow>>;

    fn addable(&self)->Result<bool>;

    fn edit_form<B:ListBuilder+'static>(
        &mut self,
        builder:&Builder<B>,
        id:String,
        row:Element,
        btn:Element
    )->Result<()>;

    fn add_form<B:ListBuilder+'static>(
        &mut self,
        b:&Builder<B>
    )->Result<()>;

    fn save<B:ListBuilder+'static>(
        &mut self,
        b:&Builder<B>,
        id:Option<String>
    )->Result<bool>;
}



pub struct BuilderInner {
    pub layout : ElementLayout,
    pub attributes: Attributes,
    pub docs : Docs,

    pub items: Arc<BTreeMap<String, ListRow>>,
    pub list_start: usize,
    pub list_limit: usize,
    pub editing_id: Option<String>,

    pub action_container: Element,
    pub list_container: ElementWrapper,
    pub form_container: ElementWrapper,
    pub save_btn: ElementWrapper,
    pub add_btn: ElementWrapper,
    pub cancel_btn: ElementWrapper,
}

#[derive(Clone)]
pub struct Builder<B> {
    pub element : Element,
    inner: Arc<Mutex<BuilderInner>>,
    //b: PhantomData<B>,
    pub imp: Arc<Mutex<B>>
}

unsafe impl<B> Send for Builder<B> where B:ListBuilder{}

impl<B> Builder<B> 
where B:ListBuilder+'static{
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub fn list_builder(&self)->Arc<Mutex<B>>{
        self.imp.clone()
    }

    pub fn inner(&self) -> LockResult<MutexGuard<BuilderInner>> {
        self.inner.lock()
    }

    pub fn new(pane : &ElementLayout, attributes: &Attributes, docs : &Docs) -> Result<Self> {

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
        

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
        }

        let mut builder = Self {
            inner: Arc::new(Mutex::new(BuilderInner{
                layout : pane.clone(),
                attributes : attributes.clone(),
                docs : docs.clone(),
                items: Arc::new(BTreeMap::new()),
                list_start:0,
                list_limit:50,
                editing_id:None,
                list_container:ElementWrapper::new(list_container),
                form_container:ElementWrapper::new(form_container),
                save_btn:ElementWrapper::new(save_btn),
                add_btn:ElementWrapper::new(add_btn),
                cancel_btn:ElementWrapper::new(cancel_btn),
                action_container,
            })),
            element,
            //b:PhantomData,
            imp: Arc::new(Mutex::new(B::new()?))
        };

        builder = builder.init()?;

        Ok(builder)
    }

    fn init(mut self)->Result<Self>{
        let mut this = self.clone();
        {
            let mut locked = self.inner()?;
            locked.list_container.on_click(move |e|->Result<()>{
                if let Some(et) = e.target(){
                    match et.dyn_into::<Element>(){
                        Ok(el)=>{
                            log_trace!("on-row-click: {:?}", el);
                            this.on_row_click(el)?;
                        }
                        Err(e)=>{
                            log_error!("Builder: Could not cast EventTarget to Element: {:?}", e);
                        }
                    }
                }
                Ok(())
            })?;

            let mut this = self.clone();
            locked.add_btn.on_click(move |_|->Result<()>{
                this.on_add_click()?;
                Ok(())
            })?;
            let mut this = self.clone();
            locked.save_btn.on_click(move |_|->Result<()>{
                this.on_save_click()?;
                Ok(())
            })?;
            let mut this = self.clone();
            locked.cancel_btn.on_click(move |_|->Result<()>{
                this.on_cancel_click()?;
                Ok(())
            })?;
        }
        self.show_save_btn(false)?;
        self.update_list()?;

        Ok(self)
    }

    fn on_add_click(&mut self)->Result<()>{
        self.imp.lock()?.add_form(self)?;
        self.set_save_btn_text(&i18n("Add"))?;
        self.inner()?.editing_id = None;
        Ok(())
    }

    fn on_save_click(&mut self)->Result<()>{
        let id = {self.inner()?.editing_id.clone()};
        let valid = self.imp.lock()?.save(&self, id)?;
        log_trace!("on_save_click:valid {}", valid);
        if valid{
            self.show_save_btn(false)?;
            self.update_list()?;
        }
        Ok(())
    }
    fn on_cancel_click(&mut self)->Result<()>{
        self.show_save_btn(false)?;
        self.show_add_btn(self.imp.lock()?.addable()?)?;
        Ok(())
    }

    fn on_row_click(&mut self, target:Element)->Result<()>{
        if let Some(row) = target.closest(".info-row")?{
            let uid = row.get_attribute("data-uid");
            if let Some(btn) = target.closest("[data-action]")?{
                let action = btn.get_attribute("data-action").ok_or(String::new())?;
                if action.eq("edit"){
                    self.on_row_edit_click(row, uid, btn)?;
                }
            }
        }
        Ok(())
    }

    fn on_row_edit_click(&mut self, row:Element, uid:Option<String>, btn:Element)->Result<()>{
        if let Some(id) = uid.as_ref(){
            //if let Some(data) = self.items.get(id){
                self.inner()?.editing_id = Some(id.clone());
                self.set_save_btn_text(&i18n("Update"))?;
                self.imp.lock()?.edit_form(self, id.clone(), row, btn)?;
            //}else{
            //    log_error!("Builder: Unbale to get Info row for uid: {}", id);
            //}
        }
        Ok(())
    }

    fn set_save_btn_text(&self, text:&str)->Result<()>{
        self.inner()?.save_btn.element.set_inner_html(text);
        Ok(())
    }

    fn show_add_btn(&self, show:bool)->Result<()>{
        let btn = &self.inner()?.add_btn.element;
        if show{
            btn.remove_attribute("hidden")?;
        }else{
            btn.set_attribute("hidden", "true")?;
        }
        Ok(())
    }
    fn show_save_btn(&self, show:bool)->Result<()>{
        let locked = self.inner()?;
        if show{
            locked.save_btn.element.remove_attribute("hidden")?;
            locked.cancel_btn.element.remove_attribute("hidden")?;
        }else{
            locked.save_btn.element.set_attribute("hidden", "true")?;
            locked.cancel_btn.element.set_attribute("hidden", "true")?;
            locked.form_container.element.set_inner_html("");
        }
        Ok(())
    }

    pub fn update_list(&mut self)->Result<()>{
        let locked = self.imp.lock()?;
        {
            let mut items:BTreeMap<String, ListRow> = BTreeMap::new();
            let mut this = self.inner()?;
            let list_el = &this.list_container.element;
            let list = locked.list(this.list_start, this.list_limit)?;
            
            list_el.set_inner_html("");
            for mut item in list{
                //let id = Id::new().to_string();
                let el = item.render_el()?;
                el.set_attribute("data-uid", &item.id)?;
                list_el.append_child(&el)?;
                items.insert(item.id.clone(), item);
            }

            this.items = Arc::new(items);
        }

        self.show_add_btn(locked.addable()?)?;
        
        Ok(())
    }

    pub fn set_form(&self, form:&Element)->Result<()>{
        {
            let form_el = &self.inner()?.form_container.element;
            form_el.set_inner_html("");
            form_el.append_child(form)?;
        }
        self.show_add_btn(false)?;
        self.show_save_btn(true)?;
        Ok(())
    }

}

