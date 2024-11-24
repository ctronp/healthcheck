# Healthcheck Service

A lightweight and customizable healthcheck service built with Rust and Axum. It supports multiple ports and paths
configured through environment variables, making it ideal for scenarios requiring dynamic health checks, such as AWS
Network Load Balancer (NLB) integrations.

## Features

- Responds with `HTTP 200 OK` to any configured path.
- Supports multiple ports and paths defined via environment variables.
- Graceful shutdown on receiving a termination signal (e.g., `Ctrl+C`).
- Logs incoming requests to the console.

## Configuration

The service uses environment variables to configure its behavior:

### Environment Variables

— `PORT` or `PORTS`: Comma-separated list of ports to listen on. Example: `PORTS=8080,8081`.
— `HEALTHCHECK` or `HEALTHCHECK_PATH`: Comma-separated list of healthcheck paths. Example:
`HEALTHCHECK_PATH=/health,/status`.

### Example

```dotenv

PORT=8080,8081
HEALTHCHECK=/health,/status

```

## Build and Run

### Prerequisites

- Rust toolchain
- Docker (optional for containerized deployment)

### Build Locally

1. Clone the repository:

```bash

git clone <repository_url>
cd <repository_name>

```

2. Build the binary:

```bash

cargo build --release

```

3. Run the binary:

```bash

PORT=8080 HEALTHCHECK=/health ./target/release/healthcheck

```

### Run with Docker

1. Build the Docker image:

```bash

docker build -t healthcheck-service .

```

2. Run the container:

```bash

docker run -e PORT=8080,8081 -e HEALTHCHECK=/health -p 8080:8080 -p 8081:8081 healthcheck-service

```

## Project Structure

- **`src/main.rs`**: Main application logic, including router setup and graceful shutdown.
- **`Dockerfile`**: Multi-stage build for an optimized container image.
- **`Cargo.toml`**: Dependency and configuration file for Rust.

## License

This project is licensed under the [MIT License](LICENSE).