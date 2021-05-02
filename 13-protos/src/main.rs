use prost::Message;
use wasmtime::*;

fn main() -> anyhow::Result<()> {
    let my_input = proto::MyInput {
        my_bool: true,
        my_i32: 45,
        my_f32: 3.14,
        my_string: "hello, world".to_owned(),
        my_bytes: vec![3, 1, 4, 1, 5, 9, 2, 6],
        records: vec![],
    };
    println!("{:?}", my_input);

    // Create our `Store` context and then compile a module and create an
    // instance from the compiled module all in one go.
    let wasmtime_store = Store::default();
    let module = Module::from_file(wasmtime_store.engine(), "policy/pkg/policy_bg.wasm")?;
    let instance = Instance::new(&wasmtime_store, &module, &[])?;

    // Load up our exports from the instance
    let entrypoint = instance.get_typed_func::<(i32, i32), i32>("entrypoint")?;
    println!("Found entrypoint...");
    let memory = instance
        .get_memory("memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;
    println!("Found memory...");
    let mut buf = Vec::with_capacity(my_input.encoded_len());
    my_input.encode(&mut buf)?;
    memory.write(0, &buf)?;
    println!("Wrote input to memory...");
    let outcome = entrypoint.call((0, buf.len() as i32))?;
    println!("outcome = {:?} ({})", proto::Outcome::from_i32(outcome), outcome);
    Ok(())
}

mod proto {
    include!(concat!(env!("OUT_DIR"), "/rpb.example.rs"));
}
