pub mod options;

use options::Options;
use walrus::ModuleConfig;
use wasm_webidl_bindings::text;

pub fn encode(options: &Options) -> () {
    let mut configuration = ModuleConfig::default();
    let binding = options.binding.clone();

    configuration.on_parse(move |module, indices_to_ids| {
        let webidl_bindings = text::parse(module, indices_to_ids, &binding)?;

        println!("The parsed Web IDL bindings are {:#?}", webidl_bindings);

        module.customs.add(webidl_bindings);

        Ok(())
    });

    let module = configuration.parse(&options.webassembly_module).unwrap();
    module.emit_wasm_file(&options.output_file).unwrap();

    ()
}
