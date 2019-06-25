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

pub fn get_binding_custom_section(module: &Module) -> Option<&dyn CustomSection> {
    module.customs.iter().find_map(|(_, section)| {
        if section.name() == "webidl-bindings" {
            Some(section)
        } else {
            None
        }
    })
}
