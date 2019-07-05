(module
  (type $hello_wasm_type (func (result i32)))
  
  (memory 128)
  
  (func $hello (type $hello_wasm_type) (result i32)
    i32.const 0
    i32.const 32
    i32.store
    i32.const 1
    i32.const 13
    i32.store
    i32.const 0)
  
  (export "memory" (memory 0))
  (export "hello" (func $hello))
  (data (i32.const 32) "Hello, World!"))