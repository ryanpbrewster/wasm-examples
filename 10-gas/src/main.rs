use wasmer_middleware_common::metering;
use wasmer_runtime::{compile_with, error, imports, Compiler, MiddlewareChain, StreamingCompiler};

static WASM: &'static [u8] = include_bytes!("../fibonacci_bg.wasm");

fn main() -> error::Result<()> {
    let import_object = imports! {};

    let module = compile_with(&WASM, &get_metered_compiler(1_000_000)).unwrap();
    let instance = module.instantiate(&import_object)?;

    for n in 1.. {
        let result = instance.call("fibrec", &[n.into()])?;
        let gas = metering::get_points_used(&instance);
        println!(
            "consumed {} to compute fib({}) = {}",
            gas,
            n,
            result[0].to_u128()
        );
    }

    Ok(())
}

pub fn get_metered_compiler(limit: u64) -> impl Compiler {
    use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
    let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
        let mut chain = MiddlewareChain::new();
        chain.push(metering::Metering::new(limit));
        chain
    });
    c
}
