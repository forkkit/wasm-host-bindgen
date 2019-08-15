//! Define all the commands.

pub(crate) mod decode;
pub(crate) mod encode;

use structopt::StructOpt;

/// WebAssembly bindings toolkit (`wasm-xbinden`) is a set of tool to
/// encode bindings into a WebAssembly module, or to decode
/// WebAssembly bindings into multiple targets.
#[derive(StructOpt, Debug)]
#[structopt(name = "wasm-host-bindgen")]
pub enum Commands {
    /// Decode WebAssembly bindings to multiple targets.
    #[structopt(name = "decode")]
    Decode(decode::Decode),

    /// Encode bindings into a WebAssembly module.
    #[structopt(name = "encode")]
    Encode(encode::Encode),
}
