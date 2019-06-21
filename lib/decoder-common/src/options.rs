use std::path::PathBuf;

#[derive(Debug)]
pub enum Target {
    C,
}

#[derive(Debug)]
pub struct Options {
    /// The WebAssembly module (`.wasm` file) containing the bindings.
    webassembly_module: PathBuf,

    /// The target for which the bindings are decoded.
    target: Target,

    /// Whether the output is verbose.
    verbose: bool,

    /// The output file.
    output: Option<PathBuf>,
}

impl Options {
    pub fn new(
        webassembly_module: PathBuf,
        target: Target,
        verbose: bool,
        output: Option<PathBuf>,
    ) -> Self {
        Self {
            webassembly_module,
            target,
            verbose,
            output,
        }
    }
}
