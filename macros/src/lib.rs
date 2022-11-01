use proc_macro::TokenStream;

mod menu;
mod link;
mod module;
mod view;
mod layout;

#[proc_macro_attribute]
pub fn view(_attr: TokenStream, item: TokenStream) -> TokenStream {
    view::view(_attr,item)
}

#[proc_macro]
pub fn section_menu(item: TokenStream) -> TokenStream {
    menu::section_menu(item)
}

#[proc_macro]
pub fn menu_group(item: TokenStream) -> TokenStream {
    menu::menu_group(item)
}

#[proc_macro]
pub fn menu_item(item: TokenStream) -> TokenStream {
    menu::menu_item(item)
}



#[proc_macro]
pub fn popup_menu(item: TokenStream) -> TokenStream {
    menu::popup_menu(item)
}

#[proc_macro]
pub fn popup_menu_link(item: TokenStream) -> TokenStream {
    menu::popup_menu_link(item)
}

#[proc_macro]
pub fn link(item: TokenStream) -> TokenStream {
    link::link_with_callback(item)
}
#[proc_macro]
pub fn menu_link(item: TokenStream) -> TokenStream {
    link::menu_link_with_callback(item)
}

#[proc_macro]
pub fn link_with_callback(item: TokenStream) -> TokenStream {
    link::link_with_callback(item)
}

#[proc_macro]
pub fn link_with_url(item: TokenStream) -> TokenStream {
    link::link_with_url(item)
}

// ~


#[proc_macro_attribute]
pub fn form(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Form,attr,item)
}

#[proc_macro_attribute]
pub fn stage(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Stage,attr,item)
}

#[proc_macro_attribute]
pub fn section(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Section,attr,item)
}    

#[proc_macro_attribute]
pub fn pane(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Pane,attr,item)
}    

#[proc_macro_attribute]
pub fn panel(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Panel,attr,item)
}

#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Page,attr,item)
}

#[proc_macro_attribute]
pub fn group(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Group, attr,item)
}

#[proc_macro_attribute]
pub fn html_layout(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::macro_handler(layout::Layout::Html, attr,item)
}

// ~


#[proc_macro]
pub fn declare_module(input: TokenStream) -> TokenStream {
    module::declare_module(input)
}

#[proc_macro_derive(Module, attributes(require_module))]
pub fn derive_module(input: TokenStream) -> TokenStream {
    module::derive_module(input)
}

