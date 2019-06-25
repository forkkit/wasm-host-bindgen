use std::{
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Options {
    /// The WebAssembly file name (`.wasm` file) that contains the bindings.
    pub webassembly_module_file: PathBuf,

    /// The WebAssembly module that contains the bindings.
    pub webassembly_module: Vec<u8>,

    /// The file containing the bindings.
    pub binding_file: PathBuf,

    /// The bindings.
    pub binding: String,

    /// Uses verbose output.
    pub verbose: bool,

    /// The output file.
    pub output_file: PathBuf,
}

impl Options {
    pub fn new(
        webassembly_module_file: PathBuf,
        binding_file: PathBuf,
        verbose: bool,
        output_file: Option<PathBuf>,
    ) -> io::Result<Self> {
        Ok(Self {
            webassembly_module_file: webassembly_module_file.clone(),
            webassembly_module: {
                let mut file = File::open(webassembly_module_file.clone())?;
                let mut buffer = Vec::new();

                file.read_to_end(&mut buffer)?;

                buffer
            },
            binding_file: binding_file.clone(),
            binding: {
                let mut file = File::open(binding_file)?;
                let mut buffer = String::new();

                file.read_to_string(&mut buffer)?;

                buffer
            },
            verbose,
            output_file: output_file.unwrap_or_else(|| {
                let mut path = webassembly_module_file;
                path.set_extension("bound.wasm");

                path
            }),
        })
    }
}
