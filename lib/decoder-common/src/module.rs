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

    pub fn parse<F>(mut self, handler: F)
    where
        F: 'static + Fn(&WalrusModule, &WebidlBindings) + Send + Sync,
    {
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
                    .unwrap();

                let data = section.data(&Default::default());
                let bindings = decode(indices_to_ids, &data).unwrap();

                handler(&module, &bindings);

                Ok(())
            });

        self.module_configuration.parse(&self.module_bytes).unwrap();
    }
}
