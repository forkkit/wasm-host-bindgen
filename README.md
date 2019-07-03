# Example

```sh
$ # Compile the `.wat` file to a `.wasm` binary module.
$ wat2wasm examples/hello_world.wat -o examples/hello_world.wasm

$ # Compile `wasm-xbindgen`.
$ cargo build

$ # Insert/Encode the Web IDL bindings into the `.wasm` binary module.
$ # Usually, this step isn't necessary. It aims to be done by a
$ # compiler (e.g. `rustc`). However, right now, the WebAssembly Web IDL
$ # Bindings specification and toolchain are early drafts, making this step
$ # is necessary.
$ ./target/debug/wasm-xbindgen \
      encode \
          examples/hello_world.wasm \
          examples/hello_world.webidl

$ # Decode the Web IDL bindings (that are stored inside the WebAssembly binary module).
$ # Decoding targets Python.
$ ./target/debug/wasm-xbindgen \
      decode \
          --target python \
          --prelude \
          --output examples/hello_world_bindings.py \
          examples/hello_world.bound.wasm

$ # Assuming `wasmer` is installed, it is possible to run the
$ # WebAssembly module with bindings.
$ python examples/hello_world.py
Hello, World!
```

