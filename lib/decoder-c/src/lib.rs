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
    fn decode(options: Options) -> Result<(), &'static str> {
        options
            .webassembly_module
            .parse(|bindings: &WebidlBindings| {
                println!("{:#?}", bindings);
            });

        Ok(())
    }
}
