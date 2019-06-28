type $hello_webidl_type
  (func
    (result ByteString))

func-binding $hello_webidl_binding export 0 ;$hello_wasm_type; $hello_webidl_type
  (result
    (utf8-cstr ;targeted object =; ByteString ;offset =; 0))

bind 0 $hello_webidl_binding