import {Cluster, VerifyMode} from "../index.js"

const nodes = process.env.CLUSTER_NODES?.split(",") ?? ["localhost:9142"];
console.log(`Connecting to ${nodes}`);

const cluster = new Cluster({
    nodes,
    ssl: {
        enabled: true,
        truststoreFilepath: "/your/path/to/certificates/client_cert.pem",
        privateKeyFilepath: "/your/path/to/certificates/client_key.pem",
        caFilepath: "/your/path/to/certificates/client_truststore.pem",
        verifyMode: VerifyMode.Peer,
    }
});

const session = await cluster.connect();

interface ConnectedClient {
    address: String,
    port: number,
    username: String,
    driver_name: String,
    driver_version: String,
}

// @ts-ignore
let result = await session.execute<ConnectedClient>("SELECT address, port, username, driver_name, driver_version FROM system.clients");

console.log(result)
// [
//  {
//     address: '127.0.0.1',
//     driver_name: 'scylla-rust-driver',
//     driver_version: '0.10.1',
//     port: 58846,
//     username: 'developer'
//  }
// ]


