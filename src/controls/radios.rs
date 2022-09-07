use crate::prelude::*;
use workflow_ux::result::Result;


#[wasm_bindgen]
extern "C" {
    // The `FlowRadiosBase` class.
    #[wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = FlowRadios , typescript_type = "FlowRadios")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowRadiosBase;

    // Getter for the `value` field of this object.
    #[wasm_bindgen (structural , method , getter , js_class = "FlowRadiosBase" , js_name = value)]
    pub fn value(this: &FlowRadiosBase) -> String;
}


#[derive(Clone)]
pub struct Radios<E> {
    element_wrapper : ElementWrapper,
    value : Rc<RefCell<String>>,
    change_callback : OptionalCallback<String>,
    p:PhantomData<E>
}

impl<E> Radios<E>
where E: EnumTrait<E> + 'static + Display
{
    
    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()
    }

    pub fn new(layout : &ElementLayout, attributes: &Attributes, _docs : &Docs) -> Result<Radios<E>> {
        let doc = document();
        let element = doc
            .create_element("flow-radios")?;

        let mut init_value: String = String::new();
        let items = E::list();
        for item in items.iter() {
            let radio = doc.create_element("flow-radio")?;
            radio.set_attribute("inputvalue", item.as_str())?;
            radio.set_inner_html(item.descr());
            element.append_child(&radio)?;
            if init_value.eq(""){
                init_value = String::from(item.as_str())
            }
        }  

        for (k,v) in attributes.iter() {
            if k.eq("value"){
                init_value = v.to_string();
            }else{
                element.set_attribute(k,v)?;
            }
        }

        element.set_attribute("selected", init_value.as_str())?;
        
        let value = Rc::new(RefCell::new(init_value.clone()));

        let pane_inner = layout
            .inner()
            .ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.append_child(&element)?;


        let mut radios = Radios {
            element_wrapper: ElementWrapper::new(element),
            value,
            change_callback:Rc::new(RefCell::new(None)),
            p:PhantomData
        };
        radios.init()?;
    
        Ok(radios)
    }

    fn init(&mut self)-> Result<()>{

        let element = self.element();
        let el = element.clone().dyn_into::<FlowRadiosBase>().expect("Unable to cast to FlowRadioBase");
        let value = self.value.clone();
        let calback = self.change_callback.clone();
        self.element_wrapper.on("select", move |event| -> Result<()>{
            
            log_trace!("flow radio changed event: {:?}", event);
            let detail = event.detail();
            log_trace!("flow radio changed event detail: {:?}", detail);

            let new_value = el.value();

            log_trace!("flow radio: new value: {:?}", new_value);
            //if let Some(op) = E::from_str(current_element_value.as_str()){
            //    log_trace!("op: {}", op);
            //}

            *value.borrow_mut() = new_value.clone();
        
            if let Some(cb) = calback.borrow_mut().as_mut(){
                cb(new_value)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn value(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn set_value(&mut self, value:String)->Result<()>{
        self.element_wrapper.element.set_attribute("selected", value.as_str())?;
        *self.value.borrow_mut() = value;
        Ok(())
    }

    pub fn on_change(&self, callback:Callback<String>)->Result<()>{
        *self.change_callback.borrow_mut() = Some(callback);
        Ok(())
    }
}