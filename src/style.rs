use crate::controls::*;

pub struct ControlStyle{
    //items:Vec<String>
}

impl ControlStyle{

    pub fn get()->Vec<&'static str>{
        Vec::from([
            mnemonic::CSS
        ])
    }

    pub fn get_str()->String{
        Self::get().join("\n")
    }
}
