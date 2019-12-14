const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("./pkg/reverse_bg.wasm");
  const compiled = await WebAssembly.compile(bytes);
  const mod = await WebAssembly.instantiate(compiled);

  const xs = [3, 1, 4, 1, 5, 9, 2, 6];
  console.log(`reverse(${xs}) == ${mod.exports.reverse(xs)}`);
}

main();











/*
const mod = require("./pkg/reverse.js");

const xs = [3, 1, 4, 1, 5, 9, 2, 6];
console.log(`reverse(${xs}) == ${mod.reverse(xs)}`);
*/
