import { readFileSync, writeFileSync } from "node:fs";

// Append to filename inspectors for custom types
function addInspector(filename) {
  writeFileSync(filename, readFileSync(filename, "utf8").concat(`
const customInspectSymbol = Symbol.for('nodejs.util.inspect.custom')

Uuid.prototype[customInspectSymbol] = function () {
  return this.toString();
}
`).trim());
}

let filename = process.argv[process.argv.length - 1]
if (filename.endsWith('index.js')) {
  addInspector(filename)
} else if (filename.endsWith('index.d.ts')) { }
