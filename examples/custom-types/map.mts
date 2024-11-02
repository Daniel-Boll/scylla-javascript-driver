import { Cluster, Map, Uuid } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS maps WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("maps");

await session.execute(
  "CREATE TABLE IF NOT EXISTS maps (a uuid, b map<text, int>, primary key (a))",
);

await session.execute("INSERT INTO maps (a, b) VALUES (?, ?)", [
  Uuid.randomV4(),
  new Map<string, number>([
    ["a", 1],
    ["b", 2],
    ["c", 3],
  ]),
]);

const results = await session.execute("SELECT * FROM maps");
console.log(results);
