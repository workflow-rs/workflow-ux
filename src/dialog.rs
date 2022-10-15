use core::fmt;
use std::{sync::{Arc, Mutex, MutexGuard, LockResult}, collections::BTreeMap};
use crate::prelude::*;
use crate::result::Result;
use workflow_html::{html, Render, Hooks, Renderables, ElementResult, Html};
use workflow_core::id::Id;

static mut DIALOGES : Option<BTreeMap<String, Dialog>> = None;


pub type Callback = Box<dyn FnMut(Dialog, DialogButton)->Result<()>>;

pub enum ButtonClass{
    Primary,
    Secondary,
    Success,
    Warning,
    Info
}

impl ButtonClass{
    pub fn to_string(&self)->String{
        match self{
            Self::Primary=>"Primary",
            Self::Secondary=>"Secondary",
            Self::Success=>"Success",
            Self::Warning=>"Warning",
            Self::Info=>"Info"
        }.to_string()
    }
}

//#[describe_enum]
#[derive(Clone)]
pub enum DialogButton{
    Ok,
    Cancel,
    Done,
    Save,
    Exit,
    TryIt,
    NotNow,
    Subscribe,
    Accept,
    Decline,
    Run,
    Delete,
    Print,
    Start,
    Stop,
    Discard,
    Yes,
    No,
    GotIt,
    Custom(String),
    __WithClass(String, String)
}

impl DialogButton{

    pub fn with_class(&self, class:ButtonClass)->Self{
        let (name, _cls) = self.name_and_class();
        Self::__WithClass(name, class.to_string())
    }
    pub fn name_and_class(&self)->(String, Option<String>){
        let (name, class) = match self{
            Self::Ok=>("Ok", None),
            Self::Cancel=>("Cancel", None),
            Self::Done=>("Done", None),
            Self::Save=>("Save", None),
            Self::Exit=>("Exit", None),
            Self::TryIt=>("Try It", None),
            Self::NotNow=>("Not Now", None),
            Self::Subscribe=>("Subscribe", None),
            Self::Accept=>("Accept", None),
            Self::Decline=>("Decline", None),
            Self::Run=>("Run", None),
            Self::Delete=>("Delete", None),
            Self::Print=>("Print", None),
            Self::Start=>("Start", None),
            Self::Stop=>("Stop", None),
            Self::Discard=>("Discard", None),
            Self::Yes=>("Yes", None),
            Self::No=>("No", None),
            Self::GotIt=>("Got It", None),
            Self::Custom(str)=>(str.as_str(), None),
            Self::__WithClass(name, class)=>{
                (name.as_str(), Some(class.clone()))
            }
        };

        (name.to_string(), class)

    }

    pub fn from_str(str:&str)->Option<Self>{
        match str{
            "Ok"=>Some(Self::Ok),
            "Cancel"=>Some(Self::Cancel),
            "Done"=>Some(Self::Done),
            "Save"=>Some(Self::Save),
            "Exit"=>Some(Self::Exit),
            "Try It"=>Some(Self::TryIt),
            "Not Now"=>Some(Self::NotNow),
            "Subscribe"=>Some(Self::Subscribe),
            "Accept"=>Some(Self::Accept),
            "Decline"=>Some(Self::Decline),
            "Run"=>Some(Self::Run),
            "Delete"=>Some(Self::Delete),
            "Print"=>Some(Self::Print),
            "Start"=>Some(Self::Start),
            "Stop"=>Some(Self::Stop),
            "Discard"=>Some(Self::Discard),
            "Yes"=>Some(Self::Yes),
            "No"=>Some(Self::No),
            "Got It"=>Some(Self::GotIt),
            _=>{
                Some(Self::Custom(str.to_string()))
            }
        }
    }

    fn render_with_class(
        self,
        parent:&mut Element,
        map:&mut Hooks,
        renderables:&mut Renderables,
        _class:Option<String>
    )->ElementResult<()>
    {
        let (name, class) = self.name_and_class();
        let action = name.replace("\"", "");
        //text = text.replace("Custom::", "");

        let cls = class.unwrap_or("".to_string()).to_lowercase();
        
        let body = html!{
            <flow-btn data-action={action} class={cls}>
                {name}
            </flow-btn>
        }?;
    
        body.render_node(parent, map, renderables)?;

        Ok(())
    }
}

impl fmt::Display for DialogButton{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_and_class().0)
    }
}

#[derive(Clone)]
pub struct DialogButtonData{
    pub btn:DialogButton,
    pub class:Option<String>
}

impl DialogButtonData{
    fn list_from<D:Into<DialogButtonData>+Clone>(list:&[D]) -> Vec<DialogButtonData> {
        list.iter().map(|a| {
            let b: DialogButtonData = (*a).clone().into();
            b
        }).collect()
    }
}

