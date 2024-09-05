# Cypher-Indexer

Cypher-Indexer is a high-performance, scalable, and extensible Solana blockchain indexer. It provides real-time indexing of Solana blockchain data without relying on IDLs, making it suitable for indexing any Solana program, including those without published IDLs.

## Features

- High-performance ingestion using Geyser plugins, RPC polling, and WebSocket subscriptions
- Advanced binary parsing for accounts and instructions without IDLs
- Scalable storage solution using ClickHouse, ScyllaDB, and Redis
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

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the indexer:
   ```
   cargo run --release
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
  "rpc_poll_interval": 1,
  "websocket_url": "wss://api.mainnet-beta.solana.com",
  "geyser_plugin_config": {
    "libpath": "/path/to/libsolana_geyser_plugin.so",
    "accounts_selector": {
      "owners": ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"]
    }
  }
}
```

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
  getAccount(pubkey: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA") {
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
GET /api/account?pubkey=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
```

## Monitoring

Prometheus metrics are exposed at `http://localhost:8080/metrics`. You can use these metrics with Grafana for visualization and alerting.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## License

Cypher-Indexer is released under the [MIT License](LICENSE).

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository or reach out to our support team at support@cypher-indexer.com.

## Acknowledgements

We would like to thank the Solana community and all the open-source projects that made this indexer possible.