use workflow_html::{Render, Hooks, html, Html, Renderables, ElementResult};
use web_sys::Element;
use crate::result::Result;
use std::{sync::{Arc, Mutex}, fmt::Debug};
use workflow_log::log_error;

pub static CSS:&'static str = include_str!("pagination.css");


#[derive(Debug)]
pub struct PaginationPage{
    pub page:u32,
    pub skip:u32,
    pub active:bool
}


#[derive(Clone, Debug)]
pub struct PaginationOptions{
    pub first:String,
    pub last:String,
    pub prev:String,
    pub next:String
}
impl PaginationOptions{
    pub fn new()->Self{
        Self{
            ..Default::default()
        }
    }
}
impl Default for PaginationOptions{
    fn default() -> Self {
        Self{
            first:"FIRST".to_string(),
            last:"LAST".to_string(),
            prev:"PREV".to_string(),
            next:"NEXT".to_string(),
        }
    }
}

pub type PaginationCallback = Arc<dyn Fn(Pagination, u32)->Result<()>>;

#[derive(Clone)]
pub struct Pagination{
    pub name: Option<String>,
    pub total_pages:u32,
    pub active_page:u32,
    pub is_last:bool,
    pub is_first:bool,
    pub prev:u32,
    pub next:u32,
    pub last:u32,
    pub last_skip:u32,
    pub prev_skip:u32,
    pub next_skip:u32,
    pub total:u32,
    pub skip:u32,
    pub limit:u32,
    pub pages:Arc<Vec<PaginationPage>>,
    pub max_pages:u32,
    pub half:u32,
    pub callback:Arc<Mutex<Option<PaginationCallback>>>,
    pub options:Option<PaginationOptions>
}

impl Debug for Pagination{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Pagination")
            .field("name", &self.name)
            .field("total_pages", &self.total_pages)
            .field("active_page", &self.active_page)
            .field("is_last", &self.is_last)
            .field("is_first", &self.is_first)
            .field("prev", &self.prev)
            .field("next", &self.next)
            .field("last", &self.last)
            .field("last_skip", &self.last_skip)
            .field("prev_skip", &self.prev_skip)
            .field("next_skip", &self.next_skip)
            .field("total", &self.total)
            .field("skip", &self.skip)
            .field("limit", &self.limit)
            .field("pages", &self.pages)
            .field("max_pages", &self.max_pages)
            .field("half", &self.half)
            .field("options", &self.options);
        Ok(())
    }
}

impl Pagination{
    pub fn new(total:u32, skip:Option<u32>, limit:Option<u32>, max_pages:Option<u32>)->Self{
        let skip = skip.unwrap_or(0);
        let limit = limit.unwrap_or(25);
        let total_pages = (total as f32 / limit as f32).ceil() as u32;
        let active_page = total_pages.min( ((skip+1) as f32 / limit as f32).ceil() as u32);
        let max_pages = max_pages.unwrap_or(10).min(total_pages).min(10);
        let half = (max_pages as f32 / 2.0).floor() as u32;
        let prev = 1.max(active_page - 1);
        let next = total_pages.min(active_page + 1);
        let mut page = 1;
        println!("active_page: {}, half:{}, max_pages:{}, total_pages:{}", active_page, half, max_pages, total_pages);
        if active_page > half{
            page = active_page + half.min(total_pages - active_page) + 1 - max_pages ;
        }

        let mut pages = Vec::new();
        for _ in 0..max_pages{
            pages.push(PaginationPage{
                page,
                skip: (page-1)*limit,
                active: active_page==page,
            });
            page = page+1;
        }
        Self{
            name:None,
            total_pages,
            active_page,
            is_last:active_page==total_pages,
            is_first:active_page==1,
            prev,
            next,
            last:total_pages,
            last_skip:(total_pages-1)*limit,
            prev_skip:(prev-1) * limit,
            next_skip:(next-1) * limit,
            total,
            skip,
            limit,
            pages:Arc::new(pages),
            max_pages,
            half,
            callback:Arc::new(Mutex::new(None)),
            options:None
        }
    }

    pub fn with_name(mut self, name:String)->Result<Self>{
        self.name = Some(name);
        Ok(self)
    }
    pub fn with_options(mut self, options:PaginationOptions)->Result<Self>{
        self.options = Some(options);
        Ok(self)
    }
    pub fn with_callback(self, callback:PaginationCallback)->Result<Self>{
        (*self.callback.lock()?) = Some(callback);
        Ok(self)
    }

    pub fn on_click(&self, target:Element)->Result<()>{
        let el = target.closest("[data-skip]")?;
        if let Some(el) = el{
            if el.has_attribute("disabled"){
                return Ok(())
            }
            let skip = el.get_attribute("data-skip").unwrap();
            let skip = skip.parse::<u32>()?;
            if let Some(cb) = self.callback.lock()?.as_ref(){
                (*cb)(self.clone(), skip)?;
            }
        }
        Ok(())
    }

    pub fn render_pagination(&self)->Result<Html>{
        let pages = self.pages.clone();
        let is_first = self.is_first;
        let is_last = self.is_last;
        let prev_skip = self.prev_skip;
        let next_skip = self.next_skip;
        //let total_pages = self.total_pages;
        let last_skip = self.last_skip;
        let name = self.name.clone().unwrap_or("workflow".to_string());
    
        let options = self.options.clone().unwrap_or(PaginationOptions::new());
        let first_text = options.first;
        let last_text = options.last;
        let prev_text = options.prev;
        let next_text = options.next;

        if pages.len() == 0{
            return Ok(html!{
                <div class="workflow-pagination-box" disabled="true">
                    <div class="workflow-pagination" data-pagination={name}>
                    </div>
                </div>
            }?);
        }

        let mut list = Vec::new();
        for p in pages.iter(){
            list.push(html!{
                <a ?active={p.active} data-skip={format!("{}", p.skip)}>{format!("{}", p.page)}</a>
            }?);
        }

        let this = self.clone();

        Ok(html!{
            <div class="workflow-pagination-box">
                <div class="workflow-pagination" data-pagination={name}
                    !click={
                        this.on_click(_target).map_err(|e|{
                            log_error!("workflow-pagination click: {}", e)
                        }).ok();
                    }>
                    <a ?disabled={is_first} class="first" data-skip="0">{first_text}</a>
                    <a ?disabled={is_first} class="prev" data-skip={format!("{prev_skip}")}>{prev_text}</a>
                    {list}
                    <a ?disabled={is_last} class="next" data-skip={format!("{next_skip}")}>{next_text}</a>
                    <a ?disabled={is_last} class="last" data-skip={format!("{last_skip}")}>{last_text}</a>
                </div>
            </div>
        }?)
    }

}

impl Render for Pagination{
    fn render_node(
        self,
        parent:&mut Element,
        map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>
        where Self: Sized
    {
        let html = self.render_pagination()?;
        html.render_node(parent, map, renderables)?;
        Ok(())
    }

    fn render(&self, _w: &mut Vec<String>)->ElementResult<()> {
        Ok(())
    }
}
