use core::future::Future;
use core::pin::Pin;
use workflow_core::{
    channel::{Receiver, Sender},
    task::spawn,
};
use workflow_ux::error::error;
use workflow_ux::prelude::*;
use workflow_ux::result::Result;

pub trait Emitter<T: Send> {
    fn register_event_channel() -> (Id, Sender<T>, Receiver<T>);
    fn unregister_event_channel(id: Id);
    fn halt_event() -> Option<T>;
}

#[workflow_async_trait]
pub trait Listener<T: Send>: Sync + Send {
    async fn digest_event(self: Arc<Self>, event: T) -> Result<bool>;
}

pub struct Subscriber<T: Send, E> {
    sender: Arc<Mutex<Option<Sender<T>>>>,
    active: Arc<Mutex<bool>>,
    listener: Arc<dyn Listener<T>>,
    e: PhantomData<E>,
}

unsafe impl<T, E> Send for Subscriber<T, E>
where
    T: Send + 'static,
    E: Emitter<T> + 'static,
{
}

unsafe impl<T, E> Sync for Subscriber<T, E>
where
    T: Send + 'static,
    E: Emitter<T> + 'static,
{
}

impl<T, E> Subscriber<T, E>
where
    T: Send + 'static,
    E: Emitter<T> + 'static,
{
    pub fn new(listener: Arc<dyn Listener<T> + Send + Sync>) -> Result<Self> {
        Ok(Self {
            sender: Arc::new(Mutex::new(None)),
            active: Arc::new(Mutex::new(false)),
            listener,
            e: PhantomData,
        })
    }

    fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }
    fn set_active(&self, active: bool) {
        *self.active.lock().unwrap() = active;
    }

    pub fn subscribe(self: Arc<Self>) -> Result<()> {
        if self.is_active() {
            return Ok(());
        }

        let (id, sender, receiver) = E::register_event_channel();

        *self.sender.lock().unwrap() = Some(sender);

        self.set_active(true);

        spawn(async move {
            loop {
                let listener_ = self.listener.clone();
                match receiver.recv().await {
                    Ok(event) => match listener_.digest_event(event).await {
                        Ok(keep_alive) => {
                            if !keep_alive {
                                log_warning!("Subscriber digest() halt");
                                break;
                            }
                        }
                        Err(err) => {
                            log_error!("Subscriber digest() error: {:?}", err);
                        }
                    },
                    Err(err) => {
                        log_error!("Subscriber recv() error: {:?}", err);
                    }
                }
            }
            self.set_active(false);
            E::unregister_event_channel(id);
        });

        Ok(())
    }

    pub fn unsubscribe(self: Arc<Self>) -> Result<()> {
        if let Some(halt_event) = E::halt_event() {
            self.sender
                .try_lock()
                .unwrap()
                .as_ref()
                .expect("No channel in Subscriber")
                .try_send(halt_event)
                .map_err(|_| error!("Subscriber::halt() ... try_send() failure"))?;
        }
        Ok(())
    }
}

pub type CallbackResult = Pin<Box<dyn Future<Output = Result<bool>> + Send>>;

pub fn subscribe<E, C, U>(receiver: Receiver<E>, callback: C, finish_callback: U) -> Result<()>
where
    E: Send + 'static,
    U: Fn() + Send + 'static,
    C: Fn(E) -> CallbackResult + Send + 'static,
{
    spawn(async move {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let f = callback(event);
                    match f.await {
                        Ok(keep_alive) => {
                            if !keep_alive {
                                log_warning!("subscribe digest() halt");
                                break;
                            }
                        }
                        Err(err) => {
                            log_error!("subscribe digest() error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    log_error!("subscribe recv() error: {:?}", err);
                }
            }
        }

        finish_callback();
    });

    Ok(())
}
