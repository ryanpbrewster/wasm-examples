WASM understands i32, i64, f32, and f64. Passing data into a WASM module is easy if it's
any fixed combination of those data types.

If you want to pass in something else (e.g., a string, or a list, or a complex
nested struct)...it's a bit more work.  AFAICT, the way to do this today is to
serialize the data into linear memory, then pass a (pointer, length) pair into
WASM and have it reconstruct the data from the serialized format.

This requires the host environment and the WASM module to agree on a binary format,
and it's moderately expensive to serialize/deserialize the data.

This directory is an example of passing in highly structured data by serializing
it using protobufs.
