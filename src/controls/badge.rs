use crate::prelude::*;
use workflow_html::{ElementResult, Hooks, Render, Renderables};
use workflow_ux::result::Result;

#[wasm_bindgen]
extern "C" {
    /// The `FlowDataBadgeGraph` class.
    #[wasm_bindgen (extends = BaseElement, js_name = FlowDataBadgeGraph , typescript_type = "FlowDataBadgeGraph")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type FlowDataBadgeGraph;

    //#[wasm_bindgen (method, getter, js_name = value)]
    //pub fn value(this: &FlowDataBadgeGraph) -> String;

    /// redraw the badge
    #[wasm_bindgen(method)]
    pub fn redraw(this: &FlowDataBadgeGraph, data: &js_sys::Array);
}

#[derive(Clone)]
pub struct Badge {
    pub element: Element,
    pub options: Options,
}

impl Badge {
    pub fn element(&self) -> FlowDataBadgeGraph {
        self.element
            .clone()
            .dyn_into::<FlowDataBadgeGraph>()
            .expect("Unable to cast element to FlowDataBadgeGraph")
    }

    pub fn new(_pane: &ElementLayout, attributes: &Attributes, _docs: &Docs) -> Result<Self> {
        let title = "".to_string();
        let title = attributes.get("title").unwrap_or(&title);
        let options = Options::from_attributes(attributes)?;
        let control = Self::create(title, options)?;
        Ok(control)
    }

    pub fn create(title: &str, options: Options) -> Result<Self> {
        /*
        let tree = html!{
            <div class="workflow-qrcode" @qr_el>
                <div class="qr-code" @qr_code_el></div>
            </div>
        }?;
        */
        /*
        <flow-data-badge-graph
            style="min-width:128px;"
            sampler="${this.task.key}-block-rate"
            suffix="${i18n.t(" / SEC")}"
            title="${i18n.t("BLOCK RATE")}"
            align="right">${this.blockrate.toFixed(2)}
            </flow-data-badge-graph>
        */
        let element = document().create_element("flow-data-badge-graph")?;

        let attributes: Attributes = options.clone().into();
        for (k, v) in attributes.iter() {
            if k.eq("text") {
                element.set_inner_html(v);
            } else if k.eq("has_colon") | k.eq("has-colon") {
                element.set_attribute("has-colon", "true")?;
            } else {
                element.set_attribute(k, v)?;
            }
        }

        element.set_attribute("title", &title)?;

        Ok(Self { element, options })
    }

    pub fn set_text(&self, text: &str) -> Result<()> {
        self.element.set_inner_html(text);
        Ok(())
    }

    pub fn redraw(&self, data: &js_sys::Array, text: Option<&str>) -> Result<()> {
        let el = self.element();
        el.redraw(data);
        if let Some(text) = text {
            el.set_inner_html(text);
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Options {
    pub sampler: Option<String>,
    //pub title: Option<String>,
    pub suffix: Option<String>,
    pub align: Option<String>,
    pub style: Option<String>,
    pub colon: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            sampler: None,
            //title: None,
            suffix: None,
            align: None,
            style: None,
            colon: true,
        }
    }
}

impl Options {
    /// Set badge sampler
    pub fn sampler(mut self, sampler: &str) -> Self {
        self.sampler = Some(sampler.to_string());
        self
    }

    // Set badge title
    //pub fn title(mut self, title:&str)->Self{
    //    self.title = Some(title.to_string());
    //    self
    //}
    /// Set badge suffix
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_string());
        self
    }
    /// Set badge alignment
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }
    /// Set badge style
    pub fn style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    /// Set badge colon
    pub fn colon(mut self, colon: bool) -> Self {
        self.colon = colon;
        self
    }

    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        let mut options = Self::default();
        if let Some(sampler) = attributes.get("sampler") {
            options.sampler = Some(sampler.clone());
        }

        //if let Some(title) = attributes.get("title"){
        //    options.title = Some(title.clone());
        //}

        if let Some(suffix) = attributes.get("suffix") {
            options.suffix = Some(suffix.clone());
        }

        if let Some(align) = attributes.get("align") {
            options.align = Some(align.clone());
        }

        if let Some(style) = attributes.get("style") {
            options.style = Some(style.clone());
        }

        if let Some(colon) = attributes.get("has_colon") {
            options.colon = colon.eq("true");
        }
        if let Some(colon) = attributes.get("has-colon") {
            options.colon = colon.eq("true");
        }

        Ok(options)
    }
}

impl From<Options> for Attributes {
    fn from(options: Options) -> Attributes {
        let mut attributes = Attributes::new();
        if let Some(sampler) = options.sampler {
            attributes.insert("sampler".to_string(), sampler.to_string());
        }
        //if let Some(title) = options.title{
        //    attributes.insert("title".to_string(), title.to_string());
        //}
        if let Some(suffix) = options.suffix {
            attributes.insert("suffix".to_string(), suffix.to_string());
        }
        if let Some(align) = options.align {
            attributes.insert("align".to_string(), align.to_string());
        }
        if let Some(style) = options.style {
            attributes.insert("style".to_string(), style.to_string());
        }
        if options.colon {
            attributes.insert("has_colon".to_string(), "true".to_string());
        }

        attributes
    }
}

impl Render for Badge {
    fn render(&self, _w: &mut Vec<String>) -> workflow_html::ElementResult<()> {
        Ok(())
    }

    fn render_node(
        self,
        parent: &mut Element,
        _map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()> {
        parent.append_child(&self.element)?;
        renderables.push(Arc::new(self));
        Ok(())
    }
}
