use std::path::PathBuf;
use structopt::StructOpt;
use wasm_xbindgen_encoder as encoder;

#[derive(StructOpt, Debug)]
pub struct Encode {
    /// The WebAssembly module (`.wasm` file) that will contain the bindings.
    #[structopt(name = "WASM_MODULE", parse(from_os_str))]
    webassembly_module_file: PathBuf,

    /// The file name containing the bindings.
    #[structopt(name = "BINDING", parse(from_os_str))]
    binding_file: PathBuf,

    /// Uses verbose output.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Defines the output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

impl Into<encoder::options::Options> for Encode {
    fn into(self) -> encoder::options::Options {
        encoder::options::Options::new(
            self.webassembly_module_file,
            self.binding_file,
            self.verbose,
            self.output,
        )
        .unwrap()
    }
}
