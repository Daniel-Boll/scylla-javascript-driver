import { Cluster } from "../index.js";

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

await session.execute("INSERT INTO basic (a, b, c) VALUES (1, 2, 'abc')");
