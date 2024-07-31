import { Cluster } from "../index.js";

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["127.0.0.1:9042"];

console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({ nodes });
const session = await cluster.connect();

await session.execute(
	"CREATE KEYSPACE IF NOT EXISTS basic WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 }",
);
await session.useKeyspace("basic");

await session.execute(
	"CREATE TABLE IF NOT EXISTS basic (a int, b int, c text, primary key (a, b))",
);

await session.execute("INSERT INTO basic (a, b, c) VALUES (1, 2, 'abc')");
await session.execute("INSERT INTO basic (a, b, c) VALUES (?, ?, ?)", [
	3,
	4,
	"def",
]);

const prepared = await session.prepare(
	"INSERT INTO basic (a, b, c) VALUES (?, 7, ?)",
);
await session.execute(prepared, [42, "I'm prepared!"]);
await session.execute(prepared, [43, "I'm prepared 2!"]);
await session.execute(prepared, [44, "I'm prepared 3!"]);

const clusterData = await session.getClusterData();
const keyspaceInfo = clusterData.getKeyspaceInfo();

if (keyspaceInfo) {
	// biome-ignore lint/complexity/noForEach: <explanation>
	Object.entries(keyspaceInfo).forEach(([keyspaceName, keyspaceData]) => {
		console.debug("========================================================");
		console.log(`Keyspace: ${keyspaceName}`);
		console.debug(
			`replication strategy: ${keyspaceData.strategy.kind}:`,
			keyspaceData.strategy.data,
		);
		// biome-ignore lint/complexity/noForEach: <explanation>
		Object.entries(keyspaceData.tables).forEach(([tableName, tableData]) => {
			console.debug("-----------------------");
			console.log(`Table: ${tableName}`);
			console.debug(`partitionKey: ${tableData.partitionKey}`);
			console.debug(`clusteringKey: ${tableData.clusteringKey}`);
			console.debug("columns: ");
			// biome-ignore lint/complexity/noForEach: <explanation>
			tableData?.columns?.forEach((column) => {
				console.log(`   Column: ${column}`);
			});
			console.debug("-----------------------");
		});
		console.debug("========================================================");
	});
}

