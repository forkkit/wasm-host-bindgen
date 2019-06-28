//#![deny(warnings)]

use wasm_xbindgen_decoder_common::{
    self as common,
    options::Options,
    walrus::{FunctionKind, Module},
    wasm_webidl_bindings::ast::{
        FunctionBindingId, ImportBinding, WebidlBindings, WebidlFunction, WebidlTypeId,
        WebidlTypeRef,
    },
};

pub struct Decoder;

impl common::Decoder for Decoder {
    fn decode(options: Options) -> Result<(), &'static str> {
        options.webassembly_module.parse(decode);

        Ok(())
    }
}

fn decode(module: &Module, webidl_bindings: &WebidlBindings) {
    dbg!(webidl_bindings);

    let binds = &webidl_bindings.binds;
    let bindings = &webidl_bindings.bindings;

    for bind in binds.iter().filter_map(|index| binds.get(*index)) {
        let wasm_function = module.funcs.get(bind.func);
        let function_binding = bindings
            .get::<ImportBinding>(ImportBinding::wrap(bind.binding))
            .expect("Web IDL AST is malformed.");

        let function_binding_wasm_type = module.types.get(function_binding.wasm_ty);
        let function_binding_webidl_type = match function_binding.webidl_ty {
            WebidlTypeRef::Id(id) => webidl_bindings
                .types
                .get::<WebidlFunction>(WebidlFunction::wrap(id)),

            _ => panic!("ho no"),
        };

        dbg!(function_binding);
        dbg!(function_binding_wasm_type);
        dbg!(function_binding_webidl_type);

        match &wasm_function.kind {
            FunctionKind::Import(imported_function) => {
                let wasm_import = module.imports.get(imported_function.import);
                dbg!(wasm_import);
            }
            _ => panic!("Not implemented yet."),
        }
    }
}
