import { Cluster } from "../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS basic WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("basic");

await session.execute("CREATE TABLE IF NOT EXISTS basic (a int, b int, c text, primary key (a, b))");

await session.execute("INSERT INTO basic (a, b, c) VALUES (1, 2, 'abc')");
await session.execute("INSERT INTO basic (a, b, c) VALUES (?, ?, ?)", [3, 4, "def"]);

const prepared = await session.prepare("INSERT INTO basic (a, b, c) VALUES (?, 7, ?)");
await session.execute(prepared, [42, "I'm prepared!"]);
await session.execute(prepared, [43, "I'm prepared 2!"]);
await session.execute(prepared, [44, "I'm prepared 3!"]);

interface RowData {
  a: number;
  b: number;
  c: string;
}
const result = await session.execute("SELECT a, b, c FROM basic");
console.log(result);

const metrics = session.metrics();
console.log(`Queries requested: ${metrics.getQueriesNum()}`);
console.log(`Iter queries requested: ${metrics.getQueriesIterNum()}`);
console.log(`Errors occurred: ${metrics.getErrorsNum()}`);
console.log(`Iter errors occurred: ${metrics.getErrorsIterNum()}`);
console.log(`Average latency: ${metrics.getLatencyAvgMs()}`);
console.log(`99.9 latency percentile: ${metrics.getLatencyPercentileMs(99.9)}`);
