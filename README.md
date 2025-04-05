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

- `PORT` or `PORTS`: Comma-separated list of ports to listen on. Example: `PORTS=8080,8081`.
- `HEALTHCHECK` or `HEALTHCHECK_PATH`: Comma-separated list of healthcheck paths. Example:
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

#### Using Docker Compose (Recommended)

The quickest way to test the service is using the included `docker-compose.yml` file:

```bash
docker-compose up
```

This will start the service on ports 8001 and 8002 with paths `/health` and `/check-health`.

#### Pre-built Docker Image

Use the pre-built Docker image available for both amd64 and arm64 architectures:

```bash
docker run -e PORT=8080,8081 -e HEALTHCHECK=/health -p 8080:8080 -p 8081:8081 ctr0np/healthcheck:latest
```

#### Build Your Own Image

If you prefer to build your own image:

```bash
docker build -t healthcheck-service .
docker run -e PORT=8080 -e HEALTHCHECK=/health -p 8080:8080 healthcheck-service
```

## Docker Compose Example

The project includes a `docker-compose.yml` file for easy testing:

```yaml
services:
  healthcheck:
    build: .
    ports:
      - "8001:8001"
      - "8002:8002"
    environment:
      - PORT=8001,8002
      - HEALTHCHECK=/health,/check-health
    restart: unless-stopped
```

You can also use the pre-built image in your own docker-compose file:

```yaml
services:
  healthcheck:
    image: ctr0np/healthcheck:latest
    ports:
      - "8001:8001"
      - "8002:8002"
    environment:
      - PORT=8001,8002
      - HEALTHCHECK=/health,/check-health
    restart: unless-stopped
```

## Project Structure

- **`src/main.rs`**: Main application logic, including router setup and graceful shutdown.
- **`Dockerfile`**: Multi-stage build for an optimized container image.
- **`docker-compose.yml`**: Example configuration for easy deployment using Docker Compose.
- **`Cargo.toml`**: Dependency and configuration file for Rust.

## License

This project is licensed under the [MIT License](LICENSE).