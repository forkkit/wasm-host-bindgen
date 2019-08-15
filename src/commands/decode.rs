use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};
use wasm_host_bindgen_decoder_common as common;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Target {
        Python,
    }
}

#[derive(StructOpt, Debug)]
pub struct Decode {
    /// The WebAssembly module (`.wasm` file) containing the bindings.
    #[structopt(name = "WASM_MODULE", parse(from_os_str))]
    webassembly_module_file: PathBuf,

    /// Selects a target for which the bindings are decoded.
    #[structopt(
        short = "t",
        long = "target",
        raw(possible_values = r#"&Target::variants()"#),
        case_insensitive = true
    )]
    target: Target,

    /// Generates the prelude, i.e. the Web IDL types for the target.
    #[structopt(short = "p", long = "with-prelude")]
    prelude: bool,

    /// Uses verbose output.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Defines the output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
}

impl Into<common::options::Target> for Target {
    fn into(self) -> common::options::Target {
        match self {
            Target::Python => common::options::Target::Python,
        }
    }
}

impl Into<common::options::Options> for Decode {
    fn into(self) -> common::options::Options {
        common::options::Options::new(
            self.webassembly_module_file,
            self.target.into(),
            self.prelude,
            self.verbose,
            self.output,
        )
        .unwrap()
    }
}
