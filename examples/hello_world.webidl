type $hello_webidl_type
  (func
    (result ByteString))

func-binding $hello_webidl_binding export ;$hello_wasm_type; 0 $hello_webidl_type
  (result
    (utf8-cstr ;targeted object =; ByteString ;offset =; 0))

bind ;$hello; 0 $hello_webidl_binding