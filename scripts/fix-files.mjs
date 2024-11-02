import { readFileSync, writeFileSync } from "node:fs";

function addGenericTypes(filename) {
  const content = readFileSync(filename, "utf8");
  const updatedContent = content
    .replace(
      /export declare class List\b(.*){/,
      "export declare class List<T = NativeTypes>$1{",
    )
    .replace(
      /export declare class Map\b(.*){/,
      "export declare class Map<T = NativeTypes, U = NativeTypes>$1{",
    )
    .replace(
      /export declare class Set\b(.*){/,
      "export declare class Set<T = NativeTypes>$1{",
    );

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

nativeBinding.Uuid.prototype[customInspectSymbol]     = function () { return this.toString(); }
nativeBinding.Duration.prototype[customInspectSymbol] = function () { return this.toString(); }
nativeBinding.Decimal.prototype[customInspectSymbol]  = function () { return this.toString(); }
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
type JSQueryResult = Record<string, WithMapType>[];
type TracingReturn = { result: JSQueryResult; tracing: TracingInfo };

export interface TracingInfo {
  client?: string; // IP address as a string
  command?: string;
  coordinator?: string; // IP address as a string
  duration?: number;
  parameters?: Record<string, string>;
  request?: string;
  /**
   * started_at is a timestamp - time since unix epoch
   */
  started_at?: string;
  events: TracingEvent[];
}

/**
 * A single event happening during a traced query
 */
export interface TracingEvent {
  event_id: string;
  activity?: string;
  source?: string; // IP address as a string
  source_elapsed?: number;
  thread?: string;
}
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
