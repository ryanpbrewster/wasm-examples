use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config)?;
    let store = Store::new(&engine);
    store.add_fuel(1_000_000)?;

    let module = Module::from_file(store.engine(), "../fibonacci.wasm")?;
    let instance = Instance::new(&store, &module, &[])?;
    let entrypoint = instance.get_typed_func::<i32, i32>("fibonacci")?;

    for n in 1.. {
        let before = store.fuel_consumed().unwrap();
        let result = entrypoint.call(n)?;
        let after = store.fuel_consumed().unwrap();
        println!(
            "consumed {} to compute fib({}) = {}",
            after - before,
            n,
            result,
        );
        store.add_fuel(after - before)?;
    }

    Ok(())
}
