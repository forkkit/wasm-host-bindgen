#![deny(warnings)]

mod commands;

use commands::Commands;
use std::{fs::File, io::prelude::*};
use structopt::StructOpt;
use wasm_host_bindgen_decoder_common::{
    decode,
    module::Module,
    options::{Emit, Options as DecoderOptions, Target},
};
use wasm_host_bindgen_decoder_python as python;
use wasm_host_bindgen_encoder::encode;

fn main() -> Result<(), &'static str> {
    match Commands::from_args() {
        Commands::Decode(command) => {
            let options: DecoderOptions = command.into();

            if let Some(emit_type) = &options.emit {
                match emit_type {
                    Emit::Ast => {
                        let mut path = options.webassembly_module_file.clone();
                        path.set_extension("emit.ast");

                        let module = Module::new(&options.webassembly_module_file)?;
                        module.parse(
                            move |_, _, webidl_bindings, _| {
                                let mut file = File::create(&path).unwrap();

                                write!(file, "{:#?}", webidl_bindings).unwrap();
                            },
                            &options,
                        );
                    }
                }
            }

            match &options.target {
                Target::Python => decode::<python::Decoder>(&options)?,
            }
        }

        Commands::Encode(command) => {
            let options = command.into();
            encode(&options)?;
            println!("{:?}", &options.output_file);
        }
    }

    Ok(())
}
