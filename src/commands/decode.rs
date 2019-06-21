use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};
use wasm_xbindgen_decoder_common as common;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Target {
        C,
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

    /// Uses verbose output.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Defines the output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

impl Into<common::options::Target> for Target {
    fn into(self) -> common::options::Target {
        match self {
            Target::C => common::options::Target::C,
        }
    }
}

impl Into<common::options::Options> for Decode {
    fn into(self) -> common::options::Options {
        common::options::Options::new(
            self.webassembly_module_file,
            self.target.into(),
            self.verbose,
            self.output,
        )
    }
}
