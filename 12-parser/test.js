const mod = require("./pkg/parser.js");
const input = "3 4     15         12";
console.log(`parse(${input}) == ${mod.parse(input)}`);
