use std::path::PathBuf;
use walrus::Module;

#[derive(Debug)]
pub enum Target {
    C,
}

#[derive(Debug)]
pub struct Options {
    /// The WebAssembly file name (`.wasm` file) containing the bindings.
    pub webassembly_module_file: PathBuf,

    /// The WebAssembly module that contains the bindings.
    pub webassembly_module: Module,

    /// The target for which the bindings are decoded.
    pub target: Target,

    /// Whether the output is verbose.
    pub verbose: bool,

    /// The output file.
    pub output: Option<PathBuf>,
}

impl Options {
    pub fn new(
        webassembly_module_file: PathBuf,
        target: Target,
        verbose: bool,
        output: Option<PathBuf>,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            webassembly_module_file: webassembly_module_file.clone(),
            webassembly_module: {
                walrus::Module::from_file(webassembly_module_file)
                    .map_err(|_| "Invalid WebAssembly module.")?
            },
            target,
            verbose,
            output,
        })
    }
}
