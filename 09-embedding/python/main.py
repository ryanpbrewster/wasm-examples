from wasmer import Instance

wasm_bytes = open('my_lib.wasm', 'rb').read()
instance = Instance(wasm_bytes)
result = instance.exports.sum_of_squares(100)

print(result)

