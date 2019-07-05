type $hello_webidl_type
  (func
    (result DOMString))

func-binding $hello_webidl_binding export ;$hello_wasm_type; 0 $hello_webidl_type
  (result
    (utf8-str ;targeted object =; DOMString ;offset-index =; 0 ;length-index =; 1))

bind ;$hello; 0 $hello_webidl_binding