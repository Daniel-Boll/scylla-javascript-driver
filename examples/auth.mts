import { Cluster } from "../index.js"

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

const cluster = new Cluster({
  nodes,
  auth: {
    username: "cassandra",
    password: "cassandra"
  }
});

const session = await cluster.connect();

await session.execute("CREATE KEYSPACE IF NOT EXISTS examples_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}");
await session.execute("DROP TABLE IF EXISTS examples_ks.auth");

console.log("Ok.");
