import { Cluster } from "../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

const { tracing, result } = await session.executeWithTracing(
  "SELECT * FROM system_schema.scylla_tables",
  [],
  // {
  //   prepare: true,
  // },
);

console.log(result, tracing);
