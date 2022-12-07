use crate::controls::*;

pub struct ControlStyle{
    //items:Vec<String>
}

const CSS:&'static str = include_str!("style.css");

impl ControlStyle{

    pub fn get()->Vec<&'static str>{
        Vec::from([
            mnemonic::CSS,
            crate::menu::CSS,
            crate::pagination::CSS,
            crate::dialog::CSS,
            CSS
        ])
    }

    pub fn get_str()->String{
        Self::get().join("\n")
    }
}
