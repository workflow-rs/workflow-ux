use std::{sync::{Arc, Mutex}, any::TypeId};

use crate::{prelude::*, app_menu::AppMenu};
use crate::{bottom_menu, layout, result::Result};
use downcast::{downcast_sync, AnySync};
use workflow_log::log_trace;

//use web_sys::{ScrollBehavior, ScrollToOptions};
//use crate::view::base_element::ExtendedElement;
#[derive(Clone)]
pub struct ContainerStack {
    element: Element,
    views : Arc<RwLock<Vec<Arc<dyn View>>>>
}

impl ContainerStack {
    pub fn new(element: Element) -> Self {
        let _ = element.set_attribute("data-container-type", "stack");
        Self {
            element,
            views : Arc::new(RwLock::new(Vec::new()))
        }
    }

    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub async fn append_view(self : &Arc<Self>, incoming : Arc<dyn View>) -> Result<()> {
        (*self.views.write().await).push(incoming.clone());
        self.element.append_child(&incoming.element())?;
        Ok(())
    }
}

impl Into<Element> for ContainerStack {
    fn into(self) -> Element {
        self.element.clone()
    }
}

#[derive(Clone)]
pub struct Container {
    element: Element,
    view : Arc<RwLock<Option<Arc<dyn View>>>>,
    app_menu: Option<Arc<AppMenu>>
}

impl Container {
    pub fn new(element: Element, app_menu:Option<Arc<AppMenu>>) -> Self {
        Container {
            element,
            view : Arc::new(RwLock::new(None)),
            app_menu
        }
    }

    pub fn element(&self) -> Element {
        self.element.clone()
    }

    pub async fn load_view(self : &Arc<Self>, incoming : Arc<dyn View>) -> Result<Option<Arc<dyn View>>> {
        let from = self.swap_from().await?;
        self.swap_to(incoming).await?;
        Ok(from)
    }

