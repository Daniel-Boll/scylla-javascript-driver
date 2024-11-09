import { Cluster, Double } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS double WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("double");

await session.execute(
  "CREATE TABLE IF NOT EXISTS double (a double, primary key (a))",
);

const input = new Double(1.1127830921);
const _ = await session.execute("INSERT INTO double (a) VALUES (?)", [input]);
console.log(_);

const results = await session.execute("SELECT a FROM double");
console.log(`${input} -> ${results[0].a}`);
