import { Cluster, BatchStatement, Query, Uuid } from "../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

const batch = new BatchStatement();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS batch_statements WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("batch_statements");
await session.execute("CREATE TABLE IF NOT EXISTS users (id UUID PRIMARY KEY, name TEXT)");

const simpleStatement = new Query("INSERT INTO users (id, name) VALUES (?, ?)");
const preparedStatement = await session.prepare("INSERT INTO users (id, name) VALUES (?, ?)");

batch.appendStatement(simpleStatement);
batch.appendStatement(preparedStatement);

await session.batch(batch, [
  [Uuid.randomV4(), "Alice"],
  [Uuid.randomV4(), "Bob"],
]);

console.log(await session.execute("SELECT * FROM users"));
