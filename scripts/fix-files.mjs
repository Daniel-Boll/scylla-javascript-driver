import { readFileSync, writeFileSync } from "node:fs";

function addGenericTypes(filename) {
  const content = readFileSync(filename, "utf8");
  const updatedContent = content
    .replace(
      /export class List\b(.*){/,
      "export class List<T = NativeTypes>$1{",
    )
    .replace(
      /export class Map\b(.*){/,
      "export class Map<T = NativeTypes, U = NativeTypes>$1{",
    )
    .replace(/export class Set\b(.*){/, "export class Set<T = NativeTypes>$1{");

  writeFileSync(filename, updatedContent);
}

// Append to filename inspectors for custom types
function addInspector(filename) {
  writeFileSync(
    filename,
    readFileSync(filename, "utf8")
      .concat(
        `
const customInspectSymbol = Symbol.for('nodejs.util.inspect.custom')

Uuid.prototype[customInspectSymbol]     = function () { return this.toString(); }
Duration.prototype[customInspectSymbol] = function () { return this.toString(); }
Decimal.prototype[customInspectSymbol]  = function () { return this.toString(); }
`,
      )
      .trim(),
  );
}

function addJSQueryResultType(filename) {
  writeFileSync(
    filename,
    readFileSync(filename, "utf8")
      .concat(
        `
type NativeTypes = number | string | Uuid | bigint | Duration | Decimal | Float | List;
type WithMapType = NativeTypes | Record<string, NativeTypes> | NativeTypes[];
type ParameterWithMapType = WithMapType;
type JSQueryResultType = Record<string, WithMapType>[];
        `,
      )
      .trim(),
  );
}

const filename = process.argv[process.argv.length - 1];
if (filename.endsWith("index.js")) addInspector(filename);
else if (filename.endsWith("index.d.ts")) {
  addGenericTypes(filename);
  addJSQueryResultType(filename);
}
