import { Cluster } from "../../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
  "CREATE KEYSPACE IF NOT EXISTS udt WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("udt");

await session.execute("CREATE TYPE IF NOT EXISTS address (street text, neighbor text)");
await session.execute("CREATE TABLE IF NOT EXISTS user (name text, address address, primary key (name))");

interface User {
  name: string;
  address: {
    street: string;
    neighbor: string;
  };
}

const user: User = {
  name: "John Doe",
  address: {
    street: "123 Main St",
    neighbor: "Downtown",
  },
};

await session.execute("INSERT INTO user (name, address) VALUES (?, ?)", [user.name, user.address]);

const users = (await session.execute("SELECT * FROM user")) as User[];
console.log(users);
