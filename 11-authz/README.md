Executing WebAssembly code can be dangerous even if you don't bind any system APIs. Fully sandboxed code
can still exhaust system resources (CPU, memory, etc).

Metering (aka "gas") is a way of instrumenting WebAssembly code before executing it. The instrumentation
imposes limits on how many instructions can be executed.
