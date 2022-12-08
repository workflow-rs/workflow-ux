
use workflow_ux::result::Result;
use workflow_ux::prelude::*;
use workflow_ux::error::error;
use workflow_core::{task::spawn, channel::{Sender, Receiver}};
//use core::future::Future;

pub trait Emitter<T:Send>{
    fn register_event_channel()->(Id, Sender<T>, Receiver<T>);
    fn unregister_event_channel(id:Id);
    fn halt_event()->Option<T>;
}

#[workflow_async_trait]
pub trait Listener<T:Send>:Sync+Send{
    async fn digest_event(self: Arc<Self>, event:T)->Result<bool>;
}

//pub type Callback<T> = Box<dyn Fn(T)->(dyn Future<Output = Result<bool>>)+Send+Sync>;
//pub type Callback<T> = Box<dyn Fn(T)->(dyn Future<Output = Result<bool>>+Sync)+Send+Sync>;

pub struct Subscriber<T:Send, E> {
    sender : Arc<Mutex<Option<Sender<T>>>>,
    e:PhantomData<E>,
    listener:Arc<dyn Listener<T>>
    //a:PhantomData<A>
    //callback: Callback<T>
}


impl<T, E> Subscriber<T, E>
where 
T:Send + 'static,
E:Emitter<T>
{
    pub fn new(/*callback:Callback<T>*/ listener:Arc<dyn Listener<T>+Send+Sync>)->Result<Self>{
        Ok(Self{
            sender: Arc::new(Mutex::new(None)),
            listener,
            e:PhantomData,
            //callback
        })
    }

    pub fn subscribe(self: Arc<Self>) -> Result<()>{

        let (id, sender, receiver) = E::register_event_channel();

        *self.sender.lock().unwrap() = Some(sender);
        let listener = self.listener.clone();

        spawn(async move {
            loop {
                let listener_ = listener.clone();
                match receiver.recv().await {
                    Ok(event) => {
                        match listener_.digest_event(event).await {
                            Ok(keep_alive) => {
                                if !keep_alive {
                                    log_warning!("Subscriber digest() halt");
                                    break;
                                }
                            },
                            Err(err) => {
                                log_error!("Subscriber digest() error: {:?}", err);
                            }
                        }
                    },
                    Err(err) => {
                        log_error!("Subscriber recv() error: {:?}", err);
                    }
                }
            }

            E::unregister_event_channel(id);
        });

        Ok(())
    }

    pub fn unsubscribe(self: Arc<Self>) -> Result<()>{
        if let Some(halt_event) = E::halt_event(){
            self
                .sender
                .try_lock()
                .unwrap()
                .as_ref()
                .expect("No channel in Subscriber")
                .try_send(halt_event)
                .map_err(|_|error!("Subscriber::halt() ... try_send() failure"))?;
        }
        Ok(())
    }

}
