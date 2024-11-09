import { Cluster, Set, Uuid } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS sets WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("sets");

await session.execute(
  "CREATE TABLE IF NOT EXISTS sets (a uuid, b set<int>, primary key (a))",
);

await session.execute("INSERT INTO sets (a, b) VALUES (?, ?)", [
  Uuid.randomV4(),
  new Set<number>([1, 2, 3, 1]),
]);

const results = await session.execute("SELECT * FROM sets");
console.log(results);
