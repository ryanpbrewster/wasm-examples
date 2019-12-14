const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("./pkg/add_bg.wasm");
  const compiled = await WebAssembly.compile(bytes);
  const mod = await WebAssembly.instantiate(compiled);
  console.log(`42 + 19 == ${mod.exports.add(42, 19)}`);
}

main();
