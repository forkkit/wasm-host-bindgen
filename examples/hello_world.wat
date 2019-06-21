(type $log_message_wasm_type (func (param i32) (result i32)))
(import "host" "log_message" (func $log_message (type $log_message_wasm_type)))

(memory $0 32)
(data (i32.const 0) "Hello, World!\00")
(export "main" (func $main))

(func $main (result i32)
  (call $log_message (i32.const 0)))