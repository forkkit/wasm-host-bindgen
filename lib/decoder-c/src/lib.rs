#![deny(warnings)]

use wasm_xbindgen_decoder_common::{
    self as common, options::Options, wasm_webidl_bindings::ast::WebidlBindings,
};

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Decoder for Decoder {
    fn decode(&self, options: Options) -> Result<(), &'static str> {
        options
            .webassembly_module
            .parse(|bindings: &WebidlBindings| {
                println!("{:?}", bindings);
            });
        /*
        match get_binding_section(&options.webassembly_module) {
            Some(section) => {
                /*
                let ids_to_indices = IdsToIndices::default();
                let reader = section.data(&ids_to_indices).into_owned();
                let indices_to_ids = IndicesToIds::default();

                println!("{:?}", reader);
                println!("{:?}", decode(&indices_to_ids, &mut reader.as_slice()));
                 */
                println!("{:?}", section);
            }

            None => return Err("No binding custom section found in the given module."),
        }
        */

        Ok(())
    }
}
