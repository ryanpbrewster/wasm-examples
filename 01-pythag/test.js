const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("./pkg/pythag_bg.wasm");
  const compiled = await WebAssembly.compile(bytes);
  const mod = await WebAssembly.instantiate(compiled);
  console.log(`isPythag(1, 2, 3) == ${mod.exports.is_pythag(1, 2, 3)}`);
  console.log(`isPythag(3, 4, 5) == ${mod.exports.is_pythag(3, 4, 5)}`);
}

main();
