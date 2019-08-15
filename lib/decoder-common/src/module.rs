use crate::options::Options;
use std::{fs::File, io::prelude::*, path::PathBuf};
use walrus::{Module as WalrusModule, ModuleConfig};
use wasm_webidl_bindings::{ast::WebidlBindings, binary::decode};

#[derive(Debug)]
pub struct Module {
    module_bytes: Vec<u8>,
    module_configuration: ModuleConfig,
}

impl Module {
    pub fn new(path: &PathBuf) -> Result<Self, &'static str> {
        let mut file = File::open(path).map_err(|_| "Failed to open the file.")?;
        let mut module_bytes = Vec::new();

        file.read_to_end(&mut module_bytes)
            .map_err(|_| "Failed to read the file.")?;

        let module_configuration = ModuleConfig::default();

        Ok(Self {
            module_bytes,
            module_configuration,
        })
    }

    pub fn parse<Function>(mut self, handler: Function, options: &Options)
    where
        Function: 'static + Fn(bool, &WalrusModule, &WebidlBindings, &File) + Send + Sync,
    {
        let output = options.output.clone();
        let prelude = options.prelude;

        self.module_configuration
            .on_parse(move |module, indices_to_ids| {
                let section = module
                    .customs
                    .iter()
                    .find_map(|(_, section)| {
                        if section.name() == super::BINDING_SECTION_NAME {
                            Some(section)
                        } else {
                            None
                        }
                    })
                    .expect("No Web IDL bindings custom section in this WebAssembly module.");

                let data = section.data(&Default::default());
                let bindings = decode(indices_to_ids, &data).expect("Failed to decode bindings.");

                let mut writer =
                    File::create(output.clone()).expect("Failed to write the decoded bindings.");

                handler(prelude, &module, &bindings, &mut writer);

                Ok(())
            });

        self.module_configuration
            .parse(&self.module_bytes)
            .expect("Failed to parse the WebAssembly module.");
    }
}