    pub async fn load_html(
        self : &Arc<Self>,
        // module : Arc<dyn ModuleInterface>,
        html : workflow_html::Html,
    ) -> Result<Option<Arc<dyn View>>> {
        // let view = view::Html::try_new(module.clone(), html)?;
        let view = view::Html::try_new(None, html)?;
        let from = self.swap_from().await?;
        self.swap_to(view).await?;
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
                // log_trace!("swap_from(): there is no previous view");
                Ok(None) 
            },
            Some(previous) => {
                // let module = previous.module();
                if let Some(module) = previous.module() {
                    // TODO query module for view eviction etc.
                    module.evict(self, previous.clone()).await?;
                    previous.clone().evict().await?;
                    log_trace!("swap_from(): finishing...");
                    Ok(Some(previous.clone()))
                } else {
                    Ok(None)
                }
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
        
        if let Some(app_menu) = &self.app_menu{
            //log_trace!("app_menu.update_bottom_menus: {:?}", incoming.bottom_menus());
            app_menu.update_bottom_menus(incoming.bottom_menus())?;
        }

        self.element.append_child(&incoming.element())?;

        /*
        let mut scroll_opt = ScrollToOptions::new();
        scroll_opt.behavior(ScrollBehavior::Smooth);
        scroll_opt.left(0.0);
        scroll_opt.top(10.0);
        /*
        let opt = js_sys::Object::new();
        js_sys::Reflect::set(&opt, &JsValue::from("top"), &JsValue::from(0))?;
        js_sys::Reflect::set(&opt, &JsValue::from("left"), &JsValue::from(0))?;
        js_sys::Reflect::set(&opt, &JsValue::from("behavior"), &JsValue::from("smooth"))?;
        self.element.clone().dyn_into::<ExtendedElement>()?.scroll_to(&opt);
        */
        //multiple scroll_to() just to trigger scroll event
        log_trace!("self.element.id:{:?}", self.element.get_attribute("class"));
        self.element.scroll_to_with_scroll_to_options(&scroll_opt);
        scroll_opt.top(0.0);
        self.element.scroll_to_with_scroll_to_options(&scroll_opt);
        */
        

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn view(&self) -> Option<Arc<dyn View>> {
        self.view.read().await.clone()
    }

    pub fn try_view(&self) -> Result<Option<Arc<dyn View>>> {
        Ok(self.view.try_read().ok_or("Unabel to lock view")?.clone())
    }

    pub fn meta<M>(&self)
    -> Result<Arc<M>> 
    where M: AnySync
    {
        let view = self.try_view()?
            .ok_or("Unable to get current view")?;
        let meta_view = view.downcast_arc::<MetaView>()?;
        let meta = meta_view.meta()?;
        Ok(meta)
    }    

}

impl Into<Element> for Container {
    fn into(self) -> Element {
        self.element.clone()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



#[workflow_async_trait]
pub trait View : Sync + Send + AnySync {
    fn element(&self) -> Element;
    fn module(&self) -> Option<Arc<dyn ModuleInterface>>;
    fn typeid(&self) -> TypeId;
    async fn evict(self : Arc<Self>) -> Result<()> { Ok(()) }
    fn drop(&self) { }

    fn bottom_menus(&self)->Option<Vec<bottom_menu::BottomMenuItem>>{
        None
    }

    // fn with_meta(
    //     self : Arc<Self>,
    //     meta: Arc<dyn Meta>,
    //     // meta: Arc<dyn AnySync>,
    // ) 
    // -> Result<Arc<dyn View>> 
    // // where Self: Sized
    // {
    //     let view : Arc<dyn View> = self;
    //     // let view : Arc<dyn View> = self.clone();
    //     let meta_view = MetaView::try_new(self, meta)?;
    //     Ok(meta_view)
    // }

    // fn meta<M>(
    //     self : Arc<Self>,
    // ) -> Result<Arc<M>> 
    // where Self: Sized + AnySync// + DowncastSync
    // {
    //     let meta_view = self.downcast_arc::<MetaView>()?;

    // }

    // fn trigger(&self)->Option<ViewTrigger>{
    //     None
    // }

    // fn drop(&self) -> Result<()> { Ok(()) }
    //  {
    //     self.module.clone()
    // }
}

downcast_sync!(dyn View);

pub fn into_meta_view(view : Arc<dyn View>, meta: Arc<dyn Meta>) -> Result<Arc<dyn View>> {

    // let view : Arc<dyn View> = self.clone();
    // let view : Arc<dyn View> = self.clone();
    let meta_view = MetaView::try_new(view, meta)?;
    Ok(meta_view)

}

pub fn get_meta<M>(view : Arc<dyn View>)
-> Result<Arc<M>> 
where M: AnySync
{
    let meta_view = view.downcast_arc::<MetaView>()?;
    let meta = meta_view.meta()?;
    Ok(meta)
}

// unsafe impl Sync for dyn View {} 

pub struct Default {
    element : Element,
    module : Option<Arc<dyn ModuleInterface>>
}

unsafe impl Send for Default { }
unsafe impl Sync for Default { }

impl Default {
    pub fn try_new(module : Option<Arc<dyn ModuleInterface>>) -> Result<Arc<dyn View>> {
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

    fn module(&self) -> Option<Arc<dyn ModuleInterface>> {
        self.module.clone()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}


pub struct Data<D> {
    data : Arc<Mutex<D>>,
    element : Element,
    module : Option<Arc<dyn ModuleInterface>>, 
}

impl<D> Data<D> {
    // pub fn try_new(module : Arc<dyn ModuleInterface>) -> Result<Arc<dyn View>> {
    pub fn try_new(module : Option<Arc<dyn ModuleInterface>>, data : D) -> Result<Arc<Data<D>>> {
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

    fn module(&self) -> Option<Arc<dyn ModuleInterface>> {
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

type AsyncMutex<A> = std::sync::Mutex<A>;
pub struct Layout<F,D> {
    layout : Arc<AsyncMutex<F>>,
    data : Arc<Mutex<Option<D>>>,
    evict : Arc<Mutex<Option<EvictFn>>>,
    drop : Arc<Mutex<Option<DropFn>>>,
    element : Element,
    module : Option<Arc<dyn ModuleInterface>>, 
}

impl<F,D> Layout<F,D> 
where 
    F : layout::Elemental + Send + 'static,
    D : Send + 'static
{
    // pub fn try_new(module : Arc<dyn ModuleInterface>) -> Result<Arc<dyn View>> {
    pub fn try_new(module : Option<Arc<dyn ModuleInterface>>, layout : F, data : Option<D>) -> Result<Arc<Layout<F,D>>> {

        let element = document().create_element("workspace-view")?;
        element.append_child(&layout.element())?;

        let view = Layout::<F,D> { 
            element,
            module,
            layout : Arc::new(AsyncMutex::new(layout)),
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

    pub fn layout(&self) -> Arc<AsyncMutex<F>> {
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


#[workflow_async_trait]
impl<F,D> View for Layout<F,D> 
where 
    F : layout::Elemental + 'static,
    D : 'static
{
    fn element(&self) -> Element {
        self.element.clone()
    }

    fn module(&self) -> Option<Arc<dyn ModuleInterface>> {
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

// #[derive(Clone)]
// pub enum ViewTrigger{
//     Create(String),
//     Update(String),
//     Delete(String),
//     Custom(String)
// }
// unsafe impl Send for ViewTrigger { }
// unsafe impl Sync for ViewTrigger { }

pub struct Html {
    element : Element,
    module : Option<Arc<dyn ModuleInterface>>,
    _html: workflow_html::Html,
    menus:Option<Vec<bottom_menu::BottomMenuItem>>,
    // trigger: Option<ViewTrigger>
}

impl Html {
    pub fn try_new(
        module : Option<Arc<dyn ModuleInterface>>,
        html : workflow_html::Html,
    ) -> Result<Arc<dyn View>> {
        // let view = Self::create(module, html, None, None)?;
        let view = Self::create(module, html, None)?;
        Ok(Arc::new(view))
    }
    // pub fn try_new_with_trigger(
    //     module : Option<Arc<dyn ModuleInterface>>,
    //     html : workflow_html::Html,
    //     // trigger:ViewTrigger
    // ) -> Result<Arc<dyn View>> {
    //     let view = Self::create(module, html, None, Some(trigger))?;
    //     Ok(Arc::new(view))
    // }

    pub fn try_new_with_menus(
        module : Option<Arc<dyn ModuleInterface>>,
        html : workflow_html::Html,
        menus:Vec<bottom_menu::BottomMenuItem>
    )-> Result<Arc<dyn View>> {
        let view = Self::create(module, html, Some(menus))?;
        // let view = Self::create(module, html, Some(menus), None)?;
        Ok(Arc::new(view))
    }

    pub fn create(
        module : Option<Arc<dyn ModuleInterface>>,
        html : workflow_html::Html,
        menus:Option<Vec<bottom_menu::BottomMenuItem>>,
        // trigger:Option<ViewTrigger>
    )-> Result<Html> {
        let element = document().create_element("workspace-view")?;
        html.inject_into(&element)?;

        let view = Html { 
            element,
            module,
            _html:html,
            menus,
            // trigger
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

    fn module(&self) -> Option<Arc<dyn ModuleInterface>> {
        self.module.clone()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn bottom_menus(&self)->Option<Vec<bottom_menu::BottomMenuItem>>{
        self.menus.clone()
    }

    // fn trigger(&self)->Option<ViewTrigger>{
    //     self.trigger.clone()
    // }
}


pub trait Meta : AnySync {
    // type Data;
    // fn get(&self) -> Option<Arc<Self::Data>>;
}

downcast_sync!(dyn Meta);

// pub struct MetaView<D:Clone>{
pub struct MetaView{
    pub view:Arc<dyn View>,
    pub meta:Arc<dyn Meta>
    // pub meta:Arc<dyn AnySync>
}

// unsafe impl<D: Clone> Send for MetaView<D> { }
// unsafe impl<D: Clone> Sync for MetaView<D> { }
unsafe impl Send for MetaView { }
unsafe impl Sync for MetaView { }

impl MetaView
{
    // pub fn try_new<V>(
    //     view : Arc<V>,
    pub fn try_new(
        view : Arc<dyn View>,
        meta: Arc<dyn Meta>
        // meta: Arc<dyn AnySync>
    ) -> Result<Arc<dyn View>> 
    // where V: View
    {
        let view: Arc<dyn View> = Arc::new(Self{
            view, meta
        });

        Ok(view)
    }

    // pub fn meta(&self)->Option<Arc<dyn AnySync>>{
    //     self.meta.clone()
    // }

    
    pub fn meta<M : AnySync>(&self)->Result<Arc<M>> {
        // self.meta.clone()
        let meta = self.meta.clone();
        let data = meta.downcast_arc::<M>()?;
        Ok(data)
    }


}


// impl<D> View for MetaView<D>
#[workflow_async_trait]
impl View for MetaView
// where D : Clone + 'static{
{
    fn element(&self) -> Element {
        self.view.element()
    }

    fn module(&self) -> Option<Arc<dyn ModuleInterface>> {
        self.view.module()
    }

    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    async fn evict(self : Arc<Self>) -> Result<()> {
        self.view.clone().evict().await
    }


}

