#![deny(warnings)]

pub mod module;
pub mod options;

use options::Options;
pub use walrus;
pub use wasm_webidl_bindings;

pub trait Decoder {
    fn decode(&self, _: Options) -> Result<(), &'static str>;
}

pub fn decode<D>(options: Options, decoder: D) -> Result<(), &'static str>
where
    D: Decoder,
{
    decoder.decode(options)
}

pub const BINDING_SECTION_NAME: &'static str = "webidl-bindings";
