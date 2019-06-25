use wasm_xbindgen_decoder_common::{self as common, get_binding_section, options::Options};

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Decoder for Decoder {
    fn decode(&self, options: Options) -> Result<(), &'static str> {
        match get_binding_section(&options.webassembly_module) {
            Some(section) => {
                println!("{:?}", section);
            }

            None => return Err("No binding custom section found in the given module."),
        }
        Ok(())
    }
}
