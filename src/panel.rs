use crate::icon::Icon;
use crate::prelude::*;
use workflow_html::{Render, Hooks, Renderables, ElementResult};
use crate::result::Result;
use crate::controls::listener::Listener;
use web_sys::Element;

#[derive(Clone, Debug, Default)]
pub struct OptString(Option<String>);

impl OptString{
    pub fn value(&self)->&Option<String>{
        &self.0
    }
}

impl From<String> for OptString{
    fn from(str: String) -> Self {
        Self(Some(str))
    }
}
impl From<&str> for OptString{
    fn from(str: &str) -> Self {
        Self(Some(str.to_string()))
    }
}

#[derive(Clone, Debug, Default)]
pub struct OptBool(Option<bool>);

impl OptBool{
    pub fn value(&self)->&Option<bool>{
        &self.0
    }

    pub fn is_true(&self)->bool{
        if let Some(b) = self.0{
            return b;
        }
        false
    }
}

impl From<String> for OptBool{
    fn from(str: String) -> Self {
        Self(Some(str.eq("true")))
    }
}

impl From<bool> for OptBool{
    fn from(b: bool) -> Self {
        Self(Some(b))
    }
}


#[derive(Clone, Debug, Default)]
pub struct InfoRow{
    pub title: String,
    pub cls: OptString,
    pub sub: OptString,
    pub value: OptString,
    pub editable: OptBool,
    pub deletable: OptBool,
    pub left_icon: OptString,
    pub right_icon: OptString,
    pub right_icon_el: Arc<Mutex<Option<Element>>>,
    pub right_icon_click_listener: Option<Listener<web_sys::MouseEvent>>
}

impl InfoRow{
    pub fn on(mut self, event:&str, cb: Box<dyn Fn(&InfoRow) -> Result<()>>)-> Self{
        log_trace!("InfoRow.on() => event: {}", event);
        let this = self.clone();
        self.right_icon_click_listener = Some(Listener::new(move|_e|->Result<()>{
            cb(&this)?;
            Ok(())
        }));

        self
    }

    pub fn render_el(&mut self)->ElementResult<Element>{
        let info_row_el = create_el("div", vec![("class", "info-row")], None)?;

        let title_el = create_el("div", vec![("class", "title")], Some(&self.title))?;
        let title_box_el = create_el("div", vec![("class", "title-box")], None)?;
        title_box_el.append_child(&title_el)?;

        if let Some(sub_title) = self.sub.value(){
            let el = create_el("div", vec![("class", "sub-title")], Some(sub_title))?;
            title_box_el.append_child(&el)?;
        }else{
            //info_row_el.class_list().add_1("no-sub")?;
        }
        
        
        if let Some(icon) = self.left_icon.value(){
            let el = create_el("img", vec![("class", "icon left"), ("src", icon)], None)?;
            info_row_el.append_child(&el)?;
        }else{
            //info_row_el.class_list().add_1("no-left-icon")?;
        }

        info_row_el.append_child(&title_box_el)?;

        if let Some(value) = self.value.value(){
            let el = create_el("div", vec![("class", "value")], Some(value))?;
            info_row_el.append_child(&el)?;
        }

        if self.editable.is_true(){
            let el = Icon::css("info-row-edit").element()?;
            el.set_attribute("data-action", "edit")?;
            info_row_el.append_child(&el)?;
        }

        if let Some(icon) = self.right_icon.value(){
            let el = create_el("img", vec![("class", "icon right"), ("src", icon)], None)?;
            info_row_el.append_child(&el)?;

            //let element_wrapper=ElementWrapper::new(el);
            if let Some(listener) = &self.right_icon_click_listener{
                /*
                element_wrapper.on_click(move|_e|->Result<()>{
                    cb(&this)?;
                    Ok(())
                })?;
                */
                el.add_event_listener_with_callback("click", listener.into_js())?;
            }
            self.right_icon_el = Arc::new(Mutex::new(Some(el)));
        }else{
            //info_row_el.class_list().add_1("no-right-icon")?;
        }

        if let Some(cls) = self.cls.value(){
            info_row_el.class_list().add_1(cls)?;
        }

        Ok(info_row_el)
    }

}


impl Render for InfoRow{
    /*
    fn on(&mut self, event:&str, cb: Box<dyn Fn(dyn Render) -> ElementResult<()>>){
        log_trace!("InfoRow.on() => event: {}", event);
        let this = self.clone();
        self.right_icon_click_listener = Some(Listener::new(move|_e|->Result<()>{
            cb(this)?;
            Ok(())
        }));

        //self
    }
    */
    fn render(&self, _w: &mut Vec<String>) -> ElementResult<()>{
        //let attr = self.get_attributes();
        //let children = self.get_children();
        //w.push(format!("<flow-menu-item text=\"{}\" value=\"{}\">{}</flow-menu-item>", self.text,  self.value, self.html));
        Ok(())
    }

    fn render_node(
        mut self,
        parent:&mut Element,
        _map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>{
        let info_row_el = self.render_el()?;
        parent.append_child(&info_row_el)?;
        renderables.push(Arc::new(self));
        Ok(())
    }
    
}

