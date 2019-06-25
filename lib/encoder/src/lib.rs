pub mod options;

use options::Options;
use walrus::ModuleConfig;
use wasm_webidl_bindings::text;

pub fn encode(options: &Options) -> Result<(), &'static str> {
    let mut configuration = ModuleConfig::default();
    let binding = options.binding.clone();

    configuration.on_parse(move |module, indices_to_ids| {
        let webidl_bindings = text::parse(module, indices_to_ids, &binding)?;

        module.customs.add(webidl_bindings);

        Ok(())
    });

    let module = configuration
        .parse(&options.webassembly_module)
        .map_err(|_| "Failed to parse the WebAssembly bytes.")?;
    module
        .emit_wasm_file(&options.output_file)
        .map_err(|_| "Failed to write the WebAssembly module.")?;

    Ok(())
}
