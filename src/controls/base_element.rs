use crate::prelude::*;
use crate::result::Result;

#[wasm_bindgen]
extern "C" {
    /*
    # [wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = ::js_sys::Object , js_name = Element , typescript_type = "Element")]
    // "The `ExtendedElement` class.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ExtendedElement;

    # [wasm_bindgen (method , structural , js_class = "Element" , js_name = scrollTo)]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_to(this: &ExtendedElement, opt:&js_sys::Object);
    */

    # [wasm_bindgen (extends = Element , extends = Node , extends = EventTarget , extends = ::js_sys::Object , js_name = BaseElement , typescript_type = "BaseElement")]
    // "The `BaseElement` class.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type BaseElement;

    # [wasm_bindgen (structural , method , js_class = "BaseElement" , js_name = focus)]
    pub fn focus(this: &BaseElement);

    // The `closest_form_control()` method.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)"]
    // *This API requires the following crate features to be activated: `Element`*
    # [wasm_bindgen (catch , method , structural , js_class = "BaseElement" , js_name = closest)]
    pub fn _closest_form_control(this: &BaseElement, selector: &str) -> Result<Option<FormControlBase>>;
}

impl BaseElement{
    pub fn show(&self, show:bool)->Result<()>{
        if show{
            self.remove_attribute("hide")?;
        }else{
            self.set_attribute("hide", "true")?;
        }

        Ok(())
    }
    pub fn show_form_control(&self, show:bool)->Result<()>{
        if let Some(ct) = self.closest_form_control()?{
            if show{
                ct.remove_attribute("hide")?;
            }else{
                ct.set_attribute("hide", "true")?;
            }
        }

        Ok(())
    }
    pub fn closest_form_control(&self)->Result<Option<FormControlBase>>{
        self._closest_form_control("flow-form-control".into())
    }

    pub fn focus_form_control(&self)->Result<()>{
        let r = self.closest_form_control()?;
        if let Some(form_control) = r {
            form_control.scroll_into_view();
            form_control.focus();
        }else{
            self.scroll_into_view();
        }

        Ok(())
    }
}
