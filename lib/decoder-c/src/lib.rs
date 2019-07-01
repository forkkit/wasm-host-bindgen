//#![deny(warnings)]

use std::{fs::File, io::Write};
use wasm_xbindgen_decoder_common::{
    self as common,
    options::Options,
    walrus::{ExportItem, Function, FunctionId, FunctionKind, LocalFunction, Module, Type, TypeId},
    wasm_webidl_bindings::ast::{
        Bind, ExportBinding, FunctionBindingId, OutgoingBindingMap, WebidlBindings, WebidlFunction,
        WebidlTypeId, WebidlTypeRef,
    },
};

pub struct Decoder;

impl common::Decoder for Decoder {
    fn decode(options: Options) -> Result<(), &'static str> {
        options.webassembly_module.parse(decode, options.output);

        Ok(())
    }
}

fn decode(module: &Module, webidl_bindings: &WebidlBindings, mut writer: &File) {
    webidl_bindings.codegen(&mut CodegenContext {
        module,
        webidl_bindings,
        writer: &mut writer,
    });
}

trait Codegen {
    fn codegen(&self, context: &mut CodegenContext);
}

struct CodegenContext<'a> {
    module: &'a Module,
    webidl_bindings: &'a WebidlBindings,
    writer: &'a mut dyn Write,
}

impl<'a> CodegenContext<'a> {
    fn get_type(&self, id: TypeId) -> &Type {
        self.module.types.get(id)
    }

    fn get_function(&self, id: FunctionId) -> &Function {
        self.module.funcs.get(id)
    }
}

impl Codegen for WebidlBindings {
    fn codegen(&self, context: &mut CodegenContext) {
        // Iterate over `bind <wasm> <webidl>` statements.
        for bind in self.binds.iter().filter_map(|index| self.binds.get(*index)) {
            bind.codegen(context);
        }
    }
}

impl Codegen for Bind {
    fn codegen(&self, context: &mut CodegenContext) {
        dbg!(self);
        let module = context.module;
        let bindings = &context.webidl_bindings.bindings;

        // Bound Wasm function.
        let wasm_function = context.get_function(self.func);

        // Get the `func-binding` statement.
        let function_binding = bindings
            .get::<ExportBinding>(ExportBinding::wrap(self.binding))
            .expect("Web IDL AST is malformed.");

        // Get the bound Wasm function type.
        let function_binding_wasm_type = context.get_type(function_binding.wasm_ty);

        // Get the WebIDL function type.
        let function_binding_webidl_type = match function_binding.webidl_ty {
            WebidlTypeRef::Id(id) => context
                .webidl_bindings
                .types
                .get::<WebidlFunction>(WebidlFunction::wrap(id)),

            _ => panic!("ho no"),
        };

        dbg!(function_binding);
        dbg!(function_binding_wasm_type);
        dbg!(function_binding_webidl_type);
        dbg!(wasm_function);

        match &wasm_function.kind {
            // The bound Wasm function is an imported function.
            FunctionKind::Import(imported_function) => {
                let wasm_import = module.imports.get(imported_function.import);
                dbg!(wasm_import);
            }

            // The bound Wasm function is an exported function.
            FunctionKind::Local(LocalFunction {
                ty: exported_function_type_id,
                ..
            }) => {
                if function_binding_wasm_type != module.types.get(*exported_function_type_id) {
                    panic!("WebIDL type for the Wasm function, and the actual Wasm function type differ.");
                }

                let wasm_export_name = module
                    .exports
                    .iter()
                    .find_map(|export| match export.item {
                        ExportItem::Function(function_id) if function_id == wasm_function.id() => {
                            dbg!(&export.name);

                            Some(&export.name)
                        }

                        _ => None,
                    })
                    .expect("Failed to find an exported function.");

                write!(
                    context.writer,
                    r#"
exports_{name} = |instance: Instance| {{
    let pointer: u32 = instance.exports["{name}"]();
    let byte_string = ByteString::new();
    let memory = instance.memory.view::<u8>();

    match memory.position(|b| b != 0) {{
        Some(offset) => ByteString::from(memory[pointer..offset]),
        None => ByteString::from(memory[pointer..]),
    }}
}}
"#,
                    name = wasm_export_name
                )
                .unwrap();

                function_binding.result.codegen(context);
            }

            _ => panic!("Not implemented yet."),
        }
    }
}

impl Codegen for OutgoingBindingMap {
    fn codegen(&self, context: &mut CodegenContext) {
        write!(context.writer, "hello world").unwrap();
    }
}
