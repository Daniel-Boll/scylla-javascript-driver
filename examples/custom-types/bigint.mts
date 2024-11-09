import { Cluster } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS bigints WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("bigints");

await session.execute("CREATE TABLE IF NOT EXISTS bigints (a bigint, primary key (a))");

await session.execute("INSERT INTO bigints (a) VALUES (?)", [1238773128n]);

const results = await session.execute("SELECT a FROM bigints");
console.log(results);
