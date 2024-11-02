import { Cluster, Set } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS sets WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("sets");

await session.execute(
  "CREATE TABLE IF NOT EXISTS sets (a set<int>, primary key (a))",
);

await session.execute("INSERT INTO sets (a) VALUES (?)", [
  new Set<number>([1, 2, 3]),
]);

const results = await session.execute("SELECT a FROM sets");
console.log(results);
