import { Cluster, Uuid } from "../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS refactor WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("refactor");

await session.execute(
  "CREATE TABLE IF NOT EXISTS refactor (a text, b int, c uuid, d bigint, primary key (a))",
);

await session.execute("INSERT INTO refactor (a, b, c, d) VALUES (?, ?, ?, ?)", [
  "is a string",
  42,
  Uuid.randomV4(),
  42192219n,
]);

const results = await session.execute("SELECT * FROM refactor");
console.log(results);
