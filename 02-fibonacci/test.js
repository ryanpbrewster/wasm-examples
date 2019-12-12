const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("./pkg/fibonacci_bg.wasm");
  const compiled = await WebAssembly.compile(bytes);
  const mod = await WebAssembly.instantiate(compiled);
  console.log(`fibrec(10) = ${mod.exports.fibrec(10)}`);
  console.log(`fibiter(10) = ${mod.exports.fibiter(10)}`);
}

main();
