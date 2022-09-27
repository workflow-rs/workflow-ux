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
        OptString(Some(str))
    }
}


#[derive(Clone, Debug, Default)]
pub struct InfoRow{
    pub title: String,
    pub cls: OptString,
    pub sub: OptString,
    pub value: OptString,
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
        /*
        {
            let mut locked = self.right_icon_el.lock().expect("Unable to lock right_icon_el");
            if let Some(el) = locked.as_mut(){
                log_trace!("InfoRow.on() => event22222: {}", event);
                let this = self.clone();
                match el.on_click(move|_e|->Result<()>{
                    cb(&this)?;
                    Ok(())
                }){
                    Ok(_)=>{

                    }
                    Err(e)=>{
                        panic!("Unable to bind InfoRow.on({}), e:{:?}", event, e);
                    }
                }
            }
        }
        */

        self
    }
}


impl Render for InfoRow{
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

        parent.append_child(&info_row_el)?;
        renderables.push(Arc::new(self));
        Ok(())
    }

}
