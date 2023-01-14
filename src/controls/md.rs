use crate::prelude::*;
use workflow_html::{ElementResult, Hooks, Render, Renderables};

#[derive(Clone)]
pub struct MD {
    pub el: Element,
    pub body: String,
}

impl Default for MD {
    fn default() -> Self {
        Self {
            el: document().create_element("div").unwrap(),
            body: "".to_string(),
        }
    }
}

impl MD {
    pub fn new(body: String) -> crate::result::Result<Self> {
        Ok(Self {
            el: document().create_element("div")?,
            body,
        })
    }
}

impl Render for MD {
    fn render_node(
        self,
        parent: &mut Element,
        _map: &mut Hooks,
        renderables: &mut Renderables,
    ) -> ElementResult<()>
    where
        Self: Sized,
    {
        self.el.class_list().add_1("md-container-el")?;
        self.el.set_inner_html(&self.body);
        parent.append_child(&self.el)?;
        renderables.push(Arc::new(self));
        Ok(())
    }

    fn render(&self, _w: &mut Vec<String>) -> ElementResult<()> {
        Ok(())
    }
}
