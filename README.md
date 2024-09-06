## Cypher-Indexer

Cypher-Indexer is a high-performance, scalable, and extensible Solana blockchain indexer. It provides real-time indexing of Solana blockchain data without relying on IDLs, making it suitable for indexing any Solana program, including those without published IDLs. This indexer is designed to work with the Cypher protocol and integrates IPFS for decentralized storage.

## Features

- High-performance ingestion using Geyser plugins, RPC polling, and WebSocket subscriptions
- Advanced binary parsing for accounts and instructions without IDLs
- Scalable storage solution using ClickHouse, ScyllaDB, and Redis
- IPFS integration for decentralized data storage
- WebAssembly (WASM) support for custom indexing logic
- Real-time processing pipeline with support for custom parsers
- GraphQL and REST API for flexible data querying
- Prometheus metrics for monitoring and alerting
- Dockerized deployment for easy scaling and management

## Prerequisites

- Rust 1.55.0 or later
- Docker and Docker Compose
- ClickHouse
- ScyllaDB
- Redis
- IPFS
- Wasmer (for WASM support)

## Quick Start

1. Clone the repository:
   ```
   git clone https://github.com/your-org/cypher-indexer.git
   cd cypher-indexer
   ```

2. Copy the example configuration file and edit it to match your environment:
   ```
   cp config.example.json config.json
   nano config.json
   ```

3. Build the project and WASM modules:
   ```
   ./build-wasm.sh
   cargo build --release
   ```

4. Run the indexer:
   ```
   ./run.sh
   ```

5. Access the API:
   - GraphQL Playground: http://localhost:8080/graphql
   - REST API documentation: http://localhost:8080/api-docs

## Configuration

The `config.json` file contains all the necessary configuration for the Cypher-Indexer. Here's an example configuration:

```json
{
  "solana_rpc_url": "https://api.mainnet-beta.solana.com",
  "clickhouse_url": "http://localhost:8123",
  "scylla_nodes": ["127.0.0.1:9042"],
  "redis_url": "redis://127.0.0.1:6379",
  "ipfs_api_url": "http://localhost:5001",
  "wasm_module_path": "./wasm/cypher_indexer.wasm",
  "rpc_poll_interval": 1,
  "websocket_url": "wss://api.mainnet-beta.solana.com",
  "geyser_plugin_config": {
    "libpath": "/path/to/libsolana_geyser_plugin.so",
    "accounts_selector": {
      "owners": ["CyphrkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"]
    }
  }
}
```

## Project Structure

The Cypher-Indexer project is structured as follows:

- `src/`: Contains the main Rust source code
  - `api/`: GraphQL and REST API implementations
  - `ingestion/`: Data ingestion sources (Geyser plugin, RPC poller, WebSocket listener)
  - `processing/`: Data processing logic (account, instruction, and event parsers)
  - `storage/`: Storage implementations (ClickHouse, ScyllaDB, Redis, IPFS)
  - `wasm/`: WebAssembly runtime and utilities
- `wasm/`: WebAssembly modules for custom indexing logic
- `tests/`: Integration and unit tests
- `benches/`: Benchmarks for performance testing

## Development

### Running Tests

To run the test suite:

```
cargo test
```

For integration tests:

```
cargo test --test integration_tests
```

### Running Benchmarks

To run the benchmarks:

```
cargo bench
```

### Linting

We use Clippy for linting. To run the linter:

```
cargo clippy -- -D warnings
```

## Deployment

1. Build the Docker image:
   ```
   docker build -t cypher-indexer .
   ```

2. Run the container:
   ```
   docker run -d --name cypher-indexer -p 8080:8080 -v /path/to/config.json:/app/config.json cypher-indexer
   ```

For production deployment, we recommend using Kubernetes or your preferred container orchestration system.

## API Usage

### GraphQL

You can access the GraphQL playground at `http://localhost:8080/graphql`. Here's an example query:

```graphql
query {
  getAccount(pubkey: "CyphrkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA") {
    pubkey
    lamports
    owner
    data
  }
}
```

### REST API

The REST API is available at `http://localhost:8080/api`. Here's an example request:

```
GET /api/account?pubkey=CyphrkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

## Monitoring

Prometheus metrics are exposed at `http://localhost:8080/metrics`. You can use these metrics with Grafana for visualization and alerting.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## License

Cypher-Indexer is released under the [MIT License](LICENSE).

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.

## Acknowledgements

We would like to thank the Solana community and all the open-source projects that made this indexer possible.