impl From<DialogButton> for DialogButtonData{
    fn from(btn: DialogButton) -> Self {
        Self{
            btn,
            class: None
        }
    }
}

impl From<(DialogButton, &str)> for DialogButtonData{
    fn from(info: (DialogButton, &str)) -> Self {
        Self{
            btn: info.0,
            class: Some(info.1.to_string())
        }
    }
}

/*
impl<D:Into<DialogButtonData>> From<&D> for DialogButtonData{
    fn from(info: &D) -> Self {
        info.clone().into()
    }
}
*/

impl Render for DialogButtonData{
    fn render(&self, _w: &mut Vec<String>)->ElementResult<()> {
        Ok(())
    }
    fn render_node(
        self,
        parent:&mut Element,
        map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>
    {

        self.btn.render_with_class(parent, map, renderables, self.class)?;

        Ok(())
    }
}




#[derive(Clone)]
pub struct DialogButtons{
    pub left:Vec<DialogButtonData>,
    pub center:Vec<DialogButtonData>,
    pub right:Vec<DialogButtonData>
}

impl DialogButtons{
    pub fn new<A,B,C>(left:&[A], center:&[B],right:&[C])->Self
    where A:Into<DialogButtonData>+Clone,
    B:Into<DialogButtonData>+Clone,
    C:Into<DialogButtonData>+Clone
    {

        Self {
            left: DialogButtonData::list_from(left),
            center: DialogButtonData::list_from(center),
            right: DialogButtonData::list_from(right),
        }
    }
}

impl Render for DialogButtons{
    fn render(&self, _w: &mut Vec<String>)->ElementResult<()> {
        Ok(())
    }
    fn render_node(
        self,
        parent:&mut Element,
        map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>
    {
        let body = html!{
            <div class="left-buttons">
                {self.left}
            </div>
            <div class="center-buttons">
                {self.center}
            </div>
            <div class="right-buttons">
                {self.right}
            </div>
        }?;
    
        body.render_node(parent, map, renderables)?;

        Ok(())
    }
}

pub struct DialogInner{
    msg:Option<Html>,
    _body:Html,
    title:Element,
    body:Element,
    btns:Element
}

#[derive(Clone)]
pub struct Dialog{
    id:String,
    element: Element,
    inner: Arc<Mutex<Option<DialogInner>>>,
    callback: Arc<Mutex<Callback>>
}

impl Dialog{
    pub fn new()->Result<Self>{
        Ok(Self::create::<DialogButton, DialogButton, DialogButton>(None, &[], &[], &[DialogButton::Ok])?)
    }

    pub fn new_with_body_and_buttons<A,B,C>(body:Html, left_btns:&[A], center_btns:&[B], right_btns:&[C])->Result<Self>
    where A:Into<DialogButtonData>+Clone,
    B:Into<DialogButtonData>+Clone,
    C:Into<DialogButtonData>+Clone
    {
        Ok(Self::create(Some(body), left_btns, center_btns, right_btns)?)
    }

    pub fn new_with_btns<A,B,C>(left:&[A], center:&[B], right:&[C])->Result<Self>
    where A:Into<DialogButtonData>+Clone,
    B:Into<DialogButtonData>+Clone,
    C:Into<DialogButtonData>+Clone
    {
        Ok(Self::create(None, left, center, right)?)
    }

    fn create<A,B,C>(body_html:Option<Html>, left:&[A], center:&[B], right:&[C])->Result<Self>
    where A:Into<DialogButtonData>+Clone,
    B:Into<DialogButtonData>+Clone,
    C:Into<DialogButtonData>+Clone
    {
        let btns = DialogButtons::new(left, center, right);
        Ok(Self {
            id: format!("dialog_{}", Id::new()),
            element: create_el("div.workflow-dialog", vec![], None)?,
            inner: Arc::new(Mutex::new(None)),
            callback: Arc::new(Mutex::new(Box::new(|d:Dialog, _btn|{
                d.close()?;
                Ok(())
            })))
        }.init(body_html, btns)?)
    }

    fn init(self, body_html:Option<Html>, btns:DialogButtons)->Result<Self>{
        
        let this = self.clone();
        let body = html!{
            <div class="workflow-dialog-inner">
                <h2 class="title" @title></h2>
                <div class="body" @body>
                    {body_html}
                </div>
                <div class="actions" @btns
                    !click={
                        this.on_btn_click(_event, _target).map_err(|e|{
                            log_trace!("error: {}", e);
                        }).ok();
                    }>
                    {btns}
                </div>
            </div>
        }?;

        body.inject_into(&self.element)?;

        document().body().unwrap().append_child(&self.element)?;

        let hooks = body.hooks().clone();
        let inner_dialog = DialogInner{
            _body:body,
            msg:None,
            title: hooks.get("title").unwrap().clone(),
            body: hooks.get("body").unwrap().clone(),
            btns: hooks.get("btns").unwrap().clone()
        };

        {
            let mut locked = self.inner()?;
            (*locked) = Some(inner_dialog);
        }

        Ok(self)
    }

