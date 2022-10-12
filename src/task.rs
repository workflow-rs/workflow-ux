use std::sync::{Arc, LockResult, MutexGuard, Mutex};
use wasm_bindgen::prelude::*;
//use workflow_log::log_trace;
use crate::result::Result;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (catch, js_name = setTimeout)]
    pub fn set_timeout(closure: &Closure<dyn FnMut()->Result<()>>, timeout: u32) -> std::result::Result<u32, JsValue>;
    #[wasm_bindgen (catch, js_name = clearTimeout)]
    pub fn clear_timeout(interval: u32) -> std::result::Result<(), JsValue>;
}

pub enum FunctionDebounceCallback{
    NoArgs(Box<dyn FnMut()->Result<()>>),
    WithString(Box<dyn FnMut(String)->Result<()>>),
    WithUsize(Box<dyn FnMut(usize)->Result<()>>)
}
#[derive(Clone)]
pub enum FunctionDebounceCallbackArgs{
    String(String),
    Usize(usize),
    None
}
pub struct FunctionDebounceInner{
    cb: FunctionDebounceCallback,
    cb_args:Option<FunctionDebounceCallbackArgs>,
    interval:Option<u32>,
    closure:Option<Closure<dyn FnMut()->Result<()>>>
}

#[derive(Clone)]
pub struct FunctionDebounce{
    duration:u32,
    inner:Arc<Mutex<FunctionDebounceInner>>
}

impl FunctionDebounce{
    pub fn create(duration:u32, cb:FunctionDebounceCallback)->Self{
        Self{
            duration,
            inner: Arc::new(Mutex::new(FunctionDebounceInner{
                cb,
                closure: None,
                cb_args: None,
                interval: None
            }))
        }
    }
    pub fn new(duration:u32, cb:Box<dyn FnMut()->Result<()>>)->Self{
        Self::create(duration, FunctionDebounceCallback::NoArgs(cb))
    }
    pub fn new_with_str(duration:u32, cb:Box<dyn FnMut(String)->Result<()>>)->Self{
        Self::create(duration, FunctionDebounceCallback::WithString(cb))
    }
    pub fn new_with_usize(duration:u32, cb:Box<dyn FnMut(usize)->Result<()>>)->Self{
        Self::create(duration, FunctionDebounceCallback::WithUsize(cb))
    }
    fn inner(&self)->LockResult<MutexGuard<FunctionDebounceInner>>{
        self.inner.lock()
    }
    fn clear_timeout(&self)->Result<()>{
        if let Some(interval) = self.inner()?.interval{
            clear_timeout(interval).unwrap();
        }

        Ok(())
    }
    fn run_callback(&self)->Result<()>{
        let mut locked = self.inner()?;
        if let Some(interval) = locked.interval{
            clear_timeout(interval).unwrap();
        }

        let args = locked.cb_args.clone().unwrap_or(FunctionDebounceCallbackArgs::None);
        match args{
            FunctionDebounceCallbackArgs::String(str)=>{
                match &mut locked.cb{
                    FunctionDebounceCallback::WithString(cb)=>{
                        cb(str)?;
                    }
                    _=>{
                        return Ok(());
                    }
                }
            }
            FunctionDebounceCallbackArgs::Usize(n)=>{
                match &mut locked.cb{
                    FunctionDebounceCallback::WithUsize(cb)=>{
                        cb(n)?;
                    }
                    _=>{
                        return Ok(());
                    }
                }
            }
            FunctionDebounceCallbackArgs::None=>{
                match &mut locked.cb{
                    FunctionDebounceCallback::NoArgs(cb)=>{
                        cb()?;
                    }
                    _=>{
                        return Ok(());
                    }
                }
            }
        }
        
        Ok(())
    }
    pub fn execute(&self)->Result<()>{
        self.execute_(FunctionDebounceCallbackArgs::None)?;
        Ok(())
    }
    pub fn execute_with_str(&self, str:String)->Result<()>{
        self.execute_(FunctionDebounceCallbackArgs::String(str))?;
        Ok(())
    }
    pub fn execute_with_usize(&self, n:usize)->Result<()>{
        self.execute_(FunctionDebounceCallbackArgs::Usize(n))?;
        Ok(())
    }
    fn execute_(&self, args:FunctionDebounceCallbackArgs)->Result<()>{
        
        self.clear_timeout()?;
        let closure;
        {
            let this = self.clone();
            closure = Closure::new(move ||->Result<()>{
                this.run_callback()?;
                Ok(())
            });
        }
        let duration = self.duration;
        let mut locked = self.inner()?;
        locked.interval = Some(set_timeout(&closure, duration).unwrap());
        locked.closure = Some(closure);
        locked.cb_args = Some(args);
        Ok(())
    }
}
