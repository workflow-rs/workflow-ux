use std::{sync::{Arc, Mutex, MutexGuard, LockResult}, collections::BTreeMap};
use crate::prelude::*;
use crate::result::Result;
use workflow_html::{html, Render, Html};
use workflow_core::id::Id;

static mut DIALOGES : Option<BTreeMap<String, Dialog>> = None;


pub struct DialogInner{
    msg:Option<Html>,
    _body:Html,
    title_el:Element,
    body_el:Element,
    ok_btn_el:Element
}

#[derive(Clone)]
pub struct Dialog{
    id:Id,
    element: Element,
    inner: Arc<Mutex<Option<DialogInner>>>,
}

impl Dialog{
    fn new()->Result<Self>{
        Ok(Self {
            id: Id::new(),
            element: create_el("div.workflow-dialog", vec![], None)?,
            inner: Arc::new(Mutex::new(None))
        }.init()?)
    }

    fn init(self)->Result<Self>{

        let this = self.clone();
        let body = html!{
            <div class="workflow-dialog-inner">
                <h2 class="title" @title></h2>
                <div class="body" @body>

                </div>
                <div class="actions">
                    <flow-btn class="primary" @ok_btn
                        !click={this.on_ok_click().unwrap_or(());}
                    >{i18n("Ok")}</flow-btn>
                </div>
            </div>
        }?;

        body.inject_into(&self.element)?;

        document().body().unwrap().append_child(&self.element)?;

        let hooks = body.hooks().clone();
        let inner_dialog = DialogInner{
            _body:body,
            msg:None,
            title_el: hooks.get("title").unwrap().clone(),
            body_el: hooks.get("body").unwrap().clone(),
            ok_btn_el: hooks.get("ok_btn").unwrap().clone()
        };

        {
            let mut locked = self.inner()?;
            (*locked) = Some(inner_dialog);
        }

        Ok(self)
    }

    pub fn on_ok_click(&self)->Result<()>{
        self.hide()?;
        self.remove_from_list()?;
        Ok(())
    }

    fn inner(&self)->LockResult<MutexGuard<Option<DialogInner>>>{
        self.inner.lock()
    }

    pub fn title_el(&self)->Result<Element>{
        Ok(self.inner()?.as_ref().unwrap().title_el.clone())
    }
    pub fn body_el(&self)->Result<Element>{
        Ok(self.inner()?.as_ref().unwrap().body_el.clone())
    }
    pub fn ok_btn_el(&self)->Result<Element>{
        Ok(self.inner()?.as_ref().unwrap().ok_btn_el.clone())
    }

    pub fn set_title(&self, title:&str)->Result<()>{
        self.title_el()?.set_inner_html(title);
        Ok(())
    }
    pub fn set_msg(&self, msg:&str)->Result<()>{
        self.body_el()?.set_inner_html(msg);
        Ok(())
    }
    pub fn set_html_msg(&self, msg:Html)->Result<()>{
        let el = self.body_el()?;
        msg.inject_into(&el)?;
        self.inner()?.as_mut().unwrap().msg = Some(msg);
        Ok(())
    }

    fn add_to_list(&self)->Result<()>{
        get_list().insert(format!("dialog_{}", self.id), self.clone());
        Ok(())
    }
    fn remove_from_list(&self)->Result<()>{
        get_list().insert(format!("dialog_{}", self.id), self.clone());
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
