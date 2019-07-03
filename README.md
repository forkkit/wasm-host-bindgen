# Examples

```sh
$ cargo build
$ ./target/debug/wasm-xbindgen encode examples/hello_world.wasm examples/hello_world.webidl
$ ./target/debug/wasm-xbindgen decode --target python --prelude --output examples/hello_world_bindings.py examples/hello_world.bound.wasm
$ python examples/hello_world.py
Hello, World!
```
