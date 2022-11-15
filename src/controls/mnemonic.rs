use crate::prelude::*;
use crate::result::Result;
use crate::error::Error;
use crate::controls::listener::Listener;
use workflow_html::{Html, Render, html};

pub static CSS:&'static str = include_str!("mnemonic.css");


#[derive(Clone)]
pub struct Mnemonic {
    pub layout : ElementLayout,
    pub attributes: Attributes,
    pub element_wrapper : ElementWrapper,
    words_el: ElementWrapper,

    #[allow(dead_code)]
    body: Arc<Html>,
    inputs:Vec<HtmlInputElement>,
    value : Arc<Mutex<String>>,
    on_change_cb:Arc<Mutex<Option<Callback<String>>>>,
}

impl Mnemonic {

    pub fn set_heading(&self, value:&str)->Result<()>{
        self.element_wrapper.element.set_attribute("label", value)?;
        Ok(())
    }

    pub fn show(&self)->Result<()>{
        self.element_wrapper.element.remove_attribute("hidden")?;
        Ok(())
    }

    pub fn hide(&self)->Result<()>{
        self.element_wrapper.element.set_attribute("hidden", "true")?;
        Ok(())
    }

    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn new(
        layout : &ElementLayout,
        attributes: &Attributes,
        docs : &Docs
    ) -> Result<Self> {
        let element = document()
            .create_element("div")?;

        //let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        //pane_inner.element.append_child(&element)?;
    
        Ok(Self::create(element, layout.clone(), attributes, docs, String::from(""))?)
    }

    fn create(
        element: Element,
        layout : ElementLayout,
        attributes: &Attributes,
        _docs : &Docs,
        mut init_value: String
    ) -> Result<Self> {
        let msg = "Enter 24-word seed phrase".to_string();
        let heading = attributes.get("heading").unwrap_or(&msg);
        let body = html!{
			<p class="heading" @heading>
				{i18n(heading)}
			</p>
			<div class="words" @words>
				{
                let mut list = Vec::new();
                for index in 0..24{
                    list.push(html!{
                        <div class="cell">
                            <input class="seed word"
                                data-index={format!("{}", index)} />
                        </div>
                    }?);
                }
                list
				}
			</div>
			<div class="error" @error></div>
        }?;

        element.class_list().add_1("mnemonic-input")?;

        let mut inputs = vec![];
        let hooks =  body.hooks();

        //let first_input = hooks.get("first_input").unwrap().clone();

        let words_el = hooks.get("words").unwrap().clone();
        let input_nodes = words_el.query_selector_all("input.seed")?;
        let len = input_nodes.length();
        for index in 0..len{
            if let Some(node) = input_nodes.get(index){
                let input = node.dyn_into::<HtmlInputElement>().unwrap();
                inputs.push(input);
            }
        }

        body.inject_into(&element)?;

        //element.set_attribute("value", init_value.as_str())?;
        element.set_attribute("tab-index","0")?;

        for (k,v) in attributes.iter() {
            element.set_attribute(k,v)?;
            if k.eq("value"){
                init_value = v.to_string();
            }
        }
        let value = Arc::new(Mutex::new(init_value));

        let mut control = Self {
            layout,
            attributes:attributes.clone(),
            element_wrapper: ElementWrapper::new(element),
            words_el:ElementWrapper::new(words_el),
            //first_input: ElementWrapper::new(first_input),
            value,
            body:Arc::new(body),
            inputs,
            on_change_cb:Arc::new(Mutex::new(None))
        };

        control.init()?;

        Ok(control)
    }

    pub fn value(&self) -> String {
        (*self.value.lock().unwrap()).clone()
    }

    fn apply_value(&self, value:&String)->Result<Vec<String>>{
        let words:Vec<String> = value
            .replace("\t", " ")
            .replace("\n", " ")
            .replace("\r", " ")
            .replace("'", "")
            .replace("\"", "")
            .split(" ").map(|word|{
                word.trim().to_string()
            }).filter(|word|{
                word.len() > 0
            }).collect();

        //log_trace!("words: {:?}", words);

        if words.len() < 24{
            return Ok(words);
        }
        for index in 0..24{
            if let Some(input) = self.inputs.get(index){
                input.set_value(&words[index])
            }
        }

        Ok(words)
    }

    pub fn set_value<T: Into<String>>(&self, value:T)->Result<()>{
        let value:String = value.into();
        let words = self.apply_value(&value)?;
        //FieldHelper::set_value_attr(&self.element_wrapper.element, &value)?;
        *self.value.lock().unwrap() = words.join(" ");
        Ok(())
    }

    pub fn mark_invalid(&self, invalid:bool)->Result<()>{
        self.element().class_list().toggle_with_force("invalid", invalid)?;
        Ok(())
    }
    

    pub fn init(&mut self)-> Result<()>{
        {
            let this = self.clone();
            let listener = Listener::new(move |event:web_sys::CustomEvent| ->Result<()> {
                this.on_input_change(event)?;
                Ok(())
            });
            self.words_el.element.add_event_listener_with_callback("change", listener.into_js())?;
            self.words_el.element.add_event_listener_with_callback("keyup", listener.into_js())?;
            self.words_el.element.add_event_listener_with_callback("keydown", listener.into_js())?;
            self.words_el.push_listener(listener);
        }

        Ok(())
    }

    fn on_input_change(&self, event:CustomEvent)->Result<()>{
        //log_trace!("received change event: {:?}", event);
        let target = match event.target(){
            Some(t)=>t,
            None=>return Ok(())
        };
        let el = match target.dyn_into::<Element>(){
            Ok(t)=>t,
            Err(_)=>return Ok(())
        };
        let input_el = match el.closest("input")?{
            Some(t)=>t,
            None=>return Ok(())
        };
        let input = input_el.dyn_into::<HtmlInputElement>()?;
        let index:u32 = match input.get_attribute("data-index"){
            Some(index) => index.parse()?,
            None => {
                return Ok(());
            }
        };

        if index > 23{
            return Ok(());
        }
        let mut input_value = input.value();
        let mut remove_space = true;
        if index == 0{
            let words = self.apply_value(&input_value)?;
            remove_space =  words.len() != 24;
        }

        input_value = input_value
            .replace("\t", " ")
            .replace("\n", " ")
            .replace("\r", " ");

        if remove_space && input_value.contains(" "){
            input.set_value(input_value.split(" ").next().unwrap())
        }

        let mut values = vec![];
        for input in &self.inputs{
            values.push(input.value());
        }

        let new_value = values.join(" ");

        let mut value = self.value.lock().unwrap();
        *value = new_value.clone();

        if let Some(cb) =  self.on_change_cb.lock().unwrap().as_mut(){
            return Ok(cb(new_value)?);
        }

        Ok(())

    }

    pub fn on_change(&self, callback:Callback<String>){
        *self.on_change_cb.lock().unwrap() = Some(callback);
    }
}


impl<'refs> TryFrom<ElementBindingContext<'refs>> for Mnemonic {
    type Error = Error;

    fn try_from(ctx : ElementBindingContext<'refs>) -> Result<Self> {
        Ok(Self::create(
            ctx.element.clone(),
            ctx.layout.clone(),
            &ctx.attributes,
            &ctx.docs,
            String::new()
        )?)

    }
}