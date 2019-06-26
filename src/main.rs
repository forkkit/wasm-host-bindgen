#![deny(warnings)]

mod commands;

use commands::Commands;
use structopt::StructOpt;
use wasm_xbindgen_decoder_c as c;
use wasm_xbindgen_decoder_common::{
    decode,
    options::{Options as DecoderOptions, Target},
};
use wasm_xbindgen_encoder::encode;

fn main() -> Result<(), &'static str> {
    match Commands::from_args() {
        Commands::Decode(command) => {
            let options: DecoderOptions = command.into();

            match options.target {
                Target::C => decode::<c::Decoder>(options)?,
            };
        }

        Commands::Encode(command) => {
            let options = command.into();
            println!("{:#?}", options);
            encode(&options)?;
        }
    }

    Ok(())
}
