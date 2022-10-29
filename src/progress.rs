use std::sync::atomic::{AtomicBool,Ordering};
use workflow_ux::prelude::*;
use workflow_ux::result::Result;
use workflow_core::id::Id;
// use workflow_ux::view;
use workflow_ux::view::*;
//{Meta,View,Html,into_meta_view,get_meta};


#[derive(Clone)]
pub struct Progress {
    id : Id,
    aborted : Arc<AtomicBool>,
    view : Arc<Mutex<Option<Arc<dyn view::View>>>>,
}

impl Meta for Progress { }

impl Eq for Progress { }

impl PartialEq for Progress {
    fn eq(&self, other: &Self) -> bool {
        &self.id == &other.id
    }
}

impl Progress {
    pub fn new(html: workflow_html::Html) -> Result<Arc<Self>> {

        let html_view = Html::try_new(None, html)?;
        let progress = Arc::new(Progress { 
            id : Id::new(),
            aborted : Arc::new(AtomicBool::new(false)),
            view : Arc::new(Mutex::new(None))
        });
        let view = into_meta_view(html_view,progress.clone())?;
        progress.view.lock().unwrap().replace(view);
        Ok(progress)
    }

    pub fn view(self: &Arc<Self>) -> Result<Arc<dyn View>> {
        Ok(self.view.lock()?.as_ref().unwrap().clone())
    }

    // utility function to get progress from a supplied view
    fn from_view(view: &Arc<dyn View>) -> Option<Arc<Progress>> {
        match get_meta::<Progress>(view.clone()) {
            Ok(progress) => Some(progress),
            Err(_) => None
        }
    }


    // TODO call this with current main view before evicting it
    pub fn abort(view: &Arc<dyn View>) {
        let current = Progress::from_view(view);
        if let Some(progress) = current {
            progress.aborted.store(true,Ordering::SeqCst);
        }
    }

    pub fn ok(&self, view: &Arc<dyn View>) -> bool {
        let current = Progress::from_view(view);
        match current {
            Some(progress) if progress.id == self.id => {
                if progress.aborted.load(Ordering::SeqCst) {
                    false
                } else {
                    true
                }
            },
            _ => false
        }
    }
}

