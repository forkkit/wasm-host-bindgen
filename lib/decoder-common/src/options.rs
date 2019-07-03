use crate::module::Module;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Target {
    Python,
}

#[derive(Debug)]
pub struct Options {
    /// The WebAssembly file name (`.wasm` file) containing the bindings.
    pub webassembly_module_file: PathBuf,

    /// The WebAssembly module, ready to be parsed.
    pub webassembly_module: Module,

    /// The target for which the bindings are decoded.
    pub target: Target,

    /// Whether the output is verbose.
    pub verbose: bool,

    /// The output that will receive the result.
    pub output: PathBuf,
}

impl Options {
    pub fn new(
        webassembly_module_file: PathBuf,
        target: Target,
        verbose: bool,
        output: PathBuf,
    ) -> Result<Self, &'static str> {
        Ok(Options {
            webassembly_module_file: webassembly_module_file.clone(),
            webassembly_module: Module::new(&webassembly_module_file)?,
            target,
            verbose,
            output,
        })
    }
}
