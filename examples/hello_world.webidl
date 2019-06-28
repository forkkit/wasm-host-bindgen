type $log_message_webidl_type
    (func
      (method any)
      (param ByteString Uint8Array))

func-binding $log_message_webidl_binding import 0 ;$log_message_wasm_type; $log_message_webidl_type
    (param
        (utf8-cstr ByteString 0))

bind 0 $log_message_webidl_binding