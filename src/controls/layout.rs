// use crate::prelude::*;
// use crate::layout::{ElementLayout, ElementLayoutStyle};
// // use std::convert::Into;
// // use intertrait::*;
// // use intertrait::cast::*;

// // #[wasm_bindgen]
// #[derive(Clone)]
// pub struct Layout<T> {
//     pub parent: ElementLayout,
//     pub element : Element,
//     pub content : T,
// }

// impl<T> Layout<T> {
//     pub fn element(&self) -> Result<Element,JsValue> {
//         Ok(self.element.clone())
//     }

//     pub fn new(parent : &ElementLayout, content : &ElementLayout, attributes: Attributes, _docs : Docs) -> Result<Layout<T>,JsValue> {  // pane-ctl

//         let parent_el = parent.element();
//         let content_el = content.element();
//         parent_el.append_child(&content_el);
//         // parent.element().append_child(&child.element());

//         // let element = document()
//         //     .create_element("div")?;

//         // let parent = parent_layout.inner().ok_or("Layout::new() unable to mut lock layout inner")?;

//         // let layout_style = ElementLayoutStyle::Pane;
//         // // let content = ElementLayout::new(&parent.id, &layout_style, attributes)?;


//         // let content = T::new(&parent.id, &layout_style, attributes)?;


//         Ok(Layout { 
//             parent : parent.clone(),
//             content : content.clone(),
//             element : content.element.clone(),
//         })
//     }

// }
