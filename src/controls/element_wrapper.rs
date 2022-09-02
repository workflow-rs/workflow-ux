use workflow_ux::result::Result;
use web_sys::{CustomEvent, MouseEvent, Element};
use crate::controls::listener::Listener;

#[derive(Clone, Debug)]
pub struct ElementWrapper{
    pub element : Element,
    listeners: Vec<Listener<CustomEvent>>,
    click_listeners:Vec<Listener<MouseEvent>>,
}
impl ElementWrapper{
    pub fn new(element : Element)->Self{
        Self { element, listeners: Vec::new(), click_listeners: Vec::new() }
    }

    pub fn on<F>(&mut self, name:&str, t:F) ->Result<()>
    where
        F: FnMut(CustomEvent) ->Result<()> + 'static
    {
        let listener = Listener::new(t);
        self.element.add_event_listener_with_callback(name, listener.into_js())?;
        self.listeners.push(listener);
        Ok(())
    }

    pub fn on_click<F>(&mut self, t:F) ->Result<()>
    where
        F: FnMut(MouseEvent) -> Result<()> + 'static
    {
        let listener = Listener::new(t);
        self.element.add_event_listener_with_callback("click", listener.into_js())?;
        self.click_listeners.push(listener);
        Ok(())
    }
}
