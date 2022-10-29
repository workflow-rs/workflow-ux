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
    container : Arc<Container>,
}

impl Meta for Progress { }

impl Eq for Progress { }

impl PartialEq for Progress {
    fn eq(&self, other: &Self) -> bool {
        &self.id == &other.id
    }
}

impl Progress {
    pub async fn try_load(html: workflow_html::Html, container : Arc<Container>) -> Result<Arc<Self>> {

        container.swap_from().await?;

        let html_view = Html::try_new(None, html)?;
        let progress = Arc::new(Progress { 
            id : Id::new(),
            aborted : Arc::new(AtomicBool::new(false)),
            view : Arc::new(Mutex::new(None)),
            container: container.clone()
        });
        let view = into_meta_view(html_view,progress.clone())?;
        progress.view.lock().unwrap().replace(view.clone());

        container.swap_to(view).await?;

        Ok(progress)
    }

    // pub fn view(self: &Arc<Self>) -> Result<Arc<dyn View>> {
    //     Ok(self.view.lock()?.as_ref().unwrap().clone())
    // }

    // utility function to get progress from a supplied view
    fn from_view(view: &Arc<dyn View>) -> Option<Arc<Progress>> {
        match get_meta::<Progress>(view.clone()) {
            Ok(progress) => Some(progress),
            Err(_) => None
        }
    }

    pub fn abort(view: &Arc<dyn View>) {
        let current = Progress::from_view(view);
        if let Some(progress) = current {
            progress.aborted.store(true,Ordering::SeqCst);
        }
    }

    pub fn aborted(self: &Arc<Self>) -> Result<bool> {
        let current = self.container.view()
            .ok_or("Current view is missing")?;
        
        let current = Progress::from_view(&current);
        let ok = match current {
            Some(progress) if progress.id == self.id => {
                if progress.aborted.load(Ordering::SeqCst) {
                    false
                } else {
                    true
                }
            },
            _ => false
        };

        Ok(!ok)
    }

    pub fn close(self : &Arc<Self>) -> Result<()> {
        if self.aborted()? {
            Err("View aborted".into())
        } else {
            Ok(())
        }
    }
    
}

