import { Cluster, Uuid } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS uuids WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("uuids");

await session.execute(
  "CREATE TABLE IF NOT EXISTS uuids (a uuid, primary key (a))",
);

await session.execute("INSERT INTO uuids (a) VALUES (?)", [Uuid.randomV4()]);

const results = await session.execute("SELECT a FROM uuids");
console.log(results);
