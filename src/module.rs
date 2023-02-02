// use std::any::Any;
// use std::{sync::Arc, any::TypeId};

use ahash::AHashMap;
use derivative::Derivative;
use std::sync::RwLock;
use workflow_async_trait::async_trait_without_send;
use workflow_ux::error::Error;
use workflow_ux::prelude::*;
use workflow_ux::result::Result;

use downcast::{downcast_sync, AnySync};

#[async_trait_without_send]
pub trait ModuleInterface: AnySync {
    // fn menu(self : Arc<Self>) -> Option<MenuGroup> { None }
    async fn main(self: Arc<Self>) -> Result<()> {
        Ok(())
    }
    async fn load(self: Arc<Self>) -> Result<()> {
        Ok(())
    }

    async fn evict(
        self: Arc<Self>,
        _container: &Arc<view::Container>,
        _view: Arc<dyn view::View>,
    ) -> Result<()> {
        Ok(())
    }

    // TODO - generate and inject HTML into the render view
    // async fn render(self : Arc<Self>, _account : &AccountDataReference) -> Result<()> { Ok(()) }

    // fn type_id(self : Arc<Self>) -> Option<TypeId> { None }
}
downcast_sync!(dyn ModuleInterface);
// unsafe impl Send for dyn ModuleInterface {}

static mut MODULES: Option<Arc<RwLock<AHashMap<String, Arc<Module>>>>> = None;
static mut DATA_TYPES_TO_MODULES: Option<Rc<RefCell<AHashMap<u32, Arc<Module>>>>> = None;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub container_types: Vec<u32>,
    #[derivative(Debug = "ignore")]
    pub iface: Arc<dyn ModuleInterface>,
    // pub iface : Arc<Box<dyn Any>>, //dyn ModuleInterface>,
    // pub iface : Arc<dyn Any>, //dyn ModuleInterface>,
    // pub iface : Arc<Box<dyn Any>> //Arc<dyn Any>, //dyn ModuleInterface>,
    // pub iface : Box<Any>, //dyn ModuleInterface>,
}
impl Module {
    // pub fn new(name : &str, iface : Arc<dyn Any>, container_types : &[u32]) -> Module
    pub fn new(name: &str, iface: Arc<dyn ModuleInterface>, container_types: &[u32]) -> Module
// pub fn new<T>(name : &str, iface : Arc<dyn ModuleInterface>, container_types : &[u32]) -> Module 
    // pub fn new<T>(name : &str, iface : Arc<dyn ModuleInterface>, container_types : &[u32]) -> Module 
    // pub fn new<T>(name : &str, iface : Arc<Box<&T>>, container_types : &[u32]) -> Module 
    // pub fn new<T>(name : &str, iface : &T, container_types : &[u32]) -> Module 
    // where T : ModuleInterface + Any + ?Sized
    {
        Module {
            name: name.to_string(),
            container_types: container_types.to_vec(),
            iface,
            // iface : Arc::new(iface),
        }
    }

    // pub fn dyn_into<T>(&self) {

    //     // TODO - return a caster iface to the desired type

    // }

    pub async fn main(self: Arc<Self>) -> Result<()> {
        log_trace!("▷ {}::main()", self.name);
        self.iface.clone().main().await
        // self.iface.clone().downcast_ref::<dyn ModuleInterface>()?.main().await
    }

    pub async fn load(self: Arc<Self>) -> Result<()> {
        log_trace!("▷ {}::load()", self.name);
        self.iface.clone().load().await
    }
}

pub fn registry() -> Arc<RwLock<AHashMap<String, Arc<Module>>>> {
    let modules = unsafe { &MODULES };
    if let Some(modules) = modules {
        modules.clone()
    } else {
        let modules = Arc::new(RwLock::new(AHashMap::new()));
        unsafe {
            MODULES = Some(modules.clone());
        }
        modules
    }
}

pub async fn register(
    name: &str,
    iface: Arc<dyn ModuleInterface>,
    container_types: &[u32],
) -> Result<()> {
    // let module = Arc::new(RwLock::new(Module::new(name,iface.clone(),container_types)));
    let module = Arc::new(Module::new(name, iface.clone(), container_types));
    if registry()
        .write()?
        .insert(name.to_string(), module.clone())
        .is_some()
    {
        panic!(
            "Error: multiple registrations for module {}.  Modules are singletons.",
            name
        );
    }
    log_trace!("* * * * * * * * * registering module: {}", name);
    // let mut iface = iface.clone().write().await;
    // iface.load().await?;
    iface.clone().main().await?;
    Ok(())
}

pub fn data_types_to_modules() -> Result<Rc<RefCell<AHashMap<u32, Arc<Module>>>>> {
    let map = unsafe { DATA_TYPES_TO_MODULES.clone() };
    match map {
        Some(map) => Ok(map),
        None => Err(Error::DataTypesToModuleMapMissing),
    }
}

pub async fn seal() -> Result<()> {
    let data_types_to_modules = Rc::new(RefCell::new(AHashMap::new()));
    {
        let mut data_types_to_modules = data_types_to_modules.borrow_mut();
        for (_, module) in registry().read()?.iter() {
            for container_type in module.container_types.iter() {
                data_types_to_modules.insert(*container_type, module.clone());
            }
        }
    }
    unsafe { DATA_TYPES_TO_MODULES = Some(data_types_to_modules) }
    Ok(())
}

pub fn get_module(name: &str) -> Option<Arc<Module>> {
    registry()
        .read()
        .unwrap_or_else(|_| panic!("Unable to locate module {} (registry rwlock failure)", name))
        .get(name)
        .cloned()
}

pub fn get_interface<T>(name: &str) -> Option<Arc<T>>
// pub fn get<T>() -> Result<Option<Arc<T>>>
where
    T: Send + Sync + 'static,
{
    // let name = stringify!(T);

    log_debug!("SEARCHING FOR MODULE: {}", name);

    match registry()
        .read()
        .expect("Unable to lock module registry")
        .get(name)
    {
        Some(module) => {
            log_debug!("MODULE FOUND!");

            let iface = module
                .iface
                .clone()
                .downcast_arc::<T>()
                .unwrap_or_else(|_| panic!("Unable to downcast module to T: {}", name));
            // .downcast_arc::<T>()
            // .map_err(|err|error!("Unable to downcast module {} {}", name,err))?;

            Some(iface)
        }
        None => {
            log_trace!("MODULE ***NOT*** FOUND!");
            None
        }
    }
}

pub fn get_from_container_type(container_type: &u32) -> Result<Option<Arc<Module>>> {
    let data_types_to_modules = data_types_to_modules()?;

    let module = data_types_to_modules.borrow().get(container_type).cloned();

    Ok(module)
}

// pub async fn render_container(account : &AccountDataReference) -> Result<()> {
//     let (pubkey, container_type) = {
//         let account = account.read().await;
//         (account.key, account.container_type())
//     };
//      log_trace!("render_container: {} -> {:?}", pubkey, container_type);
//     // let container_type = account.container_type();
//     match container_type {
//         Some(container_type) if container_type != 0 => {
//             let module = get_from_container_type(&container_type)?;
//             match module {
//                 Some(module) => {
//                     // module.render(account).await?;
//                     module.load().await?;
//                 },
//                 None => {
//                     return Err(error!("Unregistered container type: {} - must be registered with declare_module!()", container_type));
//                 }
//             }
//         },
//         Some(_) | None => {
//             return Err(error!("Unable to obtain container type for account {}", pubkey));
//         }
//     }
//     Ok(())
// }
