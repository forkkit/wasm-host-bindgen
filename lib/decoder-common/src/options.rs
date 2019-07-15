use std::path::PathBuf;

#[derive(Debug)]
pub enum Target {
    Python,
}

#[derive(Debug)]
pub enum Emit {
    Ast,
}

#[derive(Debug)]
pub struct Options {
    /// The WebAssembly file name (`.wasm` file) containing the bindings.
    pub webassembly_module_file: PathBuf,

    /// The target for which the bindings are decoded.
    pub target: Target,

    /// Alternative data to emit.
    pub emit: Option<Emit>,

    /// Whether the output must include the prelude, i.e. the Web IDL
    /// types API.
    pub prelude: bool,

    /// Whether the output is verbose.
    pub verbose: bool,

    /// The output that will receive the result.
    pub output: PathBuf,
}

impl Options {
    pub fn new(
        webassembly_module_file: PathBuf,
        target: Target,
        emit: Option<Emit>,
        prelude: bool,
        verbose: bool,
        output: PathBuf,
    ) -> Result<Self, &'static str> {
        Ok(Options {
            webassembly_module_file: webassembly_module_file.clone(),
            target,
            emit,
            prelude,
            verbose,
            output,
        })
    }
}
