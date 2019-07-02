//#![deny(warnings)]

use std::{fs::File, io::Write};
use wasm_xbindgen_decoder_common::{
    self as common,
    options::Options,
    walrus::{ExportItem, Function, FunctionId, FunctionKind, LocalFunction, Module, Type, TypeId},
    wasm_webidl_bindings::ast::{
        Bind, ExportBinding, FunctionBindingId, FunctionBindings, ImportBinding,
        OutgoingBindingExpression, OutgoingBindingExpressionUtf8CStr, OutgoingBindingMap,
        WebidlBindings, WebidlFunction, WebidlTypeId, WebidlTypeRef, WebidlTypes,
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
    let binds = &webidl_bindings.binds;

    for bind in binds.iter().filter_map(|index| binds.get(*index)) {
        bind.codegen(&mut CodegenContext {
            module,
            writer: &mut writer,
            bind,
            bindings: &webidl_bindings.bindings,
            types: &webidl_bindings.types,
        });
    }
}

trait Codegen {
    fn codegen(&self, context: &mut CodegenContext);
}

struct CodegenContext<'a> {
    module: &'a Module,
    writer: &'a mut dyn Write,
    bind: &'a Bind,
    bindings: &'a FunctionBindings,
    types: &'a WebidlTypes,
}

impl<'a> CodegenContext<'a> {
    fn get_type(&self, id: TypeId) -> &Type {
        self.module.types.get(id)
    }

    fn get_function(&self, id: FunctionId) -> &Function {
        self.module.funcs.get(id)
    }
}

// Represent the `bind <wasm-function> <webidl-function-binding>` statement.
impl Codegen for Bind {
    fn codegen(&self, context: &mut CodegenContext) {
        let module = context.module;

        /*
        // Get the WebIDL function type.
         */

        // Bound Wasm function.
        let wasm_function = context.get_function(self.func);

        // Is it a binding to an export, and is the Wasm function an export?
        if let (
            Some(export_binding),
            FunctionKind::Local(LocalFunction {
                ty: exported_function_type_id,
                ..
            }),
        ) = (
            context
                .bindings
                .get::<ExportBinding>(ExportBinding::wrap(self.binding)),
            &wasm_function.kind,
        ) {
            let export_binding_wasm_type = context.get_type(export_binding.wasm_ty);
            let exported_function_type = module.types.get(*exported_function_type_id);

            if export_binding_wasm_type != exported_function_type {
                panic!(
                    "WebIDL type for the Wasm function, and the actual Wasm function type differ."
                );
            }

            export_binding.codegen(context);
        }
        // Is it a binding to an import, and is the Wasm function an import?
        else if let (Some(import_binding), FunctionKind::Import(imported_function)) = (
            context
                .bindings
                .get::<ImportBinding>(ImportBinding::wrap(self.binding)),
            &wasm_function.kind,
        ) {
            let wasm_import = context.module.imports.get(imported_function.import);

            dbg!(import_binding);
            dbg!(wasm_import);
        }
        // Mismatch.
        else {
            unimplemented!()
        }
    }
}

impl Codegen for ExportBinding {
    fn codegen(&self, context: &mut CodegenContext) {
        assert!(self.params.bindings.is_empty());
        assert!(self.result.bindings.len() == 1);

        dbg!(self);

        // Get the `type` statement that represents the Web IDL binding function.
        let export_type = match self.webidl_ty {
            WebidlTypeRef::Id(id) => context
                .types
                .get::<WebidlFunction>(WebidlFunction::wrap(id)),
            _ => unimplemented!(),
        };

        // Check that the `type` and the `func-binding` types match.
        match (export_type, &self.result.bindings[0]) {
            (
                Some(WebidlFunction {
                    result: Some(left_type),
                    ..
                }),
                OutgoingBindingExpression::Utf8CStr(OutgoingBindingExpressionUtf8CStr {
                    ty: right_type,
                    ..
                }),
            ) => {
                if left_type != right_type {
                    panic!("Type mimatch between `type` and `func-binding`.");
                }
            }

            _ => unimplemented!(),
        };

        let wasm_function_id = context.get_function(context.bind.func).id();
        let wasm_export = context
            .module
            .exports
            .iter()
            .find(|export| match export.item {
                ExportItem::Function(function_id) if function_id == wasm_function_id => true,
                _ => false,
            })
            .expect("Bound exported function is not found.");

        dbg!(wasm_export);

        write!(
            context.writer,
            r#"
export_{name} = |instance: Instance| {{
    let pointer "#,
            name = wasm_export.name
        )
        .unwrap();
    }
}

impl Codegen for OutgoingBindingMap {
    fn codegen(&self, context: &mut CodegenContext) {
        write!(context.writer, "hello world").unwrap();
    }
}
