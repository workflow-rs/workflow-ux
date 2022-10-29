use std::sync::atomic::{AtomicBool,Ordering};
use workflow_ux::prelude::*;
use workflow_ux::result::Result;
use workflow_core::id::Id;
use workflow_ux::view::*;


#[derive(Debug, Clone)]
pub struct Progress {
    id : Id,
    aborted : Arc<AtomicBool>,
}

impl Meta for Progress { }

impl Eq for Progress { }

impl PartialEq for Progress {
    fn eq(&self, other: &Self) -> bool {
        &self.id == &other.id
    }
}

impl Progress {
    pub fn new() -> Self {
        Self { 
            id : Id::new(),
            aborted : Arc::new(AtomicBool::new(false)),
        }
    }

    // utility function to get progress from a supplied view
    fn from_view(view: &Arc<dyn View>) -> Option<Arc<Progress>> {
        match get_meta::<Progress>(view.clone()) {
            Ok(progress) => Some(progress),
            Err(_) => None
        }
    }

    // convert view into meta view with progress as meta
    pub fn attach(self: Arc<Self>, view : &Arc<dyn View>) -> Result<Arc<dyn View>> {
        self.activate_with_view(view);
        into_meta_view(view.clone(),self.clone())
    } 

    // pub fn load(self: Arc<Self>, container: &Arc<Container>) -> Result<Arc<dyn View>> {
    //     self.activate_with_container(container);
    //     into_meta_view(container.view(),self.clone())
    // } 

    pub fn activate_with_view(&self, _view: &Arc<dyn View>) {
        // TODO - add class or load view 
    }

    // pub fn activate_with_container(&self, container: &Arc<Container>) {
    //     let view = container.view();
    //     // TODO - add class or load view 
    // }

    // pub fn deactivate(&self, container: &Arc<Container>) {
    //     // TODO - remove class
    // }

    pub fn deactivate(&self, _view: &Arc<dyn View>) {
        // TODO - remove class
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
                    self.deactivate(view);
                    true
                }
            },
            _ => false
        }
    }
}

