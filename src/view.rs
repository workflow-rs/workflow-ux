use std::{sync::{Arc, Mutex}, any::TypeId};

use workflow_ux::prelude::*;

use workflow_ux::layout;
// use crate::{prelude::*, application};
use workflow_ux::result::Result;
use workflow_log::log_trace;
// pub enum Eviction {
//     Allow,
//     Disallow,
// }



#[derive(Clone)]
pub struct Container {
    element: Element,
    view : Arc<RwLock<Option<Arc<dyn View>>>>,
}

impl Container {
    pub fn new(element: Element) -> Self {
        Container {
            element,
            view : Arc::new(RwLock::new(None))
        }
    }

    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub async fn swap(self : &Arc<Self>, incoming : Arc<dyn View>) -> Result<Option<Arc<dyn View>>> {
        let from = self.swap_from().await?;
        self.swap_to(incoming).await?;
        Ok(from)
    }

    /// Initiates view swap.  Must be called before creating any objects for
    /// the next view.  This function checks if the current view can be
    /// safely evicted allowing the owning module to query user for confirmation
    /// if necessary.
    pub async fn swap_from(self : &Arc<Self>) -> Result<Option<Arc<dyn View>>> {
        let previous = self.view.read().await.clone();
        match &previous {
            None => { 
                log_trace!("swap_from(): there is no previous view");
                Ok(None) 
            },
            Some(previous) => {
                let module = previous.module();
                // TODO query module for view eviction etc.
                module.evict(self, previous.clone()).await?;
                previous.clone().evict().await?;
                log_trace!("swap_from(): finishing...");
                Ok(Some(previous.clone()))
            }
        }
    }

