//#![deny(warnings)]

use std::{fs::File, io::Write};
use wasm_xbindgen_decoder_common::{
    self as common,
    module::Module as CommonModule,
    options::Options,
    walrus::{ExportItem, Function, FunctionId, FunctionKind, LocalFunction, Module, Type, TypeId},
    wasm_webidl_bindings::ast::{
        Bind, ExportBinding, FunctionBindingId, FunctionBindings, ImportBinding,
        OutgoingBindingExpression, OutgoingBindingExpressionUtf8CStr, WebidlBindings,
        WebidlFunction, WebidlScalarType, WebidlTypeId, WebidlTypeRef, WebidlTypes,
    },
};

pub struct Decoder;

impl common::Decoder for Decoder {
    fn decode(options: &Options) -> Result<(), &'static str> {
        let module = CommonModule::new(&options.webassembly_module_file)?;
        module.parse(decode, &options);

        Ok(())
    }
}

const PRELUDE: &str = include_str!("./prelude.py");

fn decode(prelude: bool, module: &Module, webidl_bindings: &WebidlBindings, mut writer: &File) {
    if prelude {
        write!(writer, "{}", PRELUDE).unwrap();
    }

    let binds = &webidl_bindings.binds;

    for (_, bind) in binds.iter() {
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

        let wasm_function_id = context.get_function(context.bind.func).id();
        let export_name = context
            .module
            .exports
            .iter()
            .find(|export| match export.item {
                ExportItem::Function(function_id) if function_id == wasm_function_id => true,
                _ => false,
            })
            .map(|export| &export.name)
            .expect("Bound exported function is not found.");

        // Get the `type` statement that represents the Web IDL binding function.
        let webidl_output_type = match self.webidl_ty {
            WebidlTypeRef::Id(id) => context
                .types
                .get::<WebidlFunction>(WebidlFunction::wrap(id))
                .and_then(|webidl_type| webidl_type.result.as_ref()),
            _ => unimplemented!(),
        };

        // Check that the `type` and the `func-binding` types match.
        //
        // For instance, in the following code, we have to check that
        // `$hello_webidl_type` is a function that returns
        // `ByteString`, _and_ that `$hello_webidl_binding` is a
        // function binding that returns a `ByteString` too (in this
        // case, that `utf8-cstr` returns a `ByteString`).
        //
        // ```
        // type $hello_webidl_type
        //   (func
        //     (result ByteString))
        //
        // func-binding $hello_webidl_binding export $hello_wasm_type $hello_webidl_type
        //   (result
        //     (utf8-cstr ByteString 0))
        // ```
        //
        // TODO: This type checker step should land in `wasm-webidl-bindings` directly.
        match (webidl_output_type, &self.result.bindings[0]) {
            (
                Some(left_type),
                OutgoingBindingExpression::Utf8CStr(OutgoingBindingExpressionUtf8CStr {
                    ty: right_type,
                    ..
                }),
            ) => {
                if left_type != right_type {
                    panic!(format!("Type mimatch between `type` and `func-binding`: `{left:?}` is not equal to `{right:?}`", left = left_type, right = right_type));
                }
            }

            _ => unimplemented!(),
        };

        let wasm_output_type = match context.get_type(self.wasm_ty).results() {
            &[first] => Some(first),
            &[] => None,
            slice => Some(slice[0]),
        };

        match wasm_output_type {
            Some(output_type) => write!(
                context.writer,
                r#"
export_{name} = |...arguments| {{
    let output: {output_type} = original_export_{name}(arguments...);
"#,
                name = export_name,
                output_type = output_type
            )
            .unwrap(),
            _ => unimplemented!(),
        };

        &self.result.bindings[0].codegen(context);
    }
}

impl Codegen for OutgoingBindingExpression {
    fn codegen(&self, context: &mut CodegenContext) {
        match self {
            // utf8-cstr
            OutgoingBindingExpression::Utf8CStr(OutgoingBindingExpressionUtf8CStr {
                ty,
                offset,
            }) => match ty {
                WebidlTypeRef::Scalar(WebidlScalarType::ByteString) => write!(
                    context.writer,
                    r#"
    let byte_string = ByteString::new();
    let offset = output + {offset};

    match memory[offset..].position(|b| b != 0) {{
        Some(end_offset) => ByteString::from(memory[offset..end_offset]),
        None => ByteString::from(memory[offset..]),
    }}

    byte_string
}}
"#,
                    offset = offset
                )
                .unwrap(),

                _ => unimplemented!(),
            },

            _ => unimplemented!(),
        };
    }
}
