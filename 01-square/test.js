const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("./pkg/square_bg.wasm");
  const compiled = await WebAssembly.compile(bytes);
  const mod = await WebAssembly.instantiate(compiled);
  console.log(`square(4) = ${mod.exports.square(4)}`);
}

main();