    /// Executes the swap, evicting the previous view and installing the new one.
    /// Currently this is done by simply replacing children.
    /// TODO: implement transition between views
    pub async fn swap_to(self : &Arc<Self>, incoming : Arc<dyn View>) -> Result<()> {
        
        let previous = self.view.read().await.clone();
        *self.view.write().await = Some(incoming.clone());

        if let Some(previous) = previous {
            let el = previous.element();
            self.element.remove_child(&el)?;
        }
        
        bottom_menu::update_menus(incoming.bottom_menus())?;

        self.element.append_child(&incoming.element())?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn view(&self) -> Option<Arc<dyn View>> {
        self.view.read().await.clone()
    }
}

impl Into<Element> for Container {
    fn into(self) -> Element {
        self.element.clone()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



#[async_trait(?Send)]
pub trait View : Sync + Send {
    fn element(&self) -> Element;
    //  {
    //     self.element.clone()
    // }

    fn module(&self) -> Arc<dyn ModuleInterface>;

    fn typeid(&self) -> TypeId;

    // fn eviction_dis(&self) -> Eviction { Eviction::Allow }

    async fn evict(self : Arc<Self>) -> Result<()> { Ok(()) }
    // async fn evict2(&mut self) -> Result<()> { Ok(()) }
    // fn cleanup(&self) -> Result<()> { Ok(()) }

    fn drop(&self) { }

    fn bottom_menus(&self)->Option<Vec<bottom_menu::BottomMenuItem>>{
        None
    }

    // fn drop(&self) -> Result<()> { Ok(()) }
    //  {
    //     self.module.clone()
    // }
}

// unsafe impl Sync for dyn View {} 

pub struct Default {
    element : Element,
    module : Arc<dyn ModuleInterface>
}

unsafe impl Send for Default { }
unsafe impl Sync for Default { }

impl Default {
    pub fn try_new(module : Arc<dyn ModuleInterface>) -> Result<Arc<dyn View>> {
        let view = Default { 
            element : document().create_element("workspace-view")?,
            module
        };
        Ok(Arc::new(view))
    }
}

impl View for Default {
    fn element(&self) -> Element {
        self.element.clone()
    }

    fn module(&self) -> Arc<dyn ModuleInterface> {
        self.module.clone()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}


pub struct Data<D> {
    data : Arc<Mutex<D>>,
    element : Element,
    module : Arc<dyn ModuleInterface>, 
}

impl<D> Data<D> {
    // pub fn try_new(module : Arc<dyn ModuleInterface>) -> Result<Arc<dyn View>> {
    pub fn try_new(module : Arc<dyn ModuleInterface>, data : D) -> Result<Arc<Data<D>>> {
        let view = Data::<D> { 
            element : document().create_element("workspace-view")?,
            module,
            data : Arc::new(Mutex::new(data)),
        };
        Ok(Arc::new(view))
    }

    pub fn data(&self) -> Arc<Mutex<D>> {
        self.data.clone()
    }
}

unsafe impl<D> Send for Data<D> { }
unsafe impl<D> Sync for Data<D> { }

impl<D> View for Data<D> 
where D : 'static
{
    fn element(&self) -> Element {
        self.element.clone()
    }

    fn module(&self) -> Arc<dyn ModuleInterface> {
        self.module.clone()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Data<D>>()
    }
}


//     pub fn callback(self, callback: Box<dyn Fn() -> Result<()>>) -> Result<Self> {


type EvictFn = Box<dyn Fn() -> Result<()>>;
type DropFn = Box<dyn Fn()>;
// type EvictFn = Box<dyn Fn()>;

pub struct Layout<F,D> {
    layout : Arc<Mutex<F>>,
    data : Arc<Mutex<Option<D>>>,
    evict : Arc<Mutex<Option<EvictFn>>>,
    drop : Arc<Mutex<Option<DropFn>>>,
    element : Element,
    module : Arc<dyn ModuleInterface>, 
}

impl<F,D> Layout<F,D> 
where 
    F : layout::Elemental + 'static,
    D : 'static
{
    // pub fn try_new(module : Arc<dyn ModuleInterface>) -> Result<Arc<dyn View>> {
    pub fn try_new(module : Arc<dyn ModuleInterface>, layout : F, data : Option<D>) -> Result<Arc<Layout<F,D>>> {

        let element = document().create_element("workspace-view")?;
        element.append_child(&layout.element())?;

        let view = Layout::<F,D> { 
            element,
            module,
            layout : Arc::new(Mutex::new(layout)),
            data : Arc::new(Mutex::new(data)),
            // evict : Arc::new(Mutex::new(evict)),
            evict : Arc::new(Mutex::new(None)),
            drop : Arc::new(Mutex::new(None)),
        };
        Ok(Arc::new(view))
    }

    pub fn with_evict_handler(self: Arc<Self>, handler: EvictFn) -> Result<Arc<Self>> {
        *self.evict.lock().unwrap() = Some(handler);
        Ok(self)
    }

    pub fn with_drop_handler(self: Arc<Self>, handler: DropFn) -> Result<Arc<Self>> {
        *self.drop.lock().unwrap() = Some(handler);
        Ok(self)
    }

    pub fn layout(&self) -> Arc<Mutex<F>> {
        self.layout.clone()
    }

    pub fn has_data(&self) -> Result<bool> {
        Ok(self.data.lock()?.is_some())
    }

    pub fn data(&self) -> Arc<Mutex<Option<D>>> {
        self.data.clone()
    }

}

unsafe impl<F,D> Send for Layout<F,D> { }
unsafe impl<F,D> Sync for Layout<F,D> { }


#[async_trait(?Send)]
impl<F,D> View for Layout<F,D> 
where 
    F : layout::Elemental + 'static,
    D : 'static
{
    fn element(&self) -> Element {
        self.element.clone()
    }

    fn module(&self) -> Arc<dyn ModuleInterface> {
        self.module.clone()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Data<F>>()
    }

    // async fn evict(self : Arc<Layout<F,D>>) -> Result<()> {
    async fn evict(self : Arc<Layout<F,D>>) -> Result<()> {
        let evict = self.evict.lock()?;
        match &*evict {
            Some(evict) => {
                Ok(evict()?)
            },
            None => {
                Ok(())
            }

        }
    }
}

impl<F,D> Drop for Layout<F,D> 
{
    fn drop(&mut self) {

        let drop = self.drop.lock().unwrap();
        match &*drop {
            Some(drop) => {
                drop();
            },
            None => { }
        }
    }
}


pub struct Html {
    element : Element,
    module : Arc<dyn ModuleInterface>,
    _html: workflow_html::Html,
    menus:Option<Vec<bottom_menu::BottomMenuItem>>
}

impl Html {
    pub fn try_new(
        module : Arc<dyn ModuleInterface>,
        html : workflow_html::Html, //&(Vec<Element>, BTreeMap<String, Element>),
    ) -> Result<Arc<dyn View>> {
        let view = Self::create(module, html, None)?;
        Ok(Arc::new(view))
    }

    pub fn try_new_with_menus(
        module : Arc<dyn ModuleInterface>,
        html : workflow_html::Html,
        menus:Vec<bottom_menu::BottomMenuItem>
    )-> Result<Arc<dyn View>> {
        let view = Self::create(module, html, Some(menus))?;
        Ok(Arc::new(view))
    }

    pub fn create(
        module : Arc<dyn ModuleInterface>,
        html : workflow_html::Html,
        menus:Option<Vec<bottom_menu::BottomMenuItem>>
    )-> Result<Html> {
        let element = document().create_element("workspace-view")?;
        html.inject_into(&element)?;

        let view = Html { 
            element,
            module,
            _html:html,
            menus
        };

        Ok(view)
    }

}

unsafe impl Send for Html { }
unsafe impl Sync for Html { }

impl View for Html {
    fn element(&self) -> Element {
        self.element.clone()
    }

    fn module(&self) -> Arc<dyn ModuleInterface> {
        self.module.clone()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn bottom_menus(&self)->Option<Vec<bottom_menu::BottomMenuItem>>{
        self.menus.clone()
    }
}
