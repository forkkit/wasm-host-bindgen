type $log_message_webidl_type
    (func
      (method any)
      (param ByteString Uint8Array))

func-binding $encodeIntoBinding import 0 ;$log_message_wasm_type; $log_message_webidl_type
    (param
        (utf8-cstr ByteString 0))
    (result
        (as i32 (get 0)))

bind 0 $encodeIntoBinding