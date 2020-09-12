use wasmer_middleware_common::metering;
use wasmer_runtime::{compile_with, error, imports, Compiler, MiddlewareChain, StreamingCompiler, Value};

static WASM: &'static [u8] = include_bytes!("../policy/pkg/policy_bg.wasm");

fn main() -> error::Result<()> {
    let import_object = imports! {};

    let module = compile_with(&WASM, &get_metered_compiler(1_000_000)).unwrap();
    let mut instance = module.instantiate(&import_object)?;

    let host_string = r#"{"auth":{"uid":"foo"}}"#;
    // Write the string into the lineary memory
    let memory = instance.context_mut().memory(0);
    for (byte, cell) in host_string
        .bytes()
        .zip(memory.view()[0 as usize..(host_string.len()) as usize].iter())
    {
        cell.set(byte);
    }

    let result = instance.call(
        "allow",
        &[Value::I32(0), Value::I32(host_string.len() as _)],
    )?;


    let gas = metering::get_points_used(&instance);
    println!(
        "consumed {} to compute allow({}) = {:?}",
        gas,
        host_string,
        result,
    );

    Ok(())
}

pub fn get_metered_compiler(limit: u64) -> impl Compiler {
    use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
    StreamingCompiler::<SinglePassMCG, _, _, _, _>::new(move || {
        let mut chain = MiddlewareChain::new();
        chain.push(metering::Metering::new(limit));
        chain
    })
}
