use crate::prelude::*;
//use crate::layout::ElementLayout;
//use crate::controls::element_wrapper::ElementWrapper;
//use crate::attributes::Attributes;
//use crate::docs::Docs;
use workflow_ux::result::Result;
use workflow_i18n::i18n;
use workflow_ux::layout::Elemental;
use crate::view::Layout;
//use workflow_html::{html, Render};
//use workflow_ux::form::FormHandlers;

/*
#[wasm_bindgen]
extern "C" {
    // The `FormFooter` class.
    #[wasm_bindgen (extends = BaseElement, js_name = FormFooter , typescript_type = "FormFooter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FormFooterBase;

    #[wasm_bindgen (structural, method, js_class = "FormFooter", js_name = hideButton)]
    pub fn hide_button(this: &FormFooterBase, btn:&str);

    #[wasm_bindgen (structural, method, js_class = "FormFooter", js_name = showButton)]
    pub fn show_button(this: &FormFooterBase, btn:&str);
}
*/


#[derive(Clone)]
pub struct FormFooter{
    pub layout: ElementLayout,
    pub element_wrapper : ElementWrapper,
    on_submit_click_cb:Arc<Mutex<Option<CallbackFn<String>>>>,
    submit_btn: ElementWrapper
}

//unsafe impl Send for FormFooter{}

impl FormFooter {
    pub fn new(
        layout : &ElementLayout,
        _attributes: &Attributes,
        _docs : &Docs
    )->Result<Self>{
        let element = document()
            .create_element("div")?;
        element.class_list().add_1("workflow-form-footer")?;

        let submit_btn = document()
            .create_element("flow-btn")?;
        submit_btn.class_list().add_1("primary")?;
        submit_btn.set_inner_html(&i18n("Submit"));
        element.append_child(&submit_btn)?;

        let pane_inner = layout.inner().ok_or(JsValue::from("unable to mut lock pane inner"))?;
        pane_inner.element.class_list().add_1("with-form-footer")?;
        let mut control = Self{
            layout: layout.clone(),
            element_wrapper: ElementWrapper::new(element),
            on_submit_click_cb: Arc::new(Mutex::new(None)),
            submit_btn: ElementWrapper::new(submit_btn)
        };

        control.init()?;

        Ok(control)
    }

    pub fn element(&self) -> Element {
        self.element_wrapper.element.clone()//.dyn_into::<FormFooterBase>().expect("Unable to cast element to FormFooterBase")
    }

    pub fn set_submit_btn_text<T:Into<String>>(&self, text:T)-> Result<()> {
        self.submit_btn.element.set_inner_html(&text.into());
        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        let cb_opt = self.on_submit_click_cb.clone();
        self.submit_btn.on_click(move|_e|->Result<()>{
            if let Some(cb) =  cb_opt.lock().expect("Unable to lock submit_click_cb").as_mut(){
                return Ok(cb("submit".to_string())?);
            }
            Ok(())
        })?;
        Ok(())
    }

    pub fn on_submit_click(&self, callback:CallbackFn<String>)->Result<()>{
        let mut locked = self.on_submit_click_cb.lock()?;
        *locked = Some(callback);
        Ok(())
    }

    pub fn on_submit<F>(&self, layout:Arc<Mutex<F>>, struct_name:String)
    where 
    F : FormHandler + Elemental + Clone + 'static
    {

        let locked = { layout
            .lock().expect(&format!("Unable to lock form {} for footer submit action.", &struct_name))
            .clone()
        };

        workflow_core::task::wasm::spawn(async move{
            let action = locked.submit();
            action.await
        })
    }



    pub fn bind_layout<F:, D>(&mut self, struct_name:String, view:Arc<Layout<F, D>>)->Result<()>
    where 
    F : FormHandler + Elemental + Send + Clone + 'static,
    D : Send + 'static
    {
        let layout_clone = view.layout();
        let this = self.clone();
        self.submit_btn.on_click(move|_|->Result<()>{
            let struct_name = struct_name.clone();
            let layout = layout_clone.clone();
            this.on_submit(layout, struct_name);
            Ok(())
        })?;

        Ok(())
    }
}
