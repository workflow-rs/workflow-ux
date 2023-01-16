use ahash::AHashMap;
use url::Url;
use workflow_log::{log_error, log_trace, log_warning};
use workflow_ux::error::Error;
use workflow_ux::prelude::*;
use workflow_ux::result::Result;
// use web_sys::{Window,Location};
use crate::window;

#[derive(Clone)]
pub struct Application {
    element: Arc<Element>,
}

static mut APPLICATION: Option<Application> = None;

impl Application {
    pub fn new(el_selector: Option<&str>) -> Result<Application> {
        log_trace!("Creating Workflow Application");

        //console_error_panic_hook::set_once();

        // init framework components
        // TODO fetch language code from localstorage
        workflow_i18n::init_i18n("en")?;
        workflow_ux::icon::init_icon_root("/resources/icons")?;
        workflow_ux::theme::init_theme(crate::theme::Theme::default())?;
        let el_selector = el_selector.unwrap_or("workflow-app");
        let collection = document().query_selector(el_selector)?;
        let element =
            collection.unwrap_or_else(|| panic!("unable to locate '{el_selector}' element"));

        let app = Application {
            element: Arc::new(element),
        };

        unsafe {
            APPLICATION = Some(app.clone());
        }

        // crate::dom::Dom::init();

        Ok(app)
    }
}

impl Application {
    pub fn element(&self) -> Element {
        // Ok((*self.element).clone())
        (*self.element).clone()
    }

    pub fn location(&self) -> Url {
        Url::parse(
            window()
                .location()
                .href()
                .expect("Unable to get application location")
                .as_str(),
        )
        .expect("Unable to parse application location")
    }

    fn load_module(
        &self,
        pkg: &JsValue,
        name: &str,
        module_load_fn_name: &JsValue,
    ) -> Result<JsValue> {
        log_trace!("loading {}", name);
        let fn_jsv = js_sys::Reflect::get(pkg, module_load_fn_name)?;
        let args = js_sys::Array::new();
        // log_trace!("fn_jsv:{:#?}, {:#?}", fn_jsv, args);
        Ok(js_sys::Reflect::apply(&fn_jsv.into(), pkg, &args)?)
    }

    // TODO - replace with internal global registry
    pub async fn load_modules(
        &self,
        pkg: JsValue,
        module_load_order: &[&str],
        module_disable_list: &[&str],
    ) -> Result<JsValue> {
        // log_trace!("with_modules: {:?}", modules);

        let mut modules = AHashMap::<String, (JsValue, Option<String>)>::new(); //Vec::new();
        let keys = js_sys::Reflect::own_keys(&pkg)?;
        let keys_vec = keys.to_vec();
        for key in &keys_vec {
            let name: String = key.as_string().unwrap_or_else(|| "".into());
            if name.starts_with("module_register_") {
                log_trace!("PROCESSING MODULE FN: {}", name);
                let clean_name = name.replace("module_register_", "");
                let mut names = clean_name.split("_wasm"); //.to_lowercase();
                let name = names.next().unwrap();
                let mut depends_on = None;
                if let Some(a) = names.next() {
                    let d = a.replace('_', "");
                    if !d.is_empty() {
                        log_trace!("PROCESSING MODULE {} WHICH DEPENDS ON: {}", name, d);
                        depends_on = Some(d);
                    }
                }

                modules.insert(name.to_string(), (key.clone(), depends_on));
                // modules.push((name, keys_vec[idx].clone()));
            }
        }

        if modules.len() == 0 {
            panic!("workflow_ux::Application::load_modules(): no wasm bindings found!");
        }

        //log_trace!("module_disable_list: {:?}", module_disable_list);

        for name in module_load_order {
            if let Some((module_load_fn_name, depends_on)) = modules.remove(*name) {
                if module_disable_list.contains(name) {
                    log_warning!("skipping disable module {}", name);
                } else if let Some(deps) = depends_on {
                    if module_disable_list.contains(&deps.as_str()) {
                        log_warning!(
                            "skipping module '{}' beacuse it depends on disabled module '{}'",
                            name,
                            deps
                        );
                    } else {
                        self.load_module(&pkg, name, &module_load_fn_name)?;
                    }
                } else {
                    self.load_module(&pkg, name, &module_load_fn_name)?;
                }
            } else {
                log_error!("Unable to load module: {}", name);
            }
        }

        for (name, (module_load_fn_name, depends_on)) in modules.iter() {
            if module_disable_list.contains(&name.as_str()) {
                log_warning!("skipping disable module {}", name);
            } else if let Some(deps) = depends_on {
                if module_disable_list.contains(&deps.as_str()) {
                    log_warning!(
                        "skipping module '{}' beacuse it depends on disabled module '{}'",
                        name,
                        deps
                    );
                } else {
                    self.load_module(&pkg, name, module_load_fn_name)?;
                }
            } else {
                self.load_module(&pkg, name, module_load_fn_name)?;
            }
        }

        crate::module::seal().await?;

        Ok(JsValue::from(true))
    }
}

pub fn global() -> Result<Application> {
    let clone = unsafe {
        APPLICATION
            .as_ref()
            .ok_or(Error::ApplicationGlobalNotInitialized)?
            .clone()
    };
    Ok(clone)
}
