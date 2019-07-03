from wasmer import Instance
import os
import hello_world_bindings as bindings

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/hello_world.bound.wasm', 'rb').read()
instance = Instance(wasm_bytes)

exports = bindings.bind_exports(instance)
print(exports.hello())
