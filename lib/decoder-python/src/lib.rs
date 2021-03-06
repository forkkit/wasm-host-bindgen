//#![deny(warnings)]

use std::{fs::File, io::Write};
use wasm_host_bindgen_decoder_common::{
    self as common,
    module::Module as CommonModule,
    options::Options,
    walrus::{ExportItem, Function, FunctionId, FunctionKind, Module, Type, TypeId},
    wasm_webidl_bindings::ast::{
        Bind, ExportBinding, FunctionBindingId, FunctionBindings, ImportBinding,
        OutgoingBindingExpression, OutgoingBindingExpressionCopy, OutgoingBindingExpressionUtf8Str,
        WebidlBindings, WebidlFunction, WebidlScalarType, WebidlTypeId, WebidlTypeRef, WebidlTypes,
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

    writeln!(
        writer,
        r#"
export_builders = {{}};
import_builders = {{}};"#
    )
    .unwrap();

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
        if let (Some(export_binding), FunctionKind::Local(local_function)) = (
            context
                .bindings
                .get::<ExportBinding>(ExportBinding::wrap(self.binding)),
            &wasm_function.kind,
        ) {
            let export_binding_wasm_type = context.get_type(export_binding.wasm_ty);
            let exported_function_type = module.types.get(local_function.ty());

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

        let wasm_output_type = match context.get_type(self.wasm_ty).results() {
            &[first] => first,
            &[] => return, /* The exported function returns nothing, there is nothing to bind. */
            slice => slice[0],
        };

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
                OutgoingBindingExpression::Utf8Str(OutgoingBindingExpressionUtf8Str {
                    ty: right_type,
                    ..
                }),
            )
            | (
                Some(left_type),
                OutgoingBindingExpression::Copy(OutgoingBindingExpressionCopy {
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

        let export_name = {
            let wasm_function_id = context.get_function(context.bind.func).id();

            context
                .module
                .exports
                .iter()
                .find(|export| match export.item {
                    ExportItem::Function(function_id) if function_id == wasm_function_id => true,
                    _ => false,
                })
                .map(|export| &export.name)
                .expect("Bound exported function is not found.")
        };

        writeln!(
            context.writer,
            r#"
def export_{name}_builder(instance):
    def export_{name}(*arguments):
        # output: {output_type}
        output = instance.exports.{name}(*arguments)"#,
            name = export_name,
            output_type = wasm_output_type,
        )
        .unwrap();

        self.result.bindings[0].codegen(context);

        writeln!(
            context.writer,
            r#"
    return export_{name}

export_builders['{name}'] = export_{name}_builder"#,
            name = export_name,
        )
        .unwrap();
    }
}

impl Codegen for OutgoingBindingExpression {
    fn codegen(&self, context: &mut CodegenContext) {
        match self {
            // utf8-str
            OutgoingBindingExpression::Utf8Str(OutgoingBindingExpressionUtf8Str {
                ty,
                offset: offset_index,
                length: length_index,
            }) => match ty {
                WebidlTypeRef::Scalar(WebidlScalarType::DomString) => writeln!(
                    context.writer,
                    r#"
        pointer = output
        offset_index = {offset_index}
        length_index = {length_index}

        memory = instance.memory.uint8_view(pointer)
        offset = memory[pointer + offset_index]
        length = memory[pointer + length_index]

        return domstring(memory[offset : offset + length])"#,
                    offset_index = offset_index,
                    length_index = length_index,
                )
                .unwrap(),

                _ => unimplemented!("outgoing binding expression type not supported"),
            },

            // copy
            OutgoingBindingExpression::Copy(OutgoingBindingExpressionCopy {
                ty,
                offset: offset_index,
                length: length_index,
            }) => match ty {
                WebidlTypeRef::Scalar(WebidlScalarType::ByteString) => writeln!(
                    context.writer,
                    r#"
        pointer = output
        offset_index = {offset_index}
        length_index = {length_index}

        memory = instance.memory.uint8_view(pointer)
        offset = memory[pointer + offset_index]
        length = memory[pointer + length_index]

        return bytestring(memory[offset : offset + length])"#,
                    offset_index = offset_index,
                    length_index = length_index,
                )
                .unwrap(),

                _ => unimplemented!("outgoing binding expression type not supported"),
            },

            _ => unimplemented!("outgoing binding expression not supported"),
        };
    }
}
