import { Cluster } from "../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

const clusterData = await session.getClusterData();
const keyspaceInfo = clusterData.getKeyspaceInfo();

if (!keyspaceInfo) throw new Error("No data found");

console.log("ALL KEYSPACES");
for (const keyspaceName in keyspaceInfo) {
  console.log("========================================================");
  const keyspaceData = keyspaceInfo[keyspaceName];
  console.log("Keyspace: ", keyspaceName);
  console.log("replication strategy: ", keyspaceData.strategy.kind, keyspaceData.strategy.data);
  for (const tableName in keyspaceData.tables) {
    console.log("-----------------------");
    const tableData = keyspaceData.tables[tableName];
    console.log("Table: ", tableName);
    console.log("partitionKey: ", tableData.partitionKey);
    console.log("clusteringKey: ", tableData.clusteringKey);
    console.log("columns: ", tableData.columns);
    console.log("-----------------------");
  }
  console.log("========================================================");
}

console.log("================== SPECIFIC KEYSPACES ==================");
console.log("keyspace: system_auth | strategy: ", keyspaceInfo.system_auth.strategy);
console.log("keyspace: system_traces | strategy: ", keyspaceInfo.system_traces.strategy);
console.log(
  "keyspace: system_distributed_everywhere | strategy: ",
  keyspaceInfo.system_distributed_everywhere.strategy,
);
console.log("keyspace: system_distributed | strategy: ", keyspaceInfo.system_distributed.strategy);
