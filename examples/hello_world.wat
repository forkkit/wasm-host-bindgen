(module
  (type $hello_wasm_type (func (result i32)))
  
  (memory 32)
  
  (func $hello (type $hello_wasm_type) (result i32)
    i32.const 0)
  
  (export "memory" (memory 0))
  (export "hello" (func $hello))
  (data (i32.const 0) "Hello, World!\00"))