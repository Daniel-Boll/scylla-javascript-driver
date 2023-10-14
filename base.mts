import { Cluster } from "./index";

const cluster = new Cluster({
  nodes: ["127.0.0.1:9042"],
});

const session = await cluster.connect("system_schema");

const result = await session
  .execute("SELECT * FROM scylla_tables limit ?", [1])
  .catch((err) => console.error(err));

console.log(result);
