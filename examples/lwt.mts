import { Cluster, Consistency, Query, SerialConsistency } from "../index.js"

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

const cluster = new Cluster({ nodes });

const session = await cluster.connect();

await session.execute("CREATE KEYSPACE IF NOT EXISTS examples_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}");
await session.execute("CREATE TABLE IF NOT EXISTS examples_ks.tab (a int PRIMARY KEY)");

const query = new Query("INSERT INTO examples_ks.tab (a) VALUES(?) IF NOT EXISTS");
query.setConsistency(Consistency.One);
query.setSerialConsistency(SerialConsistency.Serial);

await session.execute(query, [12345]);

console.log("Ok.");
