<div align="center">
  <a href="https://github.com/daniel-boll/scylla-javascript-driver">
    <img src="assets/logo.png" alt="Scylla Nodejs Driver" width="640" />
  </a>

  <h4>🚀 ScyllaDB NodeJS Driver 🧪🔧</h4>
</div>

## ⚠️ Disclaimer ⚠️

This repository and the associated npm package are currently in a 🐣 pre-release state and are being used for testing 🧪 purposes. They are subject to change without notice 📝. Users are encouraged to use this driver with caution ❗ and not in production environments until the official release.

## 🚀 Getting Started 🚀

These instructions will get you a copy of the project up and running 🏃 on your local machine for development and testing purposes.

### 📋 Prerequisites 📋

- Docker: We use Docker 🐳 to run the Scylla database easily without the need for a complex local setup.
- Node.js: Make sure you have Node.js installed on your system to run JavaScript code.

### 🌟 Quickstart 🌟

1. **Start ScyllaDB in Docker:**

   Run a ScyllaDB instance using the following Docker command:

   ```bash
   docker run --name scylladb -d --rm -it -p 9042:9042 scylladb/scylla --smp 2
   ```

   This command pulls the Scylla image if it's not already present on your system, and starts a new 🌟 container with the Scylla database.

2. **Use the JavaScript Driver:**

   Here's a simple script to connect to the database and execute a query:

   ```javascript
   import { Cluster } from "@lambda-group/scylladb";

   const cluster = new Cluster({
     nodes: ["127.0.0.1:9042"],
   });

   const session = await cluster.connect("system_schema");

   const result = await session
     .execute("SELECT * FROM scylla_tables limit ?", [1])
     .catch(console.error);

   console.log(result);
   ```

   This script connects to the ScyllaDB instance running on your machine, performs a query, and logs the result.

### 📥 Installing 📥

To install this package, use the following command:

```bash
npm install @lambda-group/scylladb@latest
```

## 📚 Examples 📚

Reference wise you can guide yourself through the [examples/](https://github.com/Daniel-Boll/scylla-javascript-driver/tree/main/examples) folder in the repo.

## 🙏 Acknowledgments 🙏

- Thanks to the developers of ScyllaDB for creating such a high-performance database.
- Thanks to the Rust community for providing the robust `scylla` crate.
- Thanks to the `napi-rs` project for enabling efficient Rust and Node.js integrations.
