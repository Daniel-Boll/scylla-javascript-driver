import { Cluster, Varint } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS varints WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("varints");

await session.execute(
  "CREATE TABLE IF NOT EXISTS varints (a varint, primary key (a))",
);

await session.execute("INSERT INTO varints (a) VALUES (?)", [
  new Varint([0x00, 0x01, 0x02]),
]);

const results = await session.execute("SELECT a FROM varints");
console.log(results);
