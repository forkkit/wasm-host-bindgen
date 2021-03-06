#![deny(warnings)]

pub mod module;
pub mod options;

use options::Options;
pub use walrus;
pub use wasm_webidl_bindings;

pub trait Decoder {
    fn decode(_: &Options) -> Result<(), &'static str>;
}

pub fn decode<D>(options: &Options) -> Result<(), &'static str>
where
    D: Decoder,
{
    D::decode(&options)
}

pub const BINDING_SECTION_NAME: &str = "webidl-bindings";
