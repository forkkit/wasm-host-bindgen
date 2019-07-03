from wasmer import Instance
import os
import hello_world_bindings

__dir__ = os.path.dirname(os.path.realpath(__file__))

wasm_bytes = open(__dir__ + '/hello_world.bound.wasm', 'rb').read()
instance = Instance(wasm_bytes)

result = instance.exports.hello()
print(result)

hello = hello_world_bindings.export_hello_builder(instance)
result = hello()

print(result)