    fn on_btn_click(&self, event:web_sys::MouseEvent, target:Element)->Result<()>{
        log_trace!("dialog on_btn_click:{:?}, target:{:?}", event, target);
        let btn = target.closest("[data-action]")?;
        if let Some(btn) = btn{
            let action = btn.get_attribute("data-action").unwrap();
            match DialogButton::from_str(&action){
                Some(btn)=>{
                    log_trace!("dialog calling callback....");
                    (self.callback.lock()?)(self.clone(), btn)?;
                }
                None=>{
                    //
                }
            }
        }
        
        Ok(())
    }

    pub fn with_callback(self, callback:Callback)->Result<Self>{
        *self.callback.lock()? = callback;
        Ok(self)
    }

    pub fn close(&self)->Result<()>{
        self.hide()?;
        self.remove_from_list()?;
        Ok(())
    }

    fn inner(&self)->LockResult<MutexGuard<Option<DialogInner>>>{
        self.inner.lock()
    }

    pub fn title_container(&self)->Result<Element>{
        Ok(self.inner()?.as_ref().unwrap().title.clone())
    }
    pub fn body_container(&self)->Result<Element>{
        Ok(self.inner()?.as_ref().unwrap().body.clone())
    }
    pub fn btn_container(&self)->Result<Element>{
        Ok(self.inner()?.as_ref().unwrap().btns.clone())
    }

    pub fn change_buttons(&self, left:&[DialogButton], center:&[DialogButton], right:&[DialogButton])->Result<()>{
        let btns = DialogButtons::new(left, center, right);
        let btn_container = self.btn_container()?;
        btn_container.set_inner_html("");
        btns.render_tree()?.inject_into(&btn_container)?;
        Ok(())
    }

    pub fn set_title(&self, title:&str)->Result<()>{
        self.title_container()?.set_inner_html(title);
        Ok(())
    }
    pub fn set_msg(&self, msg:&str)->Result<()>{
        self.body_container()?.set_inner_html(msg);
        Ok(())
    }
    pub fn set_html_msg(&self, msg:Html)->Result<()>{
        let el = self.body_container()?;
        msg.inject_into(&el)?;
        self.inner()?.as_mut().unwrap().msg = Some(msg);
        Ok(())
    }

    fn add_to_list(&self)->Result<()>{
        get_list().insert(self.id.clone(), self.clone());
        Ok(())
    }
    fn remove_from_list(&self)->Result<()>{
        get_list().remove(&self.id);
        if let Some(p) = self.element.parent_element(){
            p.remove_child(&self.element)?;
        }
        Ok(())
    }

    pub fn show(&self)->Result<()>{
        self.element.class_list().add_1("open")?;
        self.add_to_list()?;
        Ok(())
    }
    pub fn hide(&self)->Result<()>{
        self.element.class_list().remove_1("open")?;
        Ok(())
    }

}


fn get_list()->&'static mut BTreeMap<String, Dialog>{
    match unsafe{DIALOGES.as_mut()}{
        Some(list)=>{
            list
        }
        None=>{
            unsafe{DIALOGES = Some(BTreeMap::new());}
            unsafe{DIALOGES.as_mut()}.unwrap()
        }
    }
}

pub fn show_dialog(title:&str, msg:&str)->Result<Dialog>{
    let dialog = Dialog::new()?;
    dialog.set_title(title)?;
    dialog.set_msg(msg)?;
    dialog.show()?;
    Ok(dialog)
}

pub fn show_dialog_with_html(title:&str, msg:Html)->Result<Dialog>{
    let dialog = Dialog::new()?;
    dialog.set_title(title)?;
    dialog.set_html_msg(msg)?;
    dialog.show()?;
    Ok(dialog)
}

pub fn show_error(msg:&str)->Result<Dialog>{
    let dialog = show_dialog(&i18n("Error"), msg)?;
    Ok(dialog)
}
pub fn show_success(msg:&str)->Result<Dialog>{
    let dialog = show_dialog(&i18n("Success"), msg)?;
    Ok(dialog)
}

pub fn show_error_with_html(msg:Html)->Result<Dialog>{
    let dialog = show_dialog_with_html(&i18n("Error"), msg)?;
    Ok(dialog)
}
pub fn show_success_with_html(msg:Html)->Result<Dialog>{
    let dialog = show_dialog_with_html(&i18n("Success"), msg)?;
    Ok(dialog)
}
