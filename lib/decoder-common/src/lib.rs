pub mod options;

use options::Options;
pub use walrus;
use walrus::{CustomSection, Module};

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

pub fn get_binding_section(module: &Module) -> Option<&dyn CustomSection> {
    module.customs.iter().find_map(|(_, section)| {
        if section.name() == BINDING_SECTION_NAME {
            Some(section)
        } else {
            None
        }
    })
}
