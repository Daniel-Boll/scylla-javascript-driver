import { Cluster, Float } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS floats WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("floats");

await session.execute(
  "CREATE TABLE IF NOT EXISTS floats (a float, primary key (a))",
);

const input = new Float(1.1127830921);
await session.execute("INSERT INTO floats (a) VALUES (?)", [input]);

const results = await session.execute("SELECT a FROM floats");
console.log(`${input} -> ${results[0].a}`);
