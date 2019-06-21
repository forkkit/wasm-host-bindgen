#![deny(warnings)]

mod commands;

use commands::Commands;
use structopt::StructOpt;
use wasm_xbindgen_decoder_common as common;
use wasm_xbindgen_encoder as encoder;

fn main() {
    match Commands::from_args() {
        Commands::Decode(command) => {
            let options: common::options::Options = command.into();
            println!("{:?}", options)
        }

        Commands::Encode(command) => {
            let options: encoder::options::Options = command.into();
            println!("{:?}\n\n", options);
            encoder::encode(&options);
        }
    }
}
