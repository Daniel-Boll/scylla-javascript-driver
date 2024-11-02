import { Cluster, List, Uuid } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS lists WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("lists");

await session.execute(
  "CREATE TABLE IF NOT EXISTS lists (a uuid, b list<int>, primary key (a))",
);

// NOTE: driver is not throwing errors if the return of the function is not used.
await session.execute("INSERT INTO lists (a, b) VALUES (?, ?)", [
  Uuid.randomV4(),
  new List<number>([1, 2, 3]), // TODO: add the generic to the type annotation on index.d.ts
]);

const results = await session.execute("SELECT * FROM lists");
console.log(results);
