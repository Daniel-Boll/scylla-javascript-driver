import { Cluster, Uuid } from ".";

const cluster = new Cluster({
  nodes: ["127.0.0.1:9042"],
});

const session = await cluster.connect("system_schema");

const result = await session
  .execute("SELECT * FROM scylla_tables limit ?", [2147483648])
  .catch((err) => console.error(err));

console.log(result);